use rusqlite::Connection;

use crate::application::service_repository::{ServiceId, ServiceRecord, ServiceRepository};
use crate::domain::error::DomainError;
use crate::domain::money::Currency;
use crate::domain::service::{Service, ServiceStatus};
use crate::domain::service_totals::{ServiceTotals, calculate_service_totals};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CreateServiceInput {
    pub name: String,
    pub total_cost: String,
    pub currency: Currency,
    pub shared_people_count: u32,
    pub notes: Option<String>,
}

impl Default for CreateServiceInput {
    fn default() -> Self {
        Self {
            name: String::new(),
            total_cost: String::new(),
            currency: Currency::Ars,
            shared_people_count: 1,
            notes: None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UpdateServiceInput {
    pub name: String,
    pub total_cost: String,
    pub shared_people_count: u32,
    pub notes: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DeleteConfirmation {
    Confirmed,
    Cancelled,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DeleteServiceOutcome {
    Deleted,
    Cancelled,
}

#[derive(Debug, Clone)]
pub enum FreezeServiceOutcome {
    Frozen(ServiceRecord),
    AlreadyFrozen(ServiceRecord),
}

#[derive(Debug, Clone)]
pub enum ReactivateServiceOutcome {
    Reactivated(ServiceRecord),
    AlreadyActive(ServiceRecord),
}

pub fn create_service<R>(
    connection: &Connection,
    repository: &R,
    input: CreateServiceInput,
) -> Result<ServiceRecord, DomainError>
where
    R: ServiceRepository + ?Sized,
{
    let service = Service::new_with_currency(
        input.name,
        input.total_cost.as_str(),
        input.currency,
        input.shared_people_count,
        input.notes,
    )?;

    repository.insert(connection, &service)
}

pub fn update_service<R>(
    connection: &Connection,
    repository: &R,
    id: ServiceId,
    input: UpdateServiceInput,
) -> Result<ServiceRecord, DomainError>
where
    R: ServiceRepository + ?Sized,
{
    let current = repository
        .get(connection, id)?
        .ok_or(DomainError::NotFound)?;
    let replacement = current.service().edit(
        input.name,
        input.total_cost.as_str(),
        input.shared_people_count,
        input.notes,
    )?;

    repository.update(connection, id, &replacement)
}

pub fn delete_service<R>(
    connection: &Connection,
    repository: &R,
    id: ServiceId,
    confirmation: DeleteConfirmation,
) -> Result<DeleteServiceOutcome, DomainError>
where
    R: ServiceRepository + ?Sized,
{
    match confirmation {
        DeleteConfirmation::Cancelled => Ok(DeleteServiceOutcome::Cancelled),
        DeleteConfirmation::Confirmed => {
            repository.delete(connection, id)?;
            Ok(DeleteServiceOutcome::Deleted)
        }
    }
}

pub fn freeze_service<R>(
    connection: &Connection,
    repository: &R,
    id: ServiceId,
) -> Result<FreezeServiceOutcome, DomainError>
where
    R: ServiceRepository + ?Sized,
{
    let current = repository
        .get(connection, id)?
        .ok_or(DomainError::NotFound)?;

    if current.service().status() == ServiceStatus::Frozen {
        return Ok(FreezeServiceOutcome::AlreadyFrozen(current));
    }

    let frozen = current.service().freeze();
    let record = repository.set_status(connection, id, frozen.status())?;
    Ok(FreezeServiceOutcome::Frozen(record))
}

pub fn reactivate_service<R>(
    connection: &Connection,
    repository: &R,
    id: ServiceId,
) -> Result<ReactivateServiceOutcome, DomainError>
where
    R: ServiceRepository + ?Sized,
{
    let current = repository
        .get(connection, id)?
        .ok_or(DomainError::NotFound)?;

    if current.service().status() == ServiceStatus::Active {
        return Ok(ReactivateServiceOutcome::AlreadyActive(current));
    }

    let active = current.service().reactivate();
    let record = repository.set_status(connection, id, active.status())?;
    Ok(ReactivateServiceOutcome::Reactivated(record))
}

pub fn list_frozen_services<R>(
    connection: &Connection,
    repository: &R,
) -> Result<Vec<ServiceRecord>, DomainError>
where
    R: ServiceRepository + ?Sized,
{
    repository.list_frozen(connection)
}

pub fn get_service_totals<R>(
    connection: &Connection,
    repository: &R,
) -> Result<ServiceTotals, DomainError>
where
    R: ServiceRepository + ?Sized,
{
    let records = repository.list_active(connection)?;
    let services: Vec<Service> = records
        .iter()
        .map(|record| record.service().clone())
        .collect();
    calculate_service_totals(&services)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::infrastructure::database;
    use crate::infrastructure::service_repository::SqliteServiceRepository;

    const REPOSITORY: SqliteServiceRepository = SqliteServiceRepository;

    fn ars_input(name: &str, total_cost: &str, shared_people_count: u32) -> CreateServiceInput {
        CreateServiceInput {
            name: name.to_owned(),
            total_cost: total_cost.to_owned(),
            currency: Currency::Ars,
            shared_people_count,
            notes: None,
        }
    }

    fn usd_input(name: &str, total_cost: &str, shared_people_count: u32) -> CreateServiceInput {
        CreateServiceInput {
            name: name.to_owned(),
            total_cost: total_cost.to_owned(),
            currency: Currency::Usd,
            shared_people_count,
            notes: None,
        }
    }

    fn update_input(name: &str, total_cost: &str, shared_people_count: u32) -> UpdateServiceInput {
        UpdateServiceInput {
            name: name.to_owned(),
            total_cost: total_cost.to_owned(),
            shared_people_count,
            notes: None,
        }
    }

    fn unknown_id() -> ServiceId {
        ServiceId::parse("00000000-0000-4000-8000-000000000099").unwrap()
    }

    fn record_from_freeze(outcome: FreezeServiceOutcome) -> ServiceRecord {
        match outcome {
            FreezeServiceOutcome::Frozen(record) | FreezeServiceOutcome::AlreadyFrozen(record) => {
                record
            }
        }
    }

    fn record_from_reactivate(outcome: ReactivateServiceOutcome) -> ServiceRecord {
        match outcome {
            ReactivateServiceOutcome::Reactivated(record)
            | ReactivateServiceOutcome::AlreadyActive(record) => record,
        }
    }

    struct FailingRepository;

    impl FailingRepository {
        fn error() -> DomainError {
            DomainError::Persistence("repositorio no disponible".to_owned())
        }
    }

    impl ServiceRepository for FailingRepository {
        fn insert(
            &self,
            _connection: &Connection,
            _service: &Service,
        ) -> Result<ServiceRecord, DomainError> {
            Err(Self::error())
        }

        fn get(
            &self,
            _connection: &Connection,
            _id: ServiceId,
        ) -> Result<Option<ServiceRecord>, DomainError> {
            Err(Self::error())
        }

        fn update(
            &self,
            _connection: &Connection,
            _id: ServiceId,
            _service: &Service,
        ) -> Result<ServiceRecord, DomainError> {
            Err(Self::error())
        }

        fn delete(&self, _connection: &Connection, _id: ServiceId) -> Result<(), DomainError> {
            Err(Self::error())
        }

        fn set_status(
            &self,
            _connection: &Connection,
            _id: ServiceId,
            _status: ServiceStatus,
        ) -> Result<ServiceRecord, DomainError> {
            Err(Self::error())
        }

        fn list_active(&self, _connection: &Connection) -> Result<Vec<ServiceRecord>, DomainError> {
            Err(Self::error())
        }

        fn list_frozen(&self, _connection: &Connection) -> Result<Vec<ServiceRecord>, DomainError> {
            Err(Self::error())
        }
    }

    #[test]
    fn alta_valida_devuelve_registro_activo_y_parte_calculada() {
        let connection = database::open_in_memory().unwrap();

        let record = create_service(
            &connection,
            &REPOSITORY,
            ars_input("Spotify", "10000,00", 4),
        )
        .unwrap();
        let share = record.service().my_share().unwrap();

        assert_eq!(record.service().name(), "Spotify");
        assert_eq!(
            record.service().status(),
            crate::domain::service::ServiceStatus::Active
        );
        assert_eq!(share.numerator_minor(), 250_000);
        assert_eq!(share.denominator(), 1);
        assert_eq!(share.currency(), Currency::Ars);
    }

    #[test]
    fn alta_usa_ars_por_defecto() {
        let connection = database::open_in_memory().unwrap();
        let input = CreateServiceInput {
            name: "Luz".to_owned(),
            total_cost: "100,00".to_owned(),
            shared_people_count: 1,
            ..Default::default()
        };

        let record = create_service(&connection, &REPOSITORY, input).unwrap();

        assert_eq!(record.service().currency(), Currency::Ars);
    }

    #[test]
    fn alta_en_usd_no_convierte() {
        let connection = database::open_in_memory().unwrap();
        let input = CreateServiceInput {
            name: "Netflix".to_owned(),
            total_cost: "15.99".to_owned(),
            currency: Currency::Usd,
            shared_people_count: 2,
            notes: None,
        };

        let record = create_service(&connection, &REPOSITORY, input).unwrap();

        assert_eq!(record.service().currency(), Currency::Usd);
        assert_eq!(record.service().total_cost().minor_units(), 1_599);
    }

    #[test]
    fn alta_invalida_no_persiste() {
        let connection = database::open_in_memory().unwrap();
        let input = ars_input("   ", "100,00", 1);

        let error = create_service(&connection, &REPOSITORY, input).unwrap_err();

        assert!(matches!(
            error,
            DomainError::Validation { field: "name", .. }
        ));
        assert!(REPOSITORY.list_active(&connection).unwrap().is_empty());
    }

    #[test]
    fn edicion_recalcula_la_parte() {
        let connection = database::open_in_memory().unwrap();
        let created = create_service(
            &connection,
            &REPOSITORY,
            ars_input("Spotify", "10000,00", 4),
        )
        .unwrap();

        let updated = update_service(
            &connection,
            &REPOSITORY,
            created.id(),
            update_input("Spotify", "12000,00", 4),
        )
        .unwrap();
        let share = updated.service().my_share().unwrap();

        assert_eq!(updated.service().total_cost().minor_units(), 1_200_000);
        assert_eq!(share.numerator_minor(), 300_000);
        assert_eq!(share.denominator(), 1);
    }

    #[test]
    fn edicion_invalida_no_modifica_el_servicio() {
        let connection = database::open_in_memory().unwrap();
        let created = create_service(
            &connection,
            &REPOSITORY,
            ars_input("Spotify", "10000,00", 4),
        )
        .unwrap();

        let error = update_service(
            &connection,
            &REPOSITORY,
            created.id(),
            update_input("", "12000,00", 4),
        )
        .unwrap_err();

        assert!(matches!(
            error,
            DomainError::Validation { field: "name", .. }
        ));
        let loaded = REPOSITORY
            .get(&connection, created.id())
            .unwrap()
            .expect("el servicio original debe permanecer");
        assert_eq!(loaded.service().name(), "Spotify");
        assert_eq!(loaded.service().total_cost().minor_units(), 1_000_000);
        assert_eq!(loaded.service().shared_people_count().get(), 4);
    }

    #[test]
    fn eliminacion_confirmada_quita_el_servicio() {
        let connection = database::open_in_memory().unwrap();
        let created =
            create_service(&connection, &REPOSITORY, ars_input("Luz", "100,00", 1)).unwrap();

        let outcome = delete_service(
            &connection,
            &REPOSITORY,
            created.id(),
            DeleteConfirmation::Confirmed,
        )
        .unwrap();

        assert_eq!(outcome, DeleteServiceOutcome::Deleted);
        assert!(REPOSITORY.get(&connection, created.id()).unwrap().is_none());
        assert!(REPOSITORY.list_active(&connection).unwrap().is_empty());
    }

    #[test]
    fn eliminacion_cancelada_no_modifica_el_servicio() {
        let connection = database::open_in_memory().unwrap();
        let created =
            create_service(&connection, &REPOSITORY, ars_input("Luz", "100,00", 1)).unwrap();

        let outcome = delete_service(
            &connection,
            &REPOSITORY,
            created.id(),
            DeleteConfirmation::Cancelled,
        )
        .unwrap();

        assert_eq!(outcome, DeleteServiceOutcome::Cancelled);
        let loaded = REPOSITORY
            .get(&connection, created.id())
            .unwrap()
            .expect("el servicio no debe eliminarse");
        assert_eq!(loaded.service().name(), "Luz");
        assert_eq!(REPOSITORY.list_active(&connection).unwrap().len(), 1);
    }

    #[test]
    fn congelar_activo_cambia_estado_y_conserva_datos() {
        let connection = database::open_in_memory().unwrap();
        let created = create_service(
            &connection,
            &REPOSITORY,
            CreateServiceInput {
                name: "Spotify".to_owned(),
                total_cost: "10000,00".to_owned(),
                currency: Currency::Usd,
                shared_people_count: 4,
                notes: Some("familiar".to_owned()),
            },
        )
        .unwrap();

        let outcome = freeze_service(&connection, &REPOSITORY, created.id()).unwrap();
        assert!(matches!(&outcome, FreezeServiceOutcome::Frozen(_)));
        let record = record_from_freeze(outcome);

        assert_eq!(record.service().status(), ServiceStatus::Frozen);
        assert_eq!(record.service().name(), "Spotify");
        assert_eq!(record.service().total_cost().minor_units(), 1_000_000);
        assert_eq!(record.service().currency(), Currency::Usd);
        assert_eq!(record.service().shared_people_count().get(), 4);
        assert_eq!(record.service().notes(), Some("familiar"));
        assert_eq!(record.created_at(), created.created_at());

        let active_ids: Vec<String> = REPOSITORY
            .list_active(&connection)
            .unwrap()
            .iter()
            .map(|record| record.id().to_string())
            .collect();
        let frozen_ids: Vec<String> = REPOSITORY
            .list_frozen(&connection)
            .unwrap()
            .iter()
            .map(|record| record.id().to_string())
            .collect();
        assert!(!active_ids.contains(&created.id().to_string()));
        assert!(frozen_ids.contains(&created.id().to_string()));
    }

    #[test]
    fn congelar_ya_congelado_no_modifica_estado_ni_updated_at() {
        let connection = database::open_in_memory().unwrap();
        let created =
            create_service(&connection, &REPOSITORY, ars_input("Luz", "100,00", 1)).unwrap();
        let frozen =
            record_from_freeze(freeze_service(&connection, &REPOSITORY, created.id()).unwrap());

        let outcome = freeze_service(&connection, &REPOSITORY, created.id()).unwrap();
        assert!(matches!(&outcome, FreezeServiceOutcome::AlreadyFrozen(_)));
        let record = record_from_freeze(outcome);

        assert_eq!(record.service().status(), ServiceStatus::Frozen);
        assert_eq!(record.created_at(), frozen.created_at());
        assert_eq!(record.updated_at(), frozen.updated_at());
        assert_eq!(record.service().name(), frozen.service().name());
        assert_eq!(
            record.service().total_cost().minor_units(),
            frozen.service().total_cost().minor_units()
        );
        assert_eq!(record.service().currency(), frozen.service().currency());
        assert_eq!(
            record.service().shared_people_count().get(),
            frozen.service().shared_people_count().get()
        );
        assert_eq!(record.service().notes(), frozen.service().notes());

        assert!(REPOSITORY.list_active(&connection).unwrap().is_empty());
        let frozen_ids: Vec<String> = REPOSITORY
            .list_frozen(&connection)
            .unwrap()
            .iter()
            .map(|record| record.id().to_string())
            .collect();
        assert_eq!(frozen_ids, vec![created.id().to_string()]);
    }

    #[test]
    fn reactivar_congelado_cambia_estado_y_conserva_datos() {
        let connection = database::open_in_memory().unwrap();
        let created = create_service(
            &connection,
            &REPOSITORY,
            CreateServiceInput {
                name: "Netflix".to_owned(),
                total_cost: "15,99".to_owned(),
                currency: Currency::Usd,
                shared_people_count: 2,
                notes: Some("familiar".to_owned()),
            },
        )
        .unwrap();
        let frozen =
            record_from_freeze(freeze_service(&connection, &REPOSITORY, created.id()).unwrap());

        let outcome = reactivate_service(&connection, &REPOSITORY, created.id()).unwrap();
        assert!(matches!(&outcome, ReactivateServiceOutcome::Reactivated(_)));
        let record = record_from_reactivate(outcome);

        assert_eq!(record.service().status(), ServiceStatus::Active);
        assert_eq!(record.service().name(), "Netflix");
        assert_eq!(record.service().total_cost().minor_units(), 1_599);
        assert_eq!(record.service().currency(), Currency::Usd);
        assert_eq!(record.service().shared_people_count().get(), 2);
        assert_eq!(record.service().notes(), Some("familiar"));
        assert_eq!(record.created_at(), created.created_at());
        assert_eq!(frozen.service().status(), ServiceStatus::Frozen);

        let active_ids: Vec<String> = REPOSITORY
            .list_active(&connection)
            .unwrap()
            .iter()
            .map(|record| record.id().to_string())
            .collect();
        let frozen_ids: Vec<String> = REPOSITORY
            .list_frozen(&connection)
            .unwrap()
            .iter()
            .map(|record| record.id().to_string())
            .collect();
        assert_eq!(active_ids, vec![created.id().to_string()]);
        assert!(frozen_ids.is_empty());
    }

    #[test]
    fn reactivar_activo_no_modifica_estado_ni_updated_at() {
        let connection = database::open_in_memory().unwrap();
        let created =
            create_service(&connection, &REPOSITORY, ars_input("Luz", "100,00", 1)).unwrap();

        let outcome = reactivate_service(&connection, &REPOSITORY, created.id()).unwrap();
        assert!(matches!(
            &outcome,
            ReactivateServiceOutcome::AlreadyActive(_)
        ));
        let record = record_from_reactivate(outcome);

        assert_eq!(record.service().status(), ServiceStatus::Active);
        assert_eq!(record.created_at(), created.created_at());
        assert_eq!(record.updated_at(), created.updated_at());
        assert_eq!(record.service().name(), created.service().name());
        assert_eq!(
            record.service().total_cost().minor_units(),
            created.service().total_cost().minor_units()
        );
        assert_eq!(record.service().currency(), created.service().currency());
        assert_eq!(
            record.service().shared_people_count().get(),
            created.service().shared_people_count().get()
        );
        assert_eq!(record.service().notes(), created.service().notes());

        assert_eq!(REPOSITORY.list_active(&connection).unwrap().len(), 1);
        assert!(REPOSITORY.list_frozen(&connection).unwrap().is_empty());
    }

    #[test]
    fn listar_congelados_devuelve_solo_registros_frozen() {
        let connection = database::open_in_memory().unwrap();
        let active =
            create_service(&connection, &REPOSITORY, ars_input("Activo", "100,00", 1)).unwrap();
        let first = create_service(
            &connection,
            &REPOSITORY,
            ars_input("Congelado 1", "200,00", 1),
        )
        .unwrap();
        let second = create_service(
            &connection,
            &REPOSITORY,
            ars_input("Congelado 2", "300,00", 1),
        )
        .unwrap();
        let first_frozen =
            record_from_freeze(freeze_service(&connection, &REPOSITORY, first.id()).unwrap());
        let second_frozen =
            record_from_freeze(freeze_service(&connection, &REPOSITORY, second.id()).unwrap());

        let records = list_frozen_services(&connection, &REPOSITORY).unwrap();

        assert_eq!(records.len(), 2);
        assert!(
            records
                .iter()
                .all(|record| record.service().status() == ServiceStatus::Frozen)
        );
        let actual: Vec<String> = records
            .iter()
            .map(|record| record.id().to_string())
            .collect();
        assert!(!actual.contains(&active.id().to_string()));

        let mut expected = vec![
            (first_frozen.created_at(), first_frozen.id().to_string()),
            (second_frozen.created_at(), second_frozen.id().to_string()),
        ];
        expected.sort_by(|left, right| left.0.cmp(&right.0).then_with(|| left.1.cmp(&right.1)));
        let expected_ids: Vec<String> = expected.into_iter().map(|(_, id)| id).collect();
        assert_eq!(actual, expected_ids);
    }

    #[test]
    fn congelar_y_reactivar_id_inexistente_devuelve_not_found() {
        let connection = database::open_in_memory().unwrap();
        let id = unknown_id();

        let freeze_error = freeze_service(&connection, &REPOSITORY, id).unwrap_err();
        assert!(matches!(freeze_error, DomainError::NotFound));

        let reactivate_error = reactivate_service(&connection, &REPOSITORY, id).unwrap_err();
        assert!(matches!(reactivate_error, DomainError::NotFound));
    }

    #[test]
    fn totales_de_referencia_devuelve_costo_y_gasto_esperados() {
        let connection = database::open_in_memory().unwrap();
        create_service(&connection, &REPOSITORY, ars_input("A", "10000,00", 4)).unwrap();
        create_service(&connection, &REPOSITORY, ars_input("B", "6000,00", 2)).unwrap();

        let totals = get_service_totals(&connection, &REPOSITORY).unwrap();

        assert_eq!(totals.ars().total_cost().minor_units(), 1_600_000);
        assert_eq!(totals.ars().user_share().numerator_minor(), 550_000);
        assert_eq!(totals.ars().user_share().denominator(), 1);
        assert_eq!(totals.usd().total_cost().minor_units(), 0);
        assert_eq!(totals.usd().user_share().numerator_minor(), 0);
    }

    #[test]
    fn totales_sin_servicios_activos_devuelve_cero() {
        let connection = database::open_in_memory().unwrap();

        let totals = get_service_totals(&connection, &REPOSITORY).unwrap();

        assert_eq!(totals.ars().total_cost().minor_units(), 0);
        assert_eq!(totals.ars().total_cost().currency(), Currency::Ars);
        assert_eq!(totals.ars().user_share().numerator_minor(), 0);
        assert_eq!(totals.ars().user_share().denominator(), 1);
        assert_eq!(totals.ars().user_share().currency(), Currency::Ars);
        assert_eq!(totals.usd().total_cost().minor_units(), 0);
        assert_eq!(totals.usd().total_cost().currency(), Currency::Usd);
        assert_eq!(totals.usd().user_share().numerator_minor(), 0);
        assert_eq!(totals.usd().user_share().denominator(), 1);
        assert_eq!(totals.usd().user_share().currency(), Currency::Usd);
    }

    #[test]
    fn totales_separados_por_moneda_sin_conversion() {
        let connection = database::open_in_memory().unwrap();
        create_service(&connection, &REPOSITORY, ars_input("Luz", "10000,00", 4)).unwrap();
        create_service(&connection, &REPOSITORY, usd_input("Netflix", "20,00", 2)).unwrap();

        let totals = get_service_totals(&connection, &REPOSITORY).unwrap();

        assert_eq!(totals.ars().total_cost().minor_units(), 1_000_000);
        assert_eq!(totals.ars().total_cost().currency(), Currency::Ars);
        assert_eq!(totals.ars().user_share().numerator_minor(), 250_000);
        assert_eq!(totals.ars().user_share().denominator(), 1);
        assert_eq!(totals.usd().total_cost().minor_units(), 2_000);
        assert_eq!(totals.usd().total_cost().currency(), Currency::Usd);
        assert_eq!(totals.usd().user_share().numerator_minor(), 1_000);
        assert_eq!(totals.usd().user_share().denominator(), 1);
        assert_ne!(
            totals.ars().total_cost().currency(),
            totals.usd().total_cost().currency()
        );
    }

    #[test]
    fn totales_excluyen_servicios_congelados() {
        let connection = database::open_in_memory().unwrap();
        let active = create_service(
            &connection,
            &REPOSITORY,
            ars_input("Spotify", "10000,00", 4),
        )
        .unwrap();
        let frozen =
            create_service(&connection, &REPOSITORY, ars_input("Luz", "5000,00", 2)).unwrap();
        freeze_service(&connection, &REPOSITORY, frozen.id()).unwrap();

        let totals = get_service_totals(&connection, &REPOSITORY).unwrap();

        assert_eq!(active.service().status(), ServiceStatus::Active);
        assert_eq!(totals.ars().total_cost().minor_units(), 1_000_000);
        assert_eq!(totals.ars().user_share().numerator_minor(), 250_000);
        assert_eq!(totals.ars().user_share().denominator(), 1);
    }

    #[test]
    fn totales_se_actualizan_tras_eliminar_servicio() {
        let connection = database::open_in_memory().unwrap();
        let first =
            create_service(&connection, &REPOSITORY, ars_input("A", "10000,00", 4)).unwrap();
        create_service(&connection, &REPOSITORY, ars_input("B", "6000,00", 2)).unwrap();

        let before = get_service_totals(&connection, &REPOSITORY).unwrap();
        let outcome = delete_service(
            &connection,
            &REPOSITORY,
            first.id(),
            DeleteConfirmation::Confirmed,
        )
        .unwrap();
        let after = get_service_totals(&connection, &REPOSITORY).unwrap();

        assert!(matches!(outcome, DeleteServiceOutcome::Deleted));
        assert_eq!(before.ars().total_cost().minor_units(), 1_600_000);
        assert_eq!(before.ars().user_share().numerator_minor(), 550_000);
        assert_eq!(after.ars().total_cost().minor_units(), 600_000);
        assert_eq!(after.ars().user_share().numerator_minor(), 300_000);
        assert_eq!(after.ars().user_share().denominator(), 1);
    }

    #[test]
    fn totales_se_actualizan_tras_editar_costo_y_cantidad() {
        let connection = database::open_in_memory().unwrap();
        let created = create_service(
            &connection,
            &REPOSITORY,
            ars_input("Spotify", "10000,00", 4),
        )
        .unwrap();

        let before = get_service_totals(&connection, &REPOSITORY).unwrap();
        update_service(
            &connection,
            &REPOSITORY,
            created.id(),
            update_input("Spotify", "12000,00", 4),
        )
        .unwrap();
        let after_cost = get_service_totals(&connection, &REPOSITORY).unwrap();
        update_service(
            &connection,
            &REPOSITORY,
            created.id(),
            update_input("Spotify", "12000,00", 3),
        )
        .unwrap();
        let after_people = get_service_totals(&connection, &REPOSITORY).unwrap();

        assert_eq!(before.ars().total_cost().minor_units(), 1_000_000);
        assert_eq!(before.ars().user_share().numerator_minor(), 250_000);
        assert_eq!(after_cost.ars().total_cost().minor_units(), 1_200_000);
        assert_eq!(after_cost.ars().user_share().numerator_minor(), 300_000);
        assert_eq!(after_people.ars().total_cost().minor_units(), 1_200_000);
        assert_eq!(after_people.ars().user_share().numerator_minor(), 400_000);
        assert_eq!(after_people.ars().user_share().denominator(), 1);
    }

    #[test]
    fn totales_conservan_fracciones_exactas() {
        let connection = database::open_in_memory().unwrap();
        create_service(&connection, &REPOSITORY, ars_input("A", "100,00", 3)).unwrap();
        create_service(&connection, &REPOSITORY, ars_input("B", "100,00", 3)).unwrap();

        let totals = get_service_totals(&connection, &REPOSITORY).unwrap();

        assert_eq!(totals.ars().total_cost().minor_units(), 20_000);
        assert_eq!(totals.ars().user_share().numerator_minor(), 20_000);
        assert_eq!(totals.ars().user_share().denominator(), 3);
    }

    #[test]
    fn totales_de_servicio_sin_compartir_son_el_costo_completo() {
        let connection = database::open_in_memory().unwrap();
        create_service(&connection, &REPOSITORY, ars_input("Luz", "8000,00", 1)).unwrap();

        let totals = get_service_totals(&connection, &REPOSITORY).unwrap();

        assert_eq!(totals.ars().total_cost().minor_units(), 800_000);
        assert_eq!(totals.ars().user_share().numerator_minor(), 800_000);
        assert_eq!(totals.ars().user_share().denominator(), 1);
    }

    #[test]
    fn totales_propagan_error_del_repositorio() {
        let connection = database::open_in_memory().unwrap();

        let error = get_service_totals(&connection, &FailingRepository).unwrap_err();

        assert!(matches!(
            error,
            DomainError::Persistence(message) if message == "repositorio no disponible"
        ));
    }

    #[test]
    fn totales_propagan_desbordamiento_del_calculo() {
        let connection = database::open_in_memory().unwrap();
        create_service(
            &connection,
            &REPOSITORY,
            ars_input("Max", "92233720368547758.07", 1),
        )
        .unwrap();
        create_service(&connection, &REPOSITORY, ars_input("Uno", "0,01", 1)).unwrap();

        let error = get_service_totals(&connection, &REPOSITORY).unwrap_err();

        assert!(matches!(
            error,
            DomainError::ArithmeticOverflow {
                field: "amount",
                ..
            }
        ));
    }

    #[test]
    fn totales_incluyen_servicio_reactivado() {
        let connection = database::open_in_memory().unwrap();
        let created =
            create_service(&connection, &REPOSITORY, ars_input("Luz", "5000,00", 2)).unwrap();
        freeze_service(&connection, &REPOSITORY, created.id()).unwrap();

        let frozen_totals = get_service_totals(&connection, &REPOSITORY).unwrap();
        let outcome = reactivate_service(&connection, &REPOSITORY, created.id()).unwrap();
        let reactivated_totals = get_service_totals(&connection, &REPOSITORY).unwrap();

        assert!(matches!(outcome, ReactivateServiceOutcome::Reactivated(_)));
        assert_eq!(frozen_totals.ars().total_cost().minor_units(), 0);
        assert_eq!(frozen_totals.ars().user_share().numerator_minor(), 0);
        assert_eq!(reactivated_totals.ars().total_cost().minor_units(), 500_000);
        assert_eq!(
            reactivated_totals.ars().user_share().numerator_minor(),
            250_000
        );
        assert_eq!(reactivated_totals.ars().user_share().denominator(), 1);
    }
}
