use rusqlite::Connection;
use tauri::State;

use crate::AppState;
use crate::application::error::AppError;
use crate::application::service_dtos::{
    CreateServiceInputDto, DeleteServiceOutcomeDto, DeleteServiceResultDto,
    ServiceMutationOutcomeDto, ServiceMutationResultDto, ServiceRecordDto, ServiceTotalsDto,
    UpdateServiceInputDto,
};
use crate::application::service_repository::{ServiceId, ServiceRepository};
use crate::application::service_use_cases::{
    self, CreateServiceInput, DeleteConfirmation, UpdateServiceInput,
};
use crate::domain::error::DomainError;
use crate::infrastructure::database;
use crate::infrastructure::service_repository::SqliteServiceRepository;

const REPOSITORY: SqliteServiceRepository = SqliteServiceRepository;

fn map_service_error(error: DomainError) -> AppError {
    match error {
        DomainError::NotFound => {
            AppError::new("not_found", "No se encontró el servicio solicitado.").with_field("id")
        }
        other => AppError::from(other),
    }
}

fn create_service_with<R>(
    connection: &Connection,
    repository: &R,
    dto: CreateServiceInputDto,
) -> Result<ServiceRecordDto, AppError>
where
    R: ServiceRepository + ?Sized,
{
    let input = CreateServiceInput::try_from(dto).map_err(map_service_error)?;
    let record = service_use_cases::create_service(connection, repository, input)
        .map_err(map_service_error)?;
    ServiceRecordDto::try_from(&record).map_err(map_service_error)
}

fn update_service_with<R>(
    connection: &Connection,
    repository: &R,
    id: ServiceId,
    dto: UpdateServiceInputDto,
) -> Result<ServiceRecordDto, AppError>
where
    R: ServiceRepository + ?Sized,
{
    let input = UpdateServiceInput::try_from(dto).map_err(map_service_error)?;
    let record = service_use_cases::update_service(connection, repository, id, input)
        .map_err(map_service_error)?;
    ServiceRecordDto::try_from(&record).map_err(map_service_error)
}

fn delete_service_with<R>(
    connection: &Connection,
    repository: &R,
    id: ServiceId,
    confirmed: bool,
) -> Result<DeleteServiceResultDto, AppError>
where
    R: ServiceRepository + ?Sized,
{
    let confirmation = if confirmed {
        DeleteConfirmation::Confirmed
    } else {
        DeleteConfirmation::Cancelled
    };
    let outcome = service_use_cases::delete_service(connection, repository, id, confirmation)
        .map_err(map_service_error)?;

    Ok(DeleteServiceResultDto {
        outcome: DeleteServiceOutcomeDto::from(outcome),
        id: id.to_string(),
    })
}

fn freeze_service_with<R>(
    connection: &Connection,
    repository: &R,
    id: ServiceId,
) -> Result<ServiceMutationResultDto, AppError>
where
    R: ServiceRepository + ?Sized,
{
    let outcome =
        service_use_cases::freeze_service(connection, repository, id).map_err(map_service_error)?;

    Ok(ServiceMutationResultDto {
        outcome: ServiceMutationOutcomeDto::from(&outcome),
    })
}

fn reactivate_service_with<R>(
    connection: &Connection,
    repository: &R,
    id: ServiceId,
) -> Result<ServiceMutationResultDto, AppError>
where
    R: ServiceRepository + ?Sized,
{
    let outcome = service_use_cases::reactivate_service(connection, repository, id)
        .map_err(map_service_error)?;

    Ok(ServiceMutationResultDto {
        outcome: ServiceMutationOutcomeDto::from(&outcome),
    })
}

fn list_frozen_services_with<R>(
    connection: &Connection,
    repository: &R,
) -> Result<Vec<ServiceRecordDto>, AppError>
where
    R: ServiceRepository + ?Sized,
{
    let records = service_use_cases::list_frozen_services(connection, repository)
        .map_err(map_service_error)?;
    records
        .iter()
        .map(ServiceRecordDto::try_from)
        .collect::<Result<Vec<_>, DomainError>>()
        .map_err(map_service_error)
}

