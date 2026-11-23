use chrono::{DateTime, Duration, SecondsFormat, Utc};
use rusqlite::{Connection, OptionalExtension, Row, params};

use crate::application::settings_repository::{SettingKey, SettingRecord, SettingsRepository};
use crate::domain::error::DomainError;
use crate::infrastructure::database;

const SELECT_BY_KEY: &str = "SELECT key, value, updated_at FROM app_setting WHERE key = ?1";
const SELECT_UPDATED_AT_BY_KEY: &str = "SELECT updated_at FROM app_setting WHERE key = ?1";
const UPSERT_SETTING: &str = "INSERT INTO app_setting (key, value, updated_at) VALUES (?1, ?2, ?3) ON CONFLICT(key) DO UPDATE SET value = excluded.value, updated_at = excluded.updated_at";

#[derive(Debug, Default)]
pub struct SqliteSettingsRepository;

impl SettingsRepository for SqliteSettingsRepository {
    fn set(
        &self,
        connection: &Connection,
        key: &SettingKey,
        value: &str,
    ) -> Result<SettingRecord, DomainError> {
        let transaction = connection
            .unchecked_transaction()
            .map_err(database::sqlite_error)?;
        let previous_updated_at = transaction
            .query_row(SELECT_UPDATED_AT_BY_KEY, params![key.as_str()], |row| {
                row.get::<_, String>(0)
            })
            .optional()
            .map_err(database::sqlite_error)?
            .map(|timestamp| parse_timestamp(&timestamp, "updated_at", Some(key.as_str())))
            .transpose()?;
        let updated_at =
            next_timestamp(now_truncated_to_millis(), previous_updated_at, key.as_str())?;

        transaction
            .execute(
                UPSERT_SETTING,
                params![key.as_str(), value, format_timestamp(updated_at)],
            )
            .map_err(database::sqlite_error)?;

        let row = transaction
            .query_row(
                SELECT_BY_KEY,
                params![key.as_str()],
                RawSettingRow::from_row,
            )
            .map_err(database::sqlite_error)?;
        let record = row.into_record()?;
        transaction.commit().map_err(database::sqlite_error)?;

        Ok(record)
    }

    fn get(
        &self,
        connection: &Connection,
        key: &SettingKey,
    ) -> Result<Option<SettingRecord>, DomainError> {
        let row = connection
            .query_row(
                SELECT_BY_KEY,
                params![key.as_str()],
                RawSettingRow::from_row,
            )
            .optional()
            .map_err(database::sqlite_error)?;

        row.map(RawSettingRow::into_record).transpose()
    }
}

#[derive(Debug)]
struct RawSettingRow {
    key: String,
    value: String,
    updated_at: String,
}

impl RawSettingRow {
    fn from_row(row: &Row<'_>) -> rusqlite::Result<Self> {
        Ok(Self {
            key: row.get(0)?,
            value: row.get(1)?,
            updated_at: row.get(2)?,
        })
    }

    fn into_record(self) -> Result<SettingRecord, DomainError> {
        let key = SettingKey::parse(self.key.as_str())
            .map_err(|_| persistence_context("key", self.key.as_str(), Some(self.key.as_str())))?;
        let updated_at =
            parse_timestamp(self.updated_at.as_str(), "updated_at", Some(key.as_str()))?;

        Ok(SettingRecord {
            key,
            value: self.value,
            updated_at,
        })
    }
}

fn next_timestamp(
    now: DateTime<Utc>,
    previous: Option<DateTime<Utc>>,
    key: &str,
) -> Result<DateTime<Utc>, DomainError> {
    match previous {
        Some(previous) if now <= previous => previous
            .checked_add_signed(Duration::milliseconds(1))
            .ok_or_else(|| {
                persistence_context(
                    "updated_at",
                    "desbordamiento al avanzar el timestamp",
                    Some(key),
                )
            }),
        _ => Ok(now),
    }
}

fn now_truncated_to_millis() -> DateTime<Utc> {
    DateTime::from_timestamp_millis(Utc::now().timestamp_millis())
        .expect("el timestamp actual debe ser válido")
}

