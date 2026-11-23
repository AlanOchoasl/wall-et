use chrono::{DateTime, SecondsFormat, Utc};
use rusqlite::{Connection, OptionalExtension, Row, params};

use crate::application::service_repository::{ServiceId, ServiceRecord, ServiceRepository};
use crate::domain::error::DomainError;
use crate::domain::money::{Currency, Money};
use crate::domain::service::{Service, ServiceStatus};
use crate::infrastructure::database;

const SELECT_BY_ID: &str = "SELECT id, name, total_cost_minor, currency_code, shared_people_count, notes, status, created_at, updated_at FROM service WHERE id = ?1";
const LIST_BY_STATUS: &str = "SELECT id, name, total_cost_minor, currency_code, shared_people_count, notes, status, created_at, updated_at FROM service WHERE status = ?1 ORDER BY created_at ASC, id ASC";

#[derive(Debug, Default)]
pub struct SqliteServiceRepository;

impl ServiceRepository for SqliteServiceRepository {
    fn insert(
        &self,
        connection: &Connection,
        service: &Service,
    ) -> Result<ServiceRecord, DomainError> {
        let id = ServiceId::new();
        let now = now_truncated_to_millis();

        connection
            .execute(
                "INSERT INTO service (id, name, total_cost_minor, currency_code, shared_people_count, notes, status, created_at, updated_at) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)",
                params![
                    id.to_string(),
                    service.name(),
                    service.total_cost().minor_units(),
                    service.currency().code(),
                    service.shared_people_count().get(),
                    service.notes(),
                    service.status().as_str(),
                    format_timestamp(now),
                    format_timestamp(now),
                ],
            )
            .map_err(database::sqlite_error)?;

        Ok(ServiceRecord {
            id,
            service: service.clone(),
            created_at: now,
            updated_at: now,
        })
    }

    fn get(
        &self,
        connection: &Connection,
        id: ServiceId,
    ) -> Result<Option<ServiceRecord>, DomainError> {
        let row = connection
            .query_row(
                SELECT_BY_ID,
                params![id.to_string()],
                RawServiceRow::from_row,
            )
            .optional()
            .map_err(database::sqlite_error)?;

        row.map(RawServiceRow::into_record).transpose()
    }

    fn update(
        &self,
        connection: &Connection,
        id: ServiceId,
        service: &Service,
    ) -> Result<ServiceRecord, DomainError> {
        let transaction = connection
            .unchecked_transaction()
            .map_err(database::sqlite_error)?;

        let rows = transaction
            .execute(
                "UPDATE service SET name = ?1, total_cost_minor = ?2, currency_code = ?3, shared_people_count = ?4, notes = ?5, updated_at = ?6 WHERE id = ?7",
                params![
                    service.name(),
                    service.total_cost().minor_units(),
                    service.currency().code(),
                    service.shared_people_count().get(),
                    service.notes(),
                    format_timestamp(now_truncated_to_millis()),
                    id.to_string(),
                ],
            )
            .map_err(database::sqlite_error)?;

        if rows == 0 {
            return Err(DomainError::NotFound);
        }

        let row = transaction
            .query_row(
                SELECT_BY_ID,
                params![id.to_string()],
                RawServiceRow::from_row,
            )
            .map_err(database::sqlite_error)?;
        let record = row.into_record()?;

        transaction.commit().map_err(database::sqlite_error)?;

        Ok(record)
    }

    fn delete(&self, connection: &Connection, id: ServiceId) -> Result<(), DomainError> {
        let rows = connection
            .execute("DELETE FROM service WHERE id = ?1", params![id.to_string()])
            .map_err(database::sqlite_error)?;

        if rows == 0 {
            return Err(DomainError::NotFound);
        }

        Ok(())
    }