fn list_active_services_with<R>(
    connection: &Connection,
    repository: &R,
) -> Result<Vec<ServiceRecordDto>, AppError>
where
    R: ServiceRepository + ?Sized,
{
    let records = service_use_cases::list_active_services(connection, repository)
        .map_err(map_service_error)?;
    records
        .iter()
        .map(ServiceRecordDto::try_from)
        .collect::<Result<Vec<_>, DomainError>>()
        .map_err(map_service_error)
}

fn get_service_totals_with<R>(
    connection: &Connection,
    repository: &R,
) -> Result<ServiceTotalsDto, AppError>
where
    R: ServiceRepository + ?Sized,
{
    let totals =
        service_use_cases::get_service_totals(connection, repository).map_err(map_service_error)?;
    ServiceTotalsDto::try_from(&totals).map_err(map_service_error)
}

#[tauri::command]
pub fn create_service(
    state: State<'_, AppState>,
    input: CreateServiceInputDto,
) -> Result<ServiceRecordDto, AppError> {
    let connection = database::open(&state.database_path)?;
    create_service_with(&connection, &REPOSITORY, input)
}

#[tauri::command]
pub fn update_service(
    state: State<'_, AppState>,
    id: String,
    input: UpdateServiceInputDto,
) -> Result<ServiceRecordDto, AppError> {
    let id = parse_id(&id)?;
    let connection = database::open(&state.database_path)?;
    update_service_with(&connection, &REPOSITORY, id, input)
}

#[tauri::command]
pub fn delete_service(
    state: State<'_, AppState>,
    id: String,
    confirmed: bool,
) -> Result<DeleteServiceResultDto, AppError> {
    let id = parse_id(&id)?;
    let connection = database::open(&state.database_path)?;
    delete_service_with(&connection, &REPOSITORY, id, confirmed)
}

#[tauri::command]
pub fn freeze_service(
    state: State<'_, AppState>,
    id: String,
) -> Result<ServiceMutationResultDto, AppError> {
    let id = parse_id(&id)?;
    let connection = database::open(&state.database_path)?;
    freeze_service_with(&connection, &REPOSITORY, id)
}

#[tauri::command]
pub fn reactivate_service(
    state: State<'_, AppState>,
    id: String,
) -> Result<ServiceMutationResultDto, AppError> {
    let id = parse_id(&id)?;
    let connection = database::open(&state.database_path)?;
    reactivate_service_with(&connection, &REPOSITORY, id)
}

#[tauri::command]
pub fn list_frozen_services(state: State<'_, AppState>) -> Result<Vec<ServiceRecordDto>, AppError> {
    let connection = database::open(&state.database_path)?;
    list_frozen_services_with(&connection, &REPOSITORY)
}

#[tauri::command]
pub fn list_active_services(state: State<'_, AppState>) -> Result<Vec<ServiceRecordDto>, AppError> {
    let connection = database::open(&state.database_path)?;
    list_active_services_with(&connection, &REPOSITORY)
}

#[tauri::command]
pub fn get_service_totals(state: State<'_, AppState>) -> Result<ServiceTotalsDto, AppError> {
    let connection = database::open(&state.database_path)?;
    get_service_totals_with(&connection, &REPOSITORY)
}

fn parse_id(value: &str) -> Result<ServiceId, AppError> {
    crate::application::service_dtos::parse_service_id(value).map_err(map_service_error)
}

#[cfg(test)]
mod tests {
    use std::path::{Path, PathBuf};

    use serde_json::json;
    use tempfile::TempDir;

    use super::*;
    use crate::application::service_dtos::ServiceStatusDto;
    use crate::application::service_repository::ServiceRecord;
    use crate::domain::service::{Service, ServiceStatus};

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

    fn database_path(directory: &TempDir) -> PathBuf {
        database::resolve_path(directory.path())
    }

