use rusqlite::Connection;
use tauri::State;

use crate::AppState;
use crate::application::error::AppError;
use crate::application::investment_dtos::{
    CreateInvestmentInputDto, InvestmentSummaryDto, UpdateInvestmentInputDto, parse_investment_id,
};
use crate::application::investment_queries;
use crate::application::investment_repository::{InvestmentId, InvestmentRepository};
use crate::application::investment_use_cases::{CreateInvestmentInput, UpdateInvestmentInput};
use crate::application::service_repository::ServiceRepository;
use crate::domain::error::DomainError;
use crate::infrastructure::database;
use crate::infrastructure::investment_repository::SqliteInvestmentRepository;
use crate::infrastructure::service_repository::SqliteServiceRepository;

const INVESTMENT_REPOSITORY: SqliteInvestmentRepository = SqliteInvestmentRepository;
const SERVICE_REPOSITORY: SqliteServiceRepository = SqliteServiceRepository;

fn map_investment_error(error: DomainError) -> AppError {
    match error {
        DomainError::NotFound => {
            AppError::new("not_found", "No se encontró la inversión solicitada.").with_field("id")
        }
        other => AppError::from(other),
    }
}

fn create_investment_with<S, I>(
    connection: &Connection,
    service_repository: &S,
    investment_repository: &I,
    dto: CreateInvestmentInputDto,
) -> Result<InvestmentSummaryDto, AppError>
where
    S: ServiceRepository + ?Sized,
    I: InvestmentRepository + ?Sized,
{
    let input = CreateInvestmentInput::try_from(dto).map_err(map_investment_error)?;
    let details = investment_queries::create_investment_details(
        connection,
        service_repository,
        investment_repository,
        input,
    )
    .map_err(map_investment_error)?;

    InvestmentSummaryDto::try_from(&details).map_err(map_investment_error)
}

fn update_investment_with<S, I>(
    connection: &Connection,
    service_repository: &S,
    investment_repository: &I,
    id: InvestmentId,
    dto: UpdateInvestmentInputDto,
) -> Result<InvestmentSummaryDto, AppError>
where
    S: ServiceRepository + ?Sized,
    I: InvestmentRepository + ?Sized,
{
    let input = UpdateInvestmentInput::try_from(dto).map_err(map_investment_error)?;
    let details = investment_queries::update_investment_details(
        connection,
        service_repository,
        investment_repository,
        id,
        input,
    )
    .map_err(map_investment_error)?;

    InvestmentSummaryDto::try_from(&details).map_err(map_investment_error)
}

fn list_investments_by_month_with<S, I>(
    connection: &Connection,
    service_repository: &S,
    investment_repository: &I,
    month: String,
) -> Result<Vec<InvestmentSummaryDto>, AppError>
where
    S: ServiceRepository + ?Sized,
    I: InvestmentRepository + ?Sized,
{
    let details = investment_queries::list_investment_details_by_month(
        connection,
        service_repository,
        investment_repository,
        month.as_str(),
    )
    .map_err(map_investment_error)?;

    details
        .iter()
        .map(InvestmentSummaryDto::try_from)
        .collect::<Result<Vec<_>, DomainError>>()
        .map_err(map_investment_error)
}

#[tauri::command]
pub fn create_investment(
    state: State<'_, AppState>,
    input: CreateInvestmentInputDto,
) -> Result<InvestmentSummaryDto, AppError> {
    let connection = database::open(&state.database_path)?;
    create_investment_with(
        &connection,
        &SERVICE_REPOSITORY,
        &INVESTMENT_REPOSITORY,
        input,
    )
}

#[tauri::command]
pub fn update_investment(
    state: State<'_, AppState>,
    id: String,
    input: UpdateInvestmentInputDto,
) -> Result<InvestmentSummaryDto, AppError> {
    let id = parse_id(&id)?;
    let connection = database::open(&state.database_path)?;
    update_investment_with(
        &connection,
        &SERVICE_REPOSITORY,
        &INVESTMENT_REPOSITORY,
        id,
        input,
    )
}