    fn set_status(
        &self,
        connection: &Connection,
        id: ServiceId,
        status: ServiceStatus,
    ) -> Result<ServiceRecord, DomainError> {
        let transaction = connection
            .unchecked_transaction()
            .map_err(database::sqlite_error)?;

        let rows = transaction
            .execute(
                "UPDATE service SET status = ?1, updated_at = ?2 WHERE id = ?3",
                params![
                    status.as_str(),
                    format_timestamp(now_truncated_to_millis()),
                    id.to_string(),
                ],
            )
            .map_err(database::sqlite_error)?;

        if rows == 0 {
            return Err(DomainError::NotFound);
        }

        let row = transaction
            .query_row(
                SELECT_BY_ID,
                params![id.to_string()],
                RawServiceRow::from_row,
            )
            .map_err(database::sqlite_error)?;
        let record = row.into_record()?;

        transaction.commit().map_err(database::sqlite_error)?;

        Ok(record)
    }

    fn list_active(&self, connection: &Connection) -> Result<Vec<ServiceRecord>, DomainError> {
        list_by_status(connection, ServiceStatus::Active)
    }

    fn list_frozen(&self, connection: &Connection) -> Result<Vec<ServiceRecord>, DomainError> {
        list_by_status(connection, ServiceStatus::Frozen)
    }
}

fn list_by_status(
    connection: &Connection,
    status: ServiceStatus,
) -> Result<Vec<ServiceRecord>, DomainError> {
    let mut statement = connection
        .prepare(LIST_BY_STATUS)
        .map_err(database::sqlite_error)?;
    let rows = statement
        .query_map(params![status.as_str()], RawServiceRow::from_row)
        .map_err(database::sqlite_error)?
        .collect::<rusqlite::Result<Vec<_>>>()
        .map_err(database::sqlite_error)?;

    rows.into_iter().map(RawServiceRow::into_record).collect()
}

#[derive(Debug)]
struct RawServiceRow {
    id: String,
    name: String,
    total_cost_minor: i64,
    currency_code: String,
    shared_people_count: i64,
    notes: Option<String>,
    status: String,
    created_at: String,
    updated_at: String,
}

impl RawServiceRow {
    fn from_row(row: &Row<'_>) -> rusqlite::Result<Self> {
        Ok(Self {
            id: row.get(0)?,
            name: row.get(1)?,
            total_cost_minor: row.get(2)?,
            currency_code: row.get(3)?,
            shared_people_count: row.get(4)?,
            notes: row.get(5)?,
            status: row.get(6)?,
            created_at: row.get(7)?,
            updated_at: row.get(8)?,
        })
    }

    fn into_record(self) -> Result<ServiceRecord, DomainError> {
        let id = ServiceId::parse(self.id.as_str())
            .ok_or_else(|| persistence_context("id", self.id.as_str(), Some(self.id.as_str())))?;
        let id_text = id.to_string();
        if id_text.as_str() != self.id.as_str() {
            return Err(persistence_context(
                "id",
                self.id.as_str(),
                Some(self.id.as_str()),
            ));
        }

        let currency = Currency::from_code(self.currency_code.as_str()).ok_or_else(|| {
            persistence_context(
                "currency_code",
                self.currency_code.as_str(),
                Some(id_text.as_str()),
            )
        })?;

        let shared_people_count = u32::try_from(self.shared_people_count).map_err(|_| {
            persistence_context(
                "shared_people_count",
                self.shared_people_count.to_string(),
                Some(id_text.as_str()),
            )
        })?;

        let status = ServiceStatus::from_code(self.status.as_str()).ok_or_else(|| {
            persistence_context("status", self.status.as_str(), Some(id_text.as_str()))
        })?;

        let created_at = parse_timestamp(
            self.created_at.as_str(),
            "created_at",
            Some(id_text.as_str()),
        )?;
        let updated_at = parse_timestamp(
            self.updated_at.as_str(),
            "updated_at",
            Some(id_text.as_str()),
        )?;

        let total_cost = Money::from_minor_units(self.total_cost_minor, currency);
        let service = Service::from_parts(
            self.name,
            total_cost,
            shared_people_count,
            self.notes,
            status,
        )
        .map_err(|error| {
            let column = match &error {
                DomainError::Validation { field, .. } => domain_column(field),
                _ => "service",
            };
            persistence_context(column, error.to_string(), Some(id_text.as_str()))
        })?;

        Ok(ServiceRecord {
            id,
            service,
            created_at,
            updated_at,
        })
    }
}