    fn create_input(
        name: &str,
        total_cost: &str,
        shared_people_count: u32,
    ) -> CreateServiceInputDto {
        CreateServiceInputDto {
            name: name.to_owned(),
            total_cost: total_cost.to_owned(),
            currency_code: "ARS".to_owned(),
            shared_people_count: json!(shared_people_count),
            notes: None,
        }
    }

    fn update_input(
        name: &str,
        total_cost: &str,
        shared_people_count: u32,
    ) -> UpdateServiceInputDto {
        UpdateServiceInputDto {
            name: name.to_owned(),
            total_cost: total_cost.to_owned(),
            shared_people_count: json!(shared_people_count),
            notes: None,
        }
    }

    fn create_at(path: &Path, name: &str, cost: &str, people: u32) -> ServiceRecordDto {
        create_service_at(path, create_input(name, cost, people)).unwrap()
    }

    fn load_record(path: &Path, id: &str) -> ServiceRecord {
        let connection = database::open(path).unwrap();
        let id = ServiceId::parse(id).unwrap();
        REPOSITORY.get(&connection, id).unwrap().unwrap()
    }

    fn create_service_at(
        path: &Path,
        dto: CreateServiceInputDto,
    ) -> Result<ServiceRecordDto, AppError> {
        let connection = database::open(path)?;
        create_service_with(&connection, &REPOSITORY, dto)
    }

    fn update_service_at(
        path: &Path,
        id: &str,
        dto: UpdateServiceInputDto,
    ) -> Result<ServiceRecordDto, AppError> {
        let id = parse_id(id)?;
        let connection = database::open(path)?;
        update_service_with(&connection, &REPOSITORY, id, dto)
    }

    fn delete_service_at(
        path: &Path,
        id: &str,
        confirmed: bool,
    ) -> Result<DeleteServiceResultDto, AppError> {
        let id = parse_id(id)?;
        let connection = database::open(path)?;
        delete_service_with(&connection, &REPOSITORY, id, confirmed)
    }

    fn freeze_service_at(path: &Path, id: &str) -> Result<ServiceMutationResultDto, AppError> {
        let id = parse_id(id)?;
        let connection = database::open(path)?;
        freeze_service_with(&connection, &REPOSITORY, id)
    }

    fn reactivate_service_at(path: &Path, id: &str) -> Result<ServiceMutationResultDto, AppError> {
        let id = parse_id(id)?;
        let connection = database::open(path)?;
        reactivate_service_with(&connection, &REPOSITORY, id)
    }

    fn list_frozen_at(path: &Path) -> Result<Vec<ServiceRecordDto>, AppError> {
        let connection = database::open(path)?;
        list_frozen_services_with(&connection, &REPOSITORY)
    }

    fn list_active_at(path: &Path) -> Result<Vec<ServiceRecordDto>, AppError> {
        let connection = database::open(path)?;
        list_active_services_with(&connection, &REPOSITORY)
    }

    fn totals_at(path: &Path) -> Result<ServiceTotalsDto, AppError> {
        let connection = database::open(path)?;
        get_service_totals_with(&connection, &REPOSITORY)
    }

    #[test]
    fn crear_servicio_devuelve_dto_exacto() {
        let directory = tempfile::tempdir().unwrap();
        let path = database_path(&directory);

        let record = create_at(&path, "Luz", "10000,00", 4);

        assert_eq!(record.name, "Luz");
        assert_eq!(record.total_cost.minor_units, "1000000");
        assert_eq!(record.total_cost.currency_code, "ARS");
        assert_eq!(record.my_share.numerator_minor, "250000");
        assert_eq!(record.my_share.denominator, "1");
        assert_eq!(record.status, ServiceStatusDto::Active);
    }

    #[test]
    fn editar_servicio_recalcula_y_conserva_moneda() {
        let directory = tempfile::tempdir().unwrap();
        let path = database_path(&directory);
        let record = create_at(&path, "Luz", "10000,00", 4);

        let updated =
            update_service_at(&path, &record.id, update_input("Luz", "12000,00", 3)).unwrap();

        assert_eq!(updated.total_cost.minor_units, "1200000");
        assert_eq!(updated.total_cost.currency_code, "ARS");
        assert_eq!(updated.my_share.numerator_minor, "400000");
        assert_eq!(updated.my_share.denominator, "1");
    }