#[tauri::command]
pub fn list_investments_by_month(
    state: State<'_, AppState>,
    month: String,
) -> Result<Vec<InvestmentSummaryDto>, AppError> {
    let connection = database::open(&state.database_path)?;
    list_investments_by_month_with(
        &connection,
        &SERVICE_REPOSITORY,
        &INVESTMENT_REPOSITORY,
        month,
    )
}

fn parse_id(value: &str) -> Result<InvestmentId, AppError> {
    parse_investment_id(value).map_err(map_investment_error)
}

#[cfg(test)]
mod tests {
    use std::path::{Path, PathBuf};

    use tempfile::TempDir;

    use super::*;
    use crate::application::investment_dtos::{
        CalculationReasonDto, CalculationStatusDto, CoverageOutcomeDto,
    };
    use crate::application::investment_repository::InvestmentRecord;
    use crate::application::service_repository::{ServiceId, ServiceRecord};
    use crate::application::service_use_cases::{
        self, CreateServiceInput, DeleteConfirmation, DeleteServiceOutcome, FreezeServiceOutcome,
        ReactivateServiceOutcome, UpdateServiceInput,
    };
    use crate::domain::investment::{Investment, Month};
    use crate::domain::money::Currency;
    use crate::domain::service::{Service, ServiceStatus};

    struct FailingInvestmentRepository;

    impl FailingInvestmentRepository {
        fn error() -> DomainError {
            DomainError::persistence("repositorio de inversiones no disponible")
        }
    }

    impl InvestmentRepository for FailingInvestmentRepository {
        fn insert(
            &self,
            _connection: &Connection,
            _investment: &Investment,
        ) -> Result<InvestmentRecord, DomainError> {
            Err(Self::error())
        }

        fn get(
            &self,
            _connection: &Connection,
            _id: InvestmentId,
        ) -> Result<InvestmentRecord, DomainError> {
            Err(Self::error())
        }

        fn update(
            &self,
            _connection: &Connection,
            _id: InvestmentId,
            _investment: &Investment,
        ) -> Result<InvestmentRecord, DomainError> {
            Err(Self::error())
        }

        fn list_by_month(
            &self,
            _connection: &Connection,
            _month: Month,
        ) -> Result<Vec<InvestmentRecord>, DomainError> {
            Err(Self::error())
        }
    }

    struct FailingServiceRepository;

    impl FailingServiceRepository {
        fn error() -> DomainError {
            DomainError::persistence("repositorio de servicios no disponible")
        }
    }

    impl ServiceRepository for FailingServiceRepository {
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
        month: &str,
        currency_code: &str,
        capital: &str,
        tna: &str,
        term_days: &str,
        taxes: &str,
    ) -> CreateInvestmentInputDto {
        CreateInvestmentInputDto {
            month: month.to_owned(),
            currency_code: currency_code.to_owned(),
            capital: capital.to_owned(),
            tna: tna.to_owned(),
            term_days: term_days.to_owned(),
            taxes: taxes.to_owned(),
        }
    }

    fn update_input(
        month: &str,
        capital: &str,
        tna: &str,
        term_days: &str,
        taxes: &str,
    ) -> UpdateInvestmentInputDto {
        UpdateInvestmentInputDto {
            month: month.to_owned(),
            capital: capital.to_owned(),
            tna: tna.to_owned(),
            term_days: term_days.to_owned(),
            taxes: taxes.to_owned(),
        }
    }

    fn create_at(
        path: &Path,
        dto: CreateInvestmentInputDto,
    ) -> Result<InvestmentSummaryDto, AppError> {
        let connection = database::open(path)?;
        create_investment_with(
            &connection,
            &SERVICE_REPOSITORY,
            &INVESTMENT_REPOSITORY,
            dto,
        )
    }

    fn update_at(
        path: &Path,
        id: &str,
        dto: UpdateInvestmentInputDto,
    ) -> Result<InvestmentSummaryDto, AppError> {
        let id = parse_id(id)?;
        let connection = database::open(path)?;
        update_investment_with(
            &connection,
            &SERVICE_REPOSITORY,
            &INVESTMENT_REPOSITORY,
            id,
            dto,
        )
    }