fn format_timestamp(value: DateTime<Utc>) -> String {
    value.to_rfc3339_opts(SecondsFormat::Millis, true)
}

fn parse_timestamp(
    value: &str,
    column: &str,
    key: Option<&str>,
) -> Result<DateTime<Utc>, DomainError> {
    DateTime::parse_from_rfc3339(value)
        .map(|timestamp| timestamp.with_timezone(&Utc))
        .map_err(|error| persistence_context(column, format!("{value} ({error})"), key))
}

fn persistence_context(column: &str, detail: impl AsRef<str>, key: Option<&str>) -> DomainError {
    let detail = detail.as_ref();
    match key {
        Some(key) => DomainError::persistence(format!(
            "registro corrupto en app_setting.key={key}: columna {column} inválida ({detail})"
        )),
        None => DomainError::persistence(format!(
            "registro corrupto: columna {column} inválida ({detail})"
        )),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const REPOSITORY: SqliteSettingsRepository = SqliteSettingsRepository;
    const FUTURE_TIMESTAMP: &str = "2999-01-01T00:00:00.000Z";

    fn key(value: &str) -> SettingKey {
        SettingKey::parse(value).unwrap()
    }

    fn assert_persistence(error: &DomainError, fragment: &str) {
        match error {
            DomainError::Persistence(message) => {
                assert!(message.contains(fragment), "mensaje inesperado: {message}")
            }
            other => panic!("se esperaba Persistence, se obtuvo {other:?}"),
        }
    }

    fn insert_raw_setting(connection: &Connection, raw_key: &str, value: &str, updated_at: &str) {
        connection
            .execute(
                "INSERT INTO app_setting (key, value, updated_at) VALUES (?1, ?2, ?3)",
                params![raw_key, value, updated_at],
            )
            .unwrap();
    }

    fn insert_sentinel_financial_rows(connection: &Connection) {
        connection
            .execute_batch(
                "INSERT INTO service (id, name, total_cost_minor, currency_code, shared_people_count, notes, status, created_at, updated_at) VALUES ('00000000-0000-4000-8000-000000000001', 'Servicio centinela', 100000, 'ARS', 1, NULL, 'active', '2026-01-01T00:00:00.000Z', '2026-01-01T00:00:00.000Z'); \
                 INSERT INTO investment (id, month, capital_minor, currency_code, tna_basis_points, term_days, tax_minor, created_at, updated_at) VALUES ('00000000-0000-4000-8000-000000000002', '2026-03', 200000, 'ARS', 2500, 30, 0, '2026-01-01T00:00:00.000Z', '2026-01-01T00:00:00.000Z');",
            )
            .unwrap();
    }

    #[test]
    fn inserta_y_lee_preferencia() {
        let connection = database::open_in_memory().unwrap();

        let record = REPOSITORY.set(&connection, &key("theme"), "dark").unwrap();
        let loaded = REPOSITORY.get(&connection, &key("theme")).unwrap().unwrap();

        assert_eq!(loaded.key(), record.key());
        assert_eq!(loaded.value(), "dark");
        assert_eq!(loaded.updated_at(), record.updated_at());
    }

    #[test]
    fn actualiza_preferencia_existente() {
        let connection = database::open_in_memory().unwrap();
        REPOSITORY.set(&connection, &key("theme"), "dark").unwrap();

        let updated = REPOSITORY.set(&connection, &key("theme"), "light").unwrap();
        let loaded = REPOSITORY.get(&connection, &key("theme")).unwrap().unwrap();

        assert_eq!(updated.value(), "light");
        assert_eq!(loaded.value(), "light");
    }

    #[test]
    fn actualizacion_reemplaza_valor_anterior() {
        let connection = database::open_in_memory().unwrap();
        let original = REPOSITORY.set(&connection, &key("theme"), "dark").unwrap();

        let updated = REPOSITORY.set(&connection, &key("theme"), "light").unwrap();

        assert_ne!(original.value(), updated.value());
        assert_eq!(original.value(), "dark");
        assert_eq!(updated.value(), "light");
    }

    #[test]
    fn updated_at_avanza_en_actualizacion_inmediata() {
        let connection = database::open_in_memory().unwrap();
        insert_raw_setting(&connection, "theme", "dark", FUTURE_TIMESTAMP);
        let previous = DateTime::parse_from_rfc3339(FUTURE_TIMESTAMP)
            .unwrap()
            .with_timezone(&Utc);

        let updated = REPOSITORY.set(&connection, &key("theme"), "light").unwrap();

        assert!(updated.updated_at() > previous);
    }

    #[test]
    fn get_de_clave_inexistente_devuelve_none() {
        let connection = database::open_in_memory().unwrap();

        let loaded = REPOSITORY.get(&connection, &key("missing")).unwrap();

        assert!(loaded.is_none());
    }

    #[test]
    fn multiples_claves_independientes() {
        let connection = database::open_in_memory().unwrap();
        REPOSITORY.set(&connection, &key("theme"), "dark").unwrap();
        REPOSITORY
            .set(&connection, &key("language"), "es-AR")
            .unwrap();

        let theme = REPOSITORY.get(&connection, &key("theme")).unwrap().unwrap();
        let language = REPOSITORY
            .get(&connection, &key("language"))
            .unwrap()
            .unwrap();

        assert_eq!(theme.value(), "dark");
        assert_eq!(language.value(), "es-AR");
    }

    #[test]
    fn persiste_despues_de_cerrar_y_reabrir() {
        let directory = tempfile::tempdir().unwrap();
        let path = database::resolve_path(directory.path());

        {
            let connection = database::open(&path).unwrap();
            REPOSITORY.set(&connection, &key("theme"), "dark").unwrap();
        }

        let connection = database::open(&path).unwrap();
        let loaded = REPOSITORY.get(&connection, &key("theme")).unwrap().unwrap();

        assert_eq!(loaded.value(), "dark");
    }

    #[test]
    fn updated_at_invalido_devuelve_persistence() {
        let connection = database::open_in_memory().unwrap();
        insert_raw_setting(&connection, "theme", "dark", "no-es-fecha");

        let error = REPOSITORY.get(&connection, &key("theme")).unwrap_err();

        assert_persistence(&error, "updated_at");
    }

    #[test]
    fn usa_exclusivamente_app_setting_y_no_altera_datos_financieros() {
        let connection = database::open_in_memory().unwrap();
        insert_sentinel_financial_rows(&connection);

        REPOSITORY.set(&connection, &key("theme"), "dark").unwrap();
        let loaded = REPOSITORY.get(&connection, &key("theme")).unwrap().unwrap();

        let service_name: String = connection
            .query_row("SELECT name FROM service", [], |row| row.get(0))
            .unwrap();
        let service_count: i64 = connection
            .query_row("SELECT COUNT(*) FROM service", [], |row| row.get(0))
            .unwrap();
        let investment_count: i64 = connection
            .query_row("SELECT COUNT(*) FROM investment", [], |row| row.get(0))
            .unwrap();
        let investment_capital: i64 = connection
            .query_row("SELECT capital_minor FROM investment", [], |row| row.get(0))
            .unwrap();

        assert_eq!(loaded.value(), "dark");
        assert_eq!(service_name, "Servicio centinela");
        assert_eq!(service_count, 1);
        assert_eq!(investment_count, 1);
        assert_eq!(investment_capital, 200000);
    }

    #[test]
    fn setting_key_vacia_no_se_puede_almacenar() {
        let connection = database::open_in_memory().unwrap();
        let empty_key = SettingKey::parse("").unwrap_err();

        assert!(matches!(
            empty_key,
            DomainError::Validation { field: "key", .. }
        ));
        assert_eq!(
            connection
                .query_row("SELECT COUNT(*) FROM app_setting", [], |row| row
                    .get::<_, i64>(0))
                .unwrap(),
            0
        );
    }
}