    #[test]
    fn eliminar_confirmado_devuelve_deleted_y_quita_el_servicio() {
        let directory = tempfile::tempdir().unwrap();
        let path = database_path(&directory);
        let record = create_at(&path, "Luz", "100,00", 1);

        let result = delete_service_at(&path, &record.id, true).unwrap();

        assert_eq!(result.outcome, DeleteServiceOutcomeDto::Deleted);
        assert_eq!(result.id, record.id);
        assert!(list_active_at(&path).unwrap().is_empty());
    }

    #[test]
    fn eliminar_cancelado_devuelve_cancelled_y_no_borra() {
        let directory = tempfile::tempdir().unwrap();
        let path = database_path(&directory);
        let record = create_at(&path, "Luz", "100,00", 1);

        let result = delete_service_at(&path, &record.id, false).unwrap();

        assert_eq!(result.outcome, DeleteServiceOutcomeDto::Cancelled);
        assert_eq!(list_active_at(&path).unwrap().len(), 1);
        assert!(load_record(&path, &record.id).service().status() == ServiceStatus::Active);
    }

    #[test]
    fn congelar_devuelve_frozen_y_mueve_a_congelados() {
        let directory = tempfile::tempdir().unwrap();
        let path = database_path(&directory);
        let record = create_at(&path, "Luz", "100,00", 1);

        let result = freeze_service_at(&path, &record.id).unwrap();

        assert_eq!(result.outcome, ServiceMutationOutcomeDto::Frozen);
        assert!(list_active_at(&path).unwrap().is_empty());
        assert_eq!(list_frozen_at(&path).unwrap()[0].id, record.id);
    }

    #[test]
    fn congelar_ya_congelado_es_idempotente() {
        let directory = tempfile::tempdir().unwrap();
        let path = database_path(&directory);
        let record = create_at(&path, "Luz", "100,00", 1);
        freeze_service_at(&path, &record.id).unwrap();
        let before = load_record(&path, &record.id);

        let result = freeze_service_at(&path, &record.id).unwrap();
        let after = load_record(&path, &record.id);

        assert_eq!(result.outcome, ServiceMutationOutcomeDto::AlreadyFrozen);
        assert_eq!(before.updated_at(), after.updated_at());
        assert_eq!(before.service().status(), ServiceStatus::Frozen);
    }

    #[test]
    fn reactivar_devuelve_reactivated_y_devuelve_a_activos() {
        let directory = tempfile::tempdir().unwrap();
        let path = database_path(&directory);
        let record = create_at(&path, "Luz", "100,00", 1);
        freeze_service_at(&path, &record.id).unwrap();

        let result = reactivate_service_at(&path, &record.id).unwrap();

        assert_eq!(result.outcome, ServiceMutationOutcomeDto::Reactivated);
        assert_eq!(list_active_at(&path).unwrap()[0].id, record.id);
        assert!(list_frozen_at(&path).unwrap().is_empty());
    }

    #[test]
    fn reactivar_ya_activo_es_idempotente() {
        let directory = tempfile::tempdir().unwrap();
        let path = database_path(&directory);
        let record = create_at(&path, "Luz", "100,00", 1);
        let before = load_record(&path, &record.id);

        let result = reactivate_service_at(&path, &record.id).unwrap();
        let after = load_record(&path, &record.id);

        assert_eq!(result.outcome, ServiceMutationOutcomeDto::AlreadyActive);
        assert_eq!(before.updated_at(), after.updated_at());
    }