    fn list_at(path: &Path, month: &str) -> Result<Vec<InvestmentSummaryDto>, AppError> {
        let connection = database::open(path)?;
        list_investments_by_month_with(
            &connection,
            &SERVICE_REPOSITORY,
            &INVESTMENT_REPOSITORY,
            month.to_owned(),
        )
    }

    fn create_service_at(
        path: &Path,
        name: &str,
        currency: Currency,
        total_cost: &str,
        shared_people_count: u32,
    ) -> ServiceRecord {
        let connection = database::open(path).unwrap();
        let input = CreateServiceInput {
            name: name.to_owned(),
            total_cost: total_cost.to_owned(),
            currency,
            shared_people_count,
            notes: None,
        };
        service_use_cases::create_service(&connection, &SERVICE_REPOSITORY, input).unwrap()
    }

    fn update_service_at(
        path: &Path,
        id: ServiceId,
        name: &str,
        total_cost: &str,
        shared_people_count: u32,
    ) {
        let connection = database::open(path).unwrap();
        let input = UpdateServiceInput {
            name: name.to_owned(),
            total_cost: total_cost.to_owned(),
            shared_people_count,
            notes: None,
        };
        service_use_cases::update_service(&connection, &SERVICE_REPOSITORY, id, input).unwrap();
    }

    fn delete_service_at(path: &Path, id: ServiceId) {
        let connection = database::open(path).unwrap();
        let outcome = service_use_cases::delete_service(
            &connection,
            &SERVICE_REPOSITORY,
            id,
            DeleteConfirmation::Confirmed,
        )
        .unwrap();
        assert!(matches!(outcome, DeleteServiceOutcome::Deleted));
    }

    fn freeze_service_at(path: &Path, id: ServiceId) {
        let connection = database::open(path).unwrap();
        let outcome =
            service_use_cases::freeze_service(&connection, &SERVICE_REPOSITORY, id).unwrap();
        assert!(matches!(outcome, FreezeServiceOutcome::Frozen(_)));
    }

    fn reactivate_service_at(path: &Path, id: ServiceId) {
        let connection = database::open(path).unwrap();
        let outcome =
            service_use_cases::reactivate_service(&connection, &SERVICE_REPOSITORY, id).unwrap();
        assert!(matches!(outcome, ReactivateServiceOutcome::Reactivated(_)));
    }

    #[test]
    fn crear_inversion_devuelve_origen_y_derivados_con_estados() {
        let directory = tempfile::tempdir().unwrap();
        let path = database_path(&directory);
        create_service_at(&path, "Luz", Currency::Ars, "100,00", 1);

        let summary = create_at(
            &path,
            create_input("2026-03", "ARS", "365,00", "100", "365", "65,00"),
        )
        .unwrap();

        assert_eq!(summary.record.month, "2026-03");
        assert_eq!(summary.record.currency_code, "ARS");
        assert_eq!(summary.record.capital.minor_units, "36500");
        assert_eq!(summary.record.tna, "100,00");
        assert_eq!(summary.record.term_days, 365);
        assert_eq!(summary.record.taxes.minor_units, "6500");
        assert_eq!(summary.derived.monthly_expenses.numerator_minor, "10000");
        assert_eq!(summary.derived.monthly_expenses.denominator, "1");

        assert_eq!(
            summary.derived.gross_yield.definitive.status,
            CalculationStatusDto::NoDisponible
        );
        assert_eq!(
            summary.derived.gross_yield.definitive.reason,
            Some(CalculationReasonDto::PendingGrossYieldFormula)
        );
        assert!(summary.derived.gross_yield.definitive.value.is_none());
        assert_eq!(
            summary.derived.gross_yield.provisional.status,
            CalculationStatusDto::Provisional
        );
        assert_eq!(
            summary.derived.gross_yield.provisional.reason,
            Some(CalculationReasonDto::ApprovedDemoPv01)
        );
        let gross = summary
            .derived
            .gross_yield
            .provisional
            .value
            .as_ref()
            .unwrap();
        assert_eq!(gross.numerator_minor, "36500");
        assert_eq!(gross.denominator, "1");

        let net = summary
            .derived
            .net_yield
            .provisional
            .value
            .as_ref()
            .unwrap();
        assert_eq!(net.numerator_minor, "30000");
        assert_eq!(
            summary.derived.net_yield.definitive.reason,
            Some(CalculationReasonDto::PendingNetYieldTreatment)
        );

        assert_eq!(
            summary.derived.coverage.definitive.status,
            CalculationStatusDto::NoDisponible
        );
        assert!(summary.derived.coverage.definitive.value.is_none());
        assert_eq!(
            summary.derived.coverage.provisional.status,
            CalculationStatusDto::Provisional
        );
        assert_eq!(
            summary.derived.coverage.provisional.value,
            Some(CoverageOutcomeDto::Covered)
        );
    }