fn parse_timestamp(
    value: &str,
    column: &str,
    id: Option<&str>,
) -> Result<DateTime<Utc>, DomainError> {
    DateTime::parse_from_rfc3339(value)
        .map(|timestamp| timestamp.with_timezone(&Utc))
        .map_err(|error| persistence_context(column, format!("{value} ({error})"), id))
}

fn now_truncated_to_millis() -> DateTime<Utc> {
    DateTime::from_timestamp_millis(Utc::now().timestamp_millis())
        .expect("el timestamp actual debe ser válido")
}

fn format_timestamp(value: DateTime<Utc>) -> String {
    value.to_rfc3339_opts(SecondsFormat::Millis, true)
}

fn domain_column(field: &str) -> &'static str {
    match field {
        "name" => "name",
        "totalCost" => "total_cost_minor",
        "sharedPeopleCount" => "shared_people_count",
        _ => "service",
    }
}

fn persistence_context(column: &str, detail: impl AsRef<str>, id: Option<&str>) -> DomainError {
    let detail = detail.as_ref();
    match id {
        Some(id) => DomainError::persistence(format!(
            "registro corrupto en service.id={id}: columna {column} inválida ({detail})"
        )),
        None => DomainError::persistence(format!(
            "registro corrupto: columna {column} inválida ({detail})"
        )),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const REPOSITORY: SqliteServiceRepository = SqliteServiceRepository;
    const VALID_TIMESTAMP: &str = "2026-01-01T00:00:00.000Z";

    fn ars(name: &str, cost: &str, people: u32) -> Service {
        Service::new(name, cost, people, None).unwrap()
    }

    fn usd(name: &str, cost: &str, people: u32) -> Service {
        Service::new_with_currency(name, cost, Currency::Usd, people, None).unwrap()
    }

    fn unknown_id() -> ServiceId {
        ServiceId::parse("00000000-0000-4000-8000-000000000000").unwrap()
    }

    fn assert_timestamp_in_range(
        value: DateTime<Utc>,
        before: DateTime<Utc>,
        after: DateTime<Utc>,
    ) {
        assert!(value >= before, "{value} es menor que {before}");
        assert!(value <= after, "{value} es mayor que {after}");
    }

    fn assert_persistence(error: &DomainError, fragment: &str) {
        match error {
            DomainError::Persistence(message) => {
                assert!(message.contains(fragment), "mensaje inesperado: {message}")
            }
            other => panic!("se esperaba Persistence, se obtuvo {other:?}"),
        }
    }

    #[allow(clippy::too_many_arguments)]
    fn insert_raw_full(
        connection: &Connection,
        id: &str,
        name: &str,
        total_cost_minor: i64,
        currency_code: &str,
        shared_people_count: i64,
        notes: Option<&str>,
        status: &str,
        created_at: &str,
        updated_at: &str,
    ) {
        connection
            .execute(
                "INSERT INTO service (id, name, total_cost_minor, currency_code, shared_people_count, notes, status, created_at, updated_at) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)",
                params![
                    id,
                    name,
                    total_cost_minor,
                    currency_code,
                    shared_people_count,
                    notes,
                    status,
                    created_at,
                    updated_at,
                ],
            )
            .unwrap();
    }

    #[test]
    fn inserta_y_consulta_en_ars() {
        let connection = database::open_in_memory().unwrap();
        let service = ars("Spotify", "10000,10", 4);

        let record = REPOSITORY.insert(&connection, &service).unwrap();
        let loaded = REPOSITORY
            .get(&connection, record.id)
            .unwrap()
            .expect("el servicio insertado debe existir");

        assert_eq!(loaded.id, record.id);
        assert_eq!(loaded.service.name(), "Spotify");
        assert_eq!(loaded.service.total_cost().minor_units(), 1_000_010);
        assert_eq!(loaded.service.currency(), Currency::Ars);
        assert_eq!(loaded.service.shared_people_count().get(), 4);
        assert_eq!(loaded.service.status(), ServiceStatus::Active);
    }

    #[test]
    fn inserta_y_consulta_en_usd() {
        let connection = database::open_in_memory().unwrap();
        let service = usd("Netflix", "15.99", 2);

        let record = REPOSITORY.insert(&connection, &service).unwrap();
        let loaded = REPOSITORY
            .get(&connection, record.id)
            .unwrap()
            .expect("el servicio insertado debe existir");

        assert_eq!(loaded.id, record.id);
        assert_eq!(loaded.service.name(), "Netflix");
        assert_eq!(loaded.service.total_cost().minor_units(), 1_599);
        assert_eq!(loaded.service.currency(), Currency::Usd);
        assert_eq!(loaded.service.shared_people_count().get(), 2);
    }

    #[test]
    fn insert_devuelve_timestamps_identicos_a_get() {
        let connection = database::open_in_memory().unwrap();
        let record = REPOSITORY
            .insert(&connection, &ars("Luz", "100,00", 1))
            .unwrap();

        let loaded = REPOSITORY
            .get(&connection, record.id)
            .unwrap()
            .expect("el servicio insertado debe existir");

        assert_eq!(loaded.created_at, record.created_at);
        assert_eq!(loaded.updated_at, record.updated_at);
        assert_eq!(loaded.created_at, loaded.updated_at);
    }

    #[test]
    fn get_de_id_inexistente_devuelve_none() {
        let connection = database::open_in_memory().unwrap();

        assert!(REPOSITORY.get(&connection, unknown_id()).unwrap().is_none());
    }

    #[test]
    fn update_de_id_inexistente_devuelve_not_found() {
        let connection = database::open_in_memory().unwrap();

        let error = REPOSITORY
            .update(&connection, unknown_id(), &ars("Luz", "100,00", 1))
            .unwrap_err();

        assert!(matches!(error, DomainError::NotFound));
        assert!(REPOSITORY.list_active(&connection).unwrap().is_empty());
    }

    #[test]
    fn delete_de_id_inexistente_devuelve_not_found() {
        let connection = database::open_in_memory().unwrap();

        let error = REPOSITORY.delete(&connection, unknown_id()).unwrap_err();

        assert!(matches!(error, DomainError::NotFound));
    }

    #[test]
    fn set_status_de_id_inexistente_devuelve_not_found() {
        let connection = database::open_in_memory().unwrap();

        let error = REPOSITORY
            .set_status(&connection, unknown_id(), ServiceStatus::Frozen)
            .unwrap_err();

        assert!(matches!(error, DomainError::NotFound));
    }

    #[test]
    fn distingue_notas_null_de_notas_vacias() {
        let connection = database::open_in_memory().unwrap();
        let without_notes = REPOSITORY
            .insert(&connection, &ars("Sin notas", "100,00", 1))
            .unwrap();
        let with_empty_notes = REPOSITORY
            .insert(
                &connection,
                &Service::new("Notas vacías", "100,00", 1, Some(String::new())).unwrap(),
            )
            .unwrap();

        let loaded_without = REPOSITORY
            .get(&connection, without_notes.id)
            .unwrap()
            .unwrap();
        let loaded_with_empty = REPOSITORY
            .get(&connection, with_empty_notes.id)
            .unwrap()
            .unwrap();

        assert_eq!(loaded_without.service.notes(), None);
        assert_eq!(loaded_with_empty.service.notes(), Some(""));
    }

    #[test]
    fn update_preserva_id_created_at_y_estado() {
        let connection = database::open_in_memory().unwrap();
        let record = REPOSITORY
            .insert(&connection, &ars("Spotify", "10000,00", 4))
            .unwrap();
        let replacement = ars("Spotify Premium", "12000,00", 3);
        let before = now_truncated_to_millis();

        let updated = REPOSITORY
            .update(&connection, record.id, &replacement)
            .unwrap();

        let after = now_truncated_to_millis();
        assert_eq!(updated.id, record.id);
        assert_eq!(updated.created_at, record.created_at);
        assert_eq!(updated.service.status(), ServiceStatus::Active);
        assert_eq!(updated.service.name(), "Spotify Premium");
        assert_eq!(updated.service.total_cost().minor_units(), 1_200_000);
        assert_eq!(updated.service.shared_people_count().get(), 3);
        assert_timestamp_in_range(updated.updated_at, before, after);
    }

    #[test]
    fn update_de_servicio_frozen_no_lo_reactiva() {
        let connection = database::open_in_memory().unwrap();
        let record = REPOSITORY
            .insert(&connection, &ars("Luz", "100,00", 1))
            .unwrap();
        REPOSITORY
            .set_status(&connection, record.id, ServiceStatus::Frozen)
            .unwrap();
        let replacement = Service::from_parts(
            "Luz actualizada",
            Money::from_minor_units(15_000, Currency::Ars),
            2,
            None,
            ServiceStatus::Frozen,
        )
        .unwrap();

        let updated = REPOSITORY
            .update(&connection, record.id, &replacement)
            .unwrap();

        assert_eq!(updated.service.status(), ServiceStatus::Frozen);
        assert_eq!(updated.service.name(), "Luz actualizada");
        assert_eq!(updated.created_at, record.created_at);
        let active_ids: Vec<String> = REPOSITORY
            .list_active(&connection)
            .unwrap()
            .iter()
            .map(|record| record.id.to_string())
            .collect();
        let frozen_ids: Vec<String> = REPOSITORY
            .list_frozen(&connection)
            .unwrap()
            .iter()
            .map(|record| record.id.to_string())
            .collect();
        assert!(!active_ids.contains(&record.id.to_string()));
        assert!(frozen_ids.contains(&record.id.to_string()));
    }

    #[test]
    fn elimina_correctamente() {
        let connection = database::open_in_memory().unwrap();
        let record = REPOSITORY
            .insert(&connection, &ars("Luz", "100,00", 1))
            .unwrap();

        REPOSITORY.delete(&connection, record.id).unwrap();

        assert!(REPOSITORY.get(&connection, record.id).unwrap().is_none());
        assert!(REPOSITORY.list_active(&connection).unwrap().is_empty());
    }

    #[test]
    fn cambio_de_estado_active_frozen_active() {
        let connection = database::open_in_memory().unwrap();
        let record = REPOSITORY
            .insert(&connection, &ars("Luz", "100,00", 1))
            .unwrap();

        let frozen = REPOSITORY
            .set_status(&connection, record.id, ServiceStatus::Frozen)
            .unwrap();
        assert_eq!(frozen.service.status(), ServiceStatus::Frozen);
        assert_eq!(frozen.created_at, record.created_at);
        assert_eq!(frozen.service.name(), record.service.name());

        let active = REPOSITORY
            .set_status(&connection, record.id, ServiceStatus::Active)
            .unwrap();
        assert_eq!(active.service.status(), ServiceStatus::Active);
        assert_eq!(active.created_at, record.created_at);
        assert_eq!(active.service.name(), record.service.name());
        assert_eq!(REPOSITORY.list_active(&connection).unwrap().len(), 1);
        assert!(REPOSITORY.list_frozen(&connection).unwrap().is_empty());
    }

    #[test]
    fn listados_separados_y_ordenados_por_created_at_e_id() {
        let connection = database::open_in_memory().unwrap();
        let active_first = "00000000-0000-4000-8000-000000000001";
        let active_second = "00000000-0000-4000-8000-000000000002";
        let frozen_first = "00000000-0000-4000-8000-000000000003";
        let frozen_second = "00000000-0000-4000-8000-000000000004";

        insert_raw_full(
            &connection,
            active_second,
            "Activo 2",
            10_000,
            "ARS",
            1,
            None,
            "active",
            "2026-01-01T00:00:00.001Z",
            VALID_TIMESTAMP,
        );
        insert_raw_full(
            &connection,
            active_first,
            "Activo 1",
            10_000,
            "ARS",
            1,
            None,
            "active",
            "2026-01-01T00:00:00.001Z",
            VALID_TIMESTAMP,
        );
        insert_raw_full(
            &connection,
            frozen_second,
            "Congelado 2",
            10_000,
            "ARS",
            1,
            None,
            "frozen",
            "2026-01-02T00:00:00.000Z",
            VALID_TIMESTAMP,
        );
        insert_raw_full(
            &connection,
            frozen_first,
            "Congelado 1",
            10_000,
            "ARS",
            1,
            None,
            "frozen",
            "2025-12-31T00:00:00.000Z",
            VALID_TIMESTAMP,
        );

        let active: Vec<String> = REPOSITORY
            .list_active(&connection)
            .unwrap()
            .iter()
            .map(|record| record.id.to_string())
            .collect();
        let frozen: Vec<String> = REPOSITORY
            .list_frozen(&connection)
            .unwrap()
            .iter()
            .map(|record| record.id.to_string())
            .collect();

        assert_eq!(active, vec![active_first, active_second]);
        assert_eq!(frozen, vec![frozen_first, frozen_second]);
    }

    #[test]
    fn conserva_la_moneda_despues_de_cerrar_y_reabrir_la_conexion() {
        let directory = tempfile::tempdir().unwrap();
        let path = database::resolve_path(directory.path());

        {
            let connection = database::open(&path).unwrap();
            REPOSITORY
                .insert(&connection, &ars("ARS", "100,00", 1))
                .unwrap();
            REPOSITORY
                .insert(&connection, &usd("USD", "10.00", 1))
                .unwrap();
        }

        let connection = database::open(&path).unwrap();
        let records = REPOSITORY.list_active(&connection).unwrap();
        let ars_record = records
            .iter()
            .find(|record| record.service.name() == "ARS")
            .unwrap();
        let usd_record = records
            .iter()
            .find(|record| record.service.name() == "USD")
            .unwrap();

        assert_eq!(ars_record.service.currency(), Currency::Ars);
        assert_eq!(ars_record.service.total_cost().minor_units(), 10_000);
        assert_eq!(usd_record.service.currency(), Currency::Usd);
        assert_eq!(usd_record.service.total_cost().minor_units(), 1_000);
    }

    #[test]
    fn timestamps_son_rfc3339_utc_con_milisegundos() {
        let connection = database::open_in_memory().unwrap();
        let record = REPOSITORY
            .insert(&connection, &ars("Luz", "100,00", 1))
            .unwrap();

        let raw_created: String = connection
            .query_row(
                "SELECT created_at FROM service WHERE id = ?1",
                params![record.id.to_string()],
                |row| row.get(0),
            )
            .unwrap();
        let raw_updated: String = connection
            .query_row(
                "SELECT updated_at FROM service WHERE id = ?1",
                params![record.id.to_string()],
                |row| row.get(0),
            )
            .unwrap();

        for raw in [&raw_created, &raw_updated] {
            let parsed = DateTime::parse_from_rfc3339(raw)
                .unwrap()
                .with_timezone(&Utc);
            assert_eq!(format_timestamp(parsed), raw.as_str());
            assert!(raw.ends_with('Z'));
        }
        assert_eq!(raw_created, format_timestamp(record.created_at));
        assert_eq!(raw_updated, format_timestamp(record.updated_at));
    }

    #[test]
    fn moneda_corruptas_devuelve_persistence() {
        let connection = database::open_in_memory().unwrap();
        let id = "00000000-0000-4000-8000-000000000010";
        insert_raw_full(
            &connection,
            id,
            "Luz",
            10_000,
            "EUR",
            1,
            None,
            "active",
            VALID_TIMESTAMP,
            VALID_TIMESTAMP,
        );

        let error = REPOSITORY
            .get(&connection, ServiceId::parse(id).unwrap())
            .unwrap_err();

        assert_persistence(&error, "currency_code");
    }

    #[test]
    fn estado_corrupto_devuelve_persistence() {
        let connection = database::open_in_memory().unwrap();
        let id = "00000000-0000-4000-8000-000000000011";
        insert_raw_full(
            &connection,
            id,
            "Luz",
            10_000,
            "ARS",
            1,
            None,
            "paused",
            VALID_TIMESTAMP,
            VALID_TIMESTAMP,
        );

        let error = REPOSITORY
            .get(&connection, ServiceId::parse(id).unwrap())
            .unwrap_err();

        assert_persistence(&error, "status");
    }

    #[test]
    fn cantidad_de_personas_corruptas_devuelve_persistence() {
        let connection = database::open_in_memory().unwrap();
        let zero_id = "00000000-0000-4000-8000-000000000012";
        let overflow_id = "00000000-0000-4000-8000-000000000013";

        insert_raw_full(
            &connection,
            zero_id,
            "Luz",
            10_000,
            "ARS",
            0,
            None,
            "active",
            VALID_TIMESTAMP,
            VALID_TIMESTAMP,
        );
        insert_raw_full(
            &connection,
            overflow_id,
            "Luz",
            10_000,
            "ARS",
            i64::from(u32::MAX) + 1,
            None,
            "active",
            VALID_TIMESTAMP,
            VALID_TIMESTAMP,
        );

        let zero_error = REPOSITORY
            .get(&connection, ServiceId::parse(zero_id).unwrap())
            .unwrap_err();
        let overflow_error = REPOSITORY
            .get(&connection, ServiceId::parse(overflow_id).unwrap())
            .unwrap_err();

        assert_persistence(&zero_error, "shared_people_count");
        assert_persistence(&overflow_error, "shared_people_count");
    }

    #[test]
    fn costo_corrupto_devuelve_persistence() {
        let connection = database::open_in_memory().unwrap();
        let id = "00000000-0000-4000-8000-000000000014";
        insert_raw_full(
            &connection,
            id,
            "Luz",
            -1,
            "ARS",
            1,
            None,
            "active",
            VALID_TIMESTAMP,
            VALID_TIMESTAMP,
        );

        let error = REPOSITORY
            .get(&connection, ServiceId::parse(id).unwrap())
            .unwrap_err();

        assert_persistence(&error, "total_cost_minor");
    }

    #[test]
    fn timestamps_corruptos_devuelven_persistence() {
        let connection = database::open_in_memory().unwrap();
        let bad_created_id = "00000000-0000-4000-8000-000000000015";
        let bad_updated_id = "00000000-0000-4000-8000-000000000016";

        insert_raw_full(
            &connection,
            bad_created_id,
            "Luz",
            10_000,
            "ARS",
            1,
            None,
            "active",
            "no-es-fecha",
            VALID_TIMESTAMP,
        );
        insert_raw_full(
            &connection,
            bad_updated_id,
            "Luz",
            10_000,
            "ARS",
            1,
            None,
            "active",
            VALID_TIMESTAMP,
            "no-es-fecha",
        );

        let created_error = REPOSITORY
            .get(&connection, ServiceId::parse(bad_created_id).unwrap())
            .unwrap_err();
        let updated_error = REPOSITORY
            .get(&connection, ServiceId::parse(bad_updated_id).unwrap())
            .unwrap_err();

        assert_persistence(&created_error, "created_at");
        assert_persistence(&updated_error, "updated_at");
    }

    #[test]
    fn id_corrupto_devuelve_persistence_al_listar() {
        let connection = database::open_in_memory().unwrap();
        insert_raw_full(
            &connection,
            "no-es-uuid",
            "Luz",
            10_000,
            "ARS",
            1,
            None,
            "active",
            VALID_TIMESTAMP,
            VALID_TIMESTAMP,
        );

        let error = REPOSITORY.list_active(&connection).unwrap_err();

        assert_persistence(&error, "id");
    }

    #[test]
    fn id_valido_no_canonico_se_rechaza_como_registro_corrupto() {
        for raw_id in [
            "00000000000040008000000000000001",
            "00000000-0000-4000-8000-000000000ABC",
        ] {
            let connection = database::open_in_memory().unwrap();
            assert!(ServiceId::parse(raw_id).is_some());

            insert_raw_full(
                &connection,
                raw_id,
                "Luz",
                10_000,
                "ARS",
                1,
                None,
                "active",
                VALID_TIMESTAMP,
                VALID_TIMESTAMP,
            );

            let error = REPOSITORY.list_active(&connection).unwrap_err();

            assert_persistence(&error, "id");
        }
    }

    #[test]
    fn no_persiste_calculos_derivados() {
        let connection = database::open_in_memory().unwrap();
        REPOSITORY
            .insert(&connection, &ars("Luz", "10000,00", 3))
            .unwrap();

        let mut statement = connection.prepare("PRAGMA table_info(service)").unwrap();
        let columns: Vec<String> = statement
            .query_map([], |row| row.get::<_, String>(1))
            .unwrap()
            .map(|value| value.unwrap())
            .collect();

        assert_eq!(
            columns,
            vec![
                "id",
                "name",
                "total_cost_minor",
                "currency_code",
                "shared_people_count",
                "notes",
                "status",
                "created_at",
                "updated_at",
            ]
        );
    }
}