    #[test]
    fn listar_congelados_devuelve_solo_congelados_en_orden() {
        let directory = tempfile::tempdir().unwrap();
        let path = database_path(&directory);
        let active = create_at(&path, "Activo", "100,00", 1);
        let first = create_at(&path, "Congelado 1", "200,00", 1);
        let second = create_at(&path, "Congelado 2", "300,00", 1);
        freeze_service_at(&path, &first.id).unwrap();
        freeze_service_at(&path, &second.id).unwrap();

        let records = list_frozen_at(&path).unwrap();

        assert_eq!(
            records
                .iter()
                .map(|record| record.id.clone())
                .collect::<Vec<_>>(),
            vec![first.id, second.id]
        );
        assert!(!records.iter().any(|record| record.id == active.id));
    }

    #[test]
    fn listar_activos_devuelve_solo_activos_en_orden() {
        let directory = tempfile::tempdir().unwrap();
        let path = database_path(&directory);
        let first = create_at(&path, "Activo 1", "100,00", 1);
        let second = create_at(&path, "Activo 2", "200,00", 1);
        let frozen = create_at(&path, "Congelado", "300,00", 1);
        freeze_service_at(&path, &frozen.id).unwrap();

        let records = list_active_at(&path).unwrap();

        assert_eq!(
            records
                .iter()
                .map(|record| record.id.clone())
                .collect::<Vec<_>>(),
            vec![first.id, second.id]
        );
    }

    #[test]
    fn consultar_totales_separa_ars_y_usd() {
        let directory = tempfile::tempdir().unwrap();
        let path = database_path(&directory);
        create_at(&path, "Luz", "10000,00", 4);
        let mut usd_input = create_input("Netflix", "20,00", 2);
        usd_input.currency_code = "USD".to_owned();
        create_service_at(&path, usd_input).unwrap();

        let totals = totals_at(&path).unwrap();

        assert_eq!(totals.ars.total_cost.minor_units, "1000000");
        assert_eq!(totals.ars.total_cost.currency_code, "ARS");
        assert_eq!(totals.ars.user_share.numerator_minor, "250000");
        assert_eq!(totals.usd.total_cost.minor_units, "2000");
        assert_eq!(totals.usd.total_cost.currency_code, "USD");
        assert_eq!(totals.usd.user_share.numerator_minor, "1000");
    }

    #[test]
    fn consultar_totales_sin_servicios_devuelve_cero() {
        let directory = tempfile::tempdir().unwrap();
        let path = database_path(&directory);

        let totals = totals_at(&path).unwrap();

        assert_eq!(totals.ars.total_cost.minor_units, "0");
        assert_eq!(totals.ars.user_share.numerator_minor, "0");
        assert_eq!(totals.ars.user_share.denominator, "1");
        assert_eq!(totals.usd.total_cost.minor_units, "0");
        assert_eq!(totals.usd.user_share.numerator_minor, "0");
        assert_eq!(totals.usd.user_share.denominator, "1");
    }

    #[test]
    fn consultar_totales_excluye_congelados_y_conserva_fracciones() {
        let directory = tempfile::tempdir().unwrap();
        let path = database_path(&directory);
        create_at(&path, "A", "100,00", 3);
        create_at(&path, "B", "100,00", 3);
        let frozen = create_at(&path, "Congelado", "5000,00", 2);
        freeze_service_at(&path, &frozen.id).unwrap();

        let totals = totals_at(&path).unwrap();

        assert_eq!(totals.ars.total_cost.minor_units, "20000");
        assert_eq!(totals.ars.user_share.numerator_minor, "20000");
        assert_eq!(totals.ars.user_share.denominator, "3");
    }

    #[test]
    fn identificador_malformado_devuelve_validacion_con_campo_id() {
        let directory = tempfile::tempdir().unwrap();
        let path = database_path(&directory);

        let update_error =
            update_service_at(&path, "no-uuid", update_input("Luz", "100,00", 1)).unwrap_err();
        let delete_error = delete_service_at(&path, "no-uuid", true).unwrap_err();
        let freeze_error = freeze_service_at(&path, "no-uuid").unwrap_err();
        let reactivate_error = reactivate_service_at(&path, "no-uuid").unwrap_err();

        for error in [update_error, delete_error, freeze_error, reactivate_error] {
            assert_eq!(error.code, "validation");
            assert_eq!(error.field.as_deref(), Some("id"));
        }
    }