    #[test]
    fn editar_inversion_recalcula_y_conserva_moneda() {
        let directory = tempfile::tempdir().unwrap();
        let path = database_path(&directory);
        let created = create_at(
            &path,
            create_input("2026-03", "USD", "365,00", "100", "365", "65,00"),
        )
        .unwrap();

        let updated = update_at(
            &path,
            &created.record.id,
            update_input("2026-04", "730,00", "100", "365", "65,00"),
        )
        .unwrap();

        assert_eq!(updated.record.id, created.record.id);
        assert_eq!(updated.record.currency_code, "USD");
        assert_eq!(updated.record.month, "2026-04");
        assert_eq!(updated.record.capital.minor_units, "73000");
        assert_eq!(
            updated
                .derived
                .gross_yield
                .provisional
                .value
                .as_ref()
                .unwrap()
                .numerator_minor,
            "73000"
        );
        assert_eq!(
            updated
                .derived
                .net_yield
                .provisional
                .value
                .as_ref()
                .unwrap()
                .numerator_minor,
            "66500"
        );
    }

    #[test]
    fn listar_por_mes_incluye_derivados_y_filtra_otros_meses() {
        let directory = tempfile::tempdir().unwrap();
        let path = database_path(&directory);
        let first = create_at(
            &path,
            create_input("2026-03", "ARS", "365,00", "100", "365", "65,00"),
        )
        .unwrap();
        let second = create_at(
            &path,
            create_input("2026-03", "ARS", "730,00", "100", "365", "65,00"),
        )
        .unwrap();
        create_at(
            &path,
            create_input("2026-04", "ARS", "100,00", "10", "30", "0,00"),
        )
        .unwrap();

        let records = list_at(&path, "2026-03").unwrap();

        assert_eq!(
            records
                .iter()
                .map(|item| item.record.id.clone())
                .collect::<Vec<_>>(),
            vec![first.record.id, second.record.id]
        );
        assert!(records.iter().all(|item| item.record.month == "2026-03"));
        assert!(
            records
                .iter()
                .all(|item| item.derived.gross_yield.provisional.value.is_some())
        );
    }

    #[test]
    fn errores_de_validacion_conservan_campo_en_camel_case() {
        let directory = tempfile::tempdir().unwrap();
        let path = database_path(&directory);

        let currency_error = create_at(
            &path,
            create_input("2026-03", "EUR", "100,00", "25", "30", "0"),
        )
        .unwrap_err();
        let month_error =
            create_at(&path, create_input("", "ARS", "100,00", "25", "30", "0")).unwrap_err();
        let capital_error = create_at(
            &path,
            create_input("2026-03", "ARS", "-1,00", "25", "30", "0"),
        )
        .unwrap_err();
        let taxes_error = create_at(
            &path,
            create_input("2026-03", "ARS", "100,00", "25", "30", "-1,00"),
        )
        .unwrap_err();
        let tna_error = create_at(
            &path,
            create_input("2026-03", "ARS", "100,00", "treinta", "30", "0"),
        )
        .unwrap_err();
        let term_error = create_at(
            &path,
            create_input("2026-03", "ARS", "100,00", "25", "30,5", "0"),
        )
        .unwrap_err();

        assert_eq!(currency_error.code, "validation");
        assert_eq!(currency_error.field.as_deref(), Some("currencyCode"));
        assert_eq!(month_error.field.as_deref(), Some("month"));
        assert_eq!(capital_error.field.as_deref(), Some("capital"));
        assert_eq!(taxes_error.field.as_deref(), Some("taxes"));
        assert_eq!(tna_error.field.as_deref(), Some("tna"));
        assert_eq!(term_error.field.as_deref(), Some("termDays"));
    }

    #[test]
    fn identificador_invalido_e_inexistente_devuelve_campo_id() {
        let directory = tempfile::tempdir().unwrap();
        let path = database_path(&directory);
        let valid_id = "00000000-0000-4000-8000-000000000001";

        let malformed = update_at(
            &path,
            "no-uuid",
            update_input("2026-03", "100,00", "25", "30", "0"),
        )
        .unwrap_err();
        let missing = update_at(
            &path,
            valid_id,
            update_input("2026-03", "100,00", "25", "30", "0"),
        )
        .unwrap_err();

        assert_eq!(malformed.code, "validation");
        assert_eq!(malformed.field.as_deref(), Some("id"));
        assert_eq!(missing.code, "not_found");
        assert_eq!(missing.field.as_deref(), Some("id"));
    }

    #[test]
    fn mes_invalido_en_listado_devuelve_campo_month() {
        let directory = tempfile::tempdir().unwrap();
        let path = database_path(&directory);

        let error = list_at(&path, "2026/03").unwrap_err();

        assert_eq!(error.code, "validation");
        assert_eq!(error.field.as_deref(), Some("month"));
    }

    #[test]
    fn fallo_al_derivar_no_persiste_el_alta() {
        let directory = tempfile::tempdir().unwrap();
        let path = database_path(&directory);
        let connection = database::open(&path).unwrap();

        let error = create_investment_with(
            &connection,
            &FailingServiceRepository,
            &INVESTMENT_REPOSITORY,
            create_input("2026-03", "ARS", "365,00", "100", "365", "65,00"),
        )
        .unwrap_err();

        assert_eq!(error.code, "persistence");
        drop(connection);

        let connection = database::open(&path).unwrap();
        let records = INVESTMENT_REPOSITORY
            .list_by_month(&connection, Month::parse("2026-03").unwrap())
            .unwrap();
        assert!(records.is_empty());
    }

    #[test]
    fn fallo_al_derivar_no_persiste_la_edicion() {
        let directory = tempfile::tempdir().unwrap();
        let path = database_path(&directory);
        let created = create_at(
            &path,
            create_input("2026-03", "ARS", "365,00", "100", "365", "65,00"),
        )
        .unwrap();
        let id = parse_id(&created.record.id).unwrap();
        let connection = database::open(&path).unwrap();

        let error = update_investment_with(
            &connection,
            &FailingServiceRepository,
            &INVESTMENT_REPOSITORY,
            id,
            update_input("2026-04", "730,00", "100", "365", "65,00"),
        )
        .unwrap_err();

        assert_eq!(error.code, "persistence");
        drop(connection);

        let records = list_at(&path, "2026-03").unwrap();
        assert_eq!(records.len(), 1);
        assert_eq!(records[0].record.id, created.record.id);
        assert_eq!(records[0].record.month, created.record.month);
        assert_eq!(
            records[0].record.capital.minor_units,
            created.record.capital.minor_units
        );
    }