    #[test]
    fn identificador_inexistente_devuelve_not_found_con_campo_id() {
        let directory = tempfile::tempdir().unwrap();
        let path = database_path(&directory);
        let id = "00000000-0000-4000-8000-000000000099";

        let update_error =
            update_service_at(&path, id, update_input("Luz", "100,00", 1)).unwrap_err();
        let delete_error = delete_service_at(&path, id, true).unwrap_err();
        let freeze_error = freeze_service_at(&path, id).unwrap_err();
        let reactivate_error = reactivate_service_at(&path, id).unwrap_err();

        for error in [update_error, delete_error, freeze_error, reactivate_error] {
            assert_eq!(error.code, "not_found");
            assert_eq!(error.field.as_deref(), Some("id"));
        }
    }

    #[test]
    fn errores_de_validacion_conservan_campo_en_camel_case() {
        let directory = tempfile::tempdir().unwrap();
        let path = database_path(&directory);

        let name_error = create_service_at(&path, create_input("   ", "100,00", 1)).unwrap_err();
        let cost_error = create_service_at(&path, create_input("Luz", "100,999", 1)).unwrap_err();
        let people_error = create_service_at(&path, create_input("Luz", "100,00", 0)).unwrap_err();
        let negative_people_error = create_service_at(
            &path,
            CreateServiceInputDto {
                shared_people_count: json!(-1),
                ..create_input("Luz", "100,00", 1)
            },
        )
        .unwrap_err();
        let non_integer_people_error = update_service_at(
            &path,
            "00000000-0000-4000-8000-000000000001",
            UpdateServiceInputDto {
                shared_people_count: json!(1.5),
                ..update_input("Luz", "100,00", 1)
            },
        )
        .unwrap_err();
        let currency_input = CreateServiceInputDto {
            currency_code: "EUR".to_owned(),
            ..create_input("Luz", "100,00", 1)
        };
        let currency_error = create_service_at(&path, currency_input).unwrap_err();

        assert_eq!(name_error.code, "validation");
        assert_eq!(name_error.field.as_deref(), Some("name"));
        assert_eq!(cost_error.field.as_deref(), Some("totalCost"));
        assert_eq!(people_error.field.as_deref(), Some("sharedPeopleCount"));
        assert_eq!(
            negative_people_error.field.as_deref(),
            Some("sharedPeopleCount")
        );
        assert_eq!(
            non_integer_people_error.field.as_deref(),
            Some("sharedPeopleCount")
        );
        assert_eq!(currency_error.field.as_deref(), Some("currencyCode"));
    }

    #[test]
    fn errores_de_persistencia_de_todos_los_comandos_no_reciben_campo_artificial() {
        let connection = database::open_in_memory().unwrap();
        let id = ServiceId::parse("00000000-0000-4000-8000-000000000001").unwrap();

        let errors = [
            create_service_with(
                &connection,
                &FailingRepository,
                create_input("Luz", "100,00", 1),
            )
            .unwrap_err(),
            update_service_with(
                &connection,
                &FailingRepository,
                id,
                update_input("Luz", "100,00", 1),
            )
            .unwrap_err(),
            delete_service_with(&connection, &FailingRepository, id, true).unwrap_err(),
            freeze_service_with(&connection, &FailingRepository, id).unwrap_err(),
            reactivate_service_with(&connection, &FailingRepository, id).unwrap_err(),
            list_frozen_services_with(&connection, &FailingRepository).unwrap_err(),
            list_active_services_with(&connection, &FailingRepository).unwrap_err(),
            get_service_totals_with(&connection, &FailingRepository).unwrap_err(),
        ];

        for error in errors {
            assert_eq!(error.code, "persistence");
            assert!(error.field.is_none());
            assert_eq!(
                serde_json::to_value(&error).unwrap(),
                json!({
                    "code": "persistence",
                    "message": "No se pudo acceder a los datos: repositorio no disponible"
                })
            );
        }
    }
}