    #[test]
    fn errores_de_persistencia_no_reciben_campo_artificial() {
        let connection = database::open_in_memory().unwrap();
        let id = InvestmentId::parse("00000000-0000-4000-8000-000000000001").unwrap();
        let investment = Investment::new("2026-03", "365,00", "100", "365", "65,00").unwrap();
        INVESTMENT_REPOSITORY
            .insert(&connection, &investment)
            .unwrap();

        let errors = [
            create_investment_with(
                &connection,
                &SERVICE_REPOSITORY,
                &FailingInvestmentRepository,
                create_input("2026-03", "ARS", "365,00", "100", "365", "65,00"),
            )
            .unwrap_err(),
            update_investment_with(
                &connection,
                &SERVICE_REPOSITORY,
                &FailingInvestmentRepository,
                id,
                update_input("2026-03", "730,00", "100", "365", "65,00"),
            )
            .unwrap_err(),
            list_investments_by_month_with(
                &connection,
                &SERVICE_REPOSITORY,
                &FailingInvestmentRepository,
                "2026-03".to_owned(),
            )
            .unwrap_err(),
            list_investments_by_month_with(
                &connection,
                &FailingServiceRepository,
                &INVESTMENT_REPOSITORY,
                "2026-03".to_owned(),
            )
            .unwrap_err(),
        ];

        for error in errors {
            assert_eq!(error.code, "persistence");
            assert!(error.field.is_none());
        }
    }

    fn list_unique_summary(path: &Path) -> InvestmentSummaryDto {
        let mut records = list_at(path, "2026-03").unwrap();
        assert_eq!(records.len(), 1);
        let summary = records.remove(0);
        let repeated = list_at(path, "2026-03").unwrap();
        assert_eq!(repeated.len(), 1);
        assert_eq!(repeated[0], summary);
        summary
    }

    fn assert_common_routes(summary: &InvestmentSummaryDto) {
        assert_eq!(summary.record.month, "2026-03");
        assert_eq!(summary.record.currency_code, "ARS");
        assert_eq!(summary.record.capital.minor_units, "36500");

        assert_eq!(
            summary.derived.gross_yield.definitive.status,
            CalculationStatusDto::NoDisponible
        );
        assert_eq!(
            summary.derived.gross_yield.definitive.reason,
            Some(CalculationReasonDto::PendingGrossYieldFormula)
        );
        assert!(summary.derived.gross_yield.definitive.value.is_none());
        assert_eq!(
            summary.derived.gross_yield.provisional.status,
            CalculationStatusDto::Provisional
        );
        assert_eq!(
            summary.derived.gross_yield.provisional.reason,
            Some(CalculationReasonDto::ApprovedDemoPv01)
        );
        let gross = summary
            .derived
            .gross_yield
            .provisional
            .value
            .as_ref()
            .unwrap();
        assert_eq!(gross.numerator_minor, "36500");
        assert_eq!(gross.denominator, "1");
        assert_eq!(gross.currency_code, "ARS");

        assert_eq!(
            summary.derived.net_yield.definitive.status,
            CalculationStatusDto::NoDisponible
        );
        assert_eq!(
            summary.derived.net_yield.definitive.reason,
            Some(CalculationReasonDto::PendingNetYieldTreatment)
        );
        assert!(summary.derived.net_yield.definitive.value.is_none());
        assert_eq!(
            summary.derived.net_yield.provisional.status,
            CalculationStatusDto::Provisional
        );
        assert_eq!(
            summary.derived.net_yield.provisional.reason,
            Some(CalculationReasonDto::ApprovedDemoPv02)
        );
        let net = summary
            .derived
            .net_yield
            .provisional
            .value
            .as_ref()
            .unwrap();
        assert_eq!(net.numerator_minor, "30000");
        assert_eq!(net.denominator, "1");
        assert_eq!(net.currency_code, "ARS");

        assert_eq!(
            summary.derived.coverage.definitive.status,
            CalculationStatusDto::NoDisponible
        );
        assert_eq!(
            summary.derived.coverage.definitive.reason,
            Some(CalculationReasonDto::PendingNetYield)
        );
        assert!(summary.derived.coverage.definitive.value.is_none());
        assert_eq!(
            summary.derived.coverage.provisional.status,
            CalculationStatusDto::Provisional
        );
        assert_eq!(
            summary.derived.coverage.provisional.reason,
            Some(CalculationReasonDto::ApprovedDemoPv03)
        );
    }

    fn assert_expenses(summary: &InvestmentSummaryDto, numerator: &str, denominator: &str) {
        let expenses = &summary.derived.monthly_expenses;
        assert_eq!(expenses.numerator_minor, numerator);
        assert_eq!(expenses.denominator, denominator);
        assert_eq!(expenses.currency_code, "ARS");
    }

    fn assert_deficit(summary: &InvestmentSummaryDto, numerator: &str, denominator: &str) {
        match &summary.derived.coverage.provisional.value {
            Some(CoverageOutcomeDto::Deficit { amount }) => {
                assert_eq!(amount.numerator_minor, numerator);
                assert_eq!(amount.denominator, denominator);
                assert_eq!(amount.currency_code, "ARS");
            }
            other => panic!("se esperaba déficit, se obtuvo {other:?}"),
        }
    }

    fn table_columns(connection: &Connection, table: &str) -> Vec<String> {
        let query = format!("PRAGMA table_info({table})");
        let mut statement = connection.prepare(&query).unwrap();
        statement
            .query_map([], |row| row.get::<_, String>(1))
            .unwrap()
            .collect::<Result<Vec<_>, _>>()
            .unwrap()
    }

    fn assert_no_derived_persistence(path: &Path) {
        let connection = database::open(path).unwrap();
        let investment_columns = table_columns(&connection, "investment");
        for forbidden in ["monthly_expenses", "gross_yield", "net_yield", "coverage"] {
            assert!(!investment_columns.iter().any(|column| column == forbidden));
        }
        let service_columns = table_columns(&connection, "service");
        assert!(!service_columns.iter().any(|column| column == "my_share"));

        let investment_count: i64 = connection
            .query_row("SELECT COUNT(*) FROM investment", [], |row| row.get(0))
            .unwrap();
        assert_eq!(investment_count, 1);
        let service_count: i64 = connection
            .query_row("SELECT COUNT(*) FROM service", [], |row| row.get(0))
            .unwrap();
        assert_eq!(service_count, 1);
    }

    #[test]
    fn los_cambios_de_servicios_recalculan_gastos_y_cobertura_de_inversiones() {
        let directory = tempfile::tempdir().unwrap();
        let path = database_path(&directory);

        create_at(
            &path,
            create_input("2026-03", "ARS", "365,00", "100", "365", "65,00"),
        )
        .unwrap();

        let main_service = create_service_at(&path, "Principal", Currency::Ars, "100,00", 1);
        let summary = list_unique_summary(&path);
        assert_common_routes(&summary);
        assert_expenses(&summary, "10000", "1");
        assert_eq!(
            summary.derived.coverage.provisional.value,
            Some(CoverageOutcomeDto::Covered)
        );

        update_service_at(&path, main_service.id(), "Principal", "1000,00", 3);
        let summary = list_unique_summary(&path);
        assert_common_routes(&summary);
        assert_expenses(&summary, "100000", "3");
        assert_deficit(&summary, "10000", "3");

        let auxiliary_service = create_service_at(&path, "Auxiliar", Currency::Ars, "50,00", 1);
        let summary = list_unique_summary(&path);
        assert_common_routes(&summary);
        assert_expenses(&summary, "115000", "3");
        assert_deficit(&summary, "25000", "3");

        delete_service_at(&path, auxiliary_service.id());
        let summary = list_unique_summary(&path);
        assert_common_routes(&summary);
        assert_expenses(&summary, "100000", "3");
        assert_deficit(&summary, "10000", "3");

        freeze_service_at(&path, main_service.id());
        let summary = list_unique_summary(&path);
        assert_common_routes(&summary);
        assert_expenses(&summary, "0", "1");
        assert_eq!(
            summary.derived.coverage.provisional.value,
            Some(CoverageOutcomeDto::ZeroExpenses)
        );

        reactivate_service_at(&path, main_service.id());
        let summary = list_unique_summary(&path);
        assert_common_routes(&summary);
        assert_expenses(&summary, "100000", "3");
        assert_deficit(&summary, "10000", "3");

        assert_no_derived_persistence(&path);
    }
}
