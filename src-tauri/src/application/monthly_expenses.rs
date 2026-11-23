use rusqlite::Connection;

use crate::application::investment_repository::{InvestmentId, InvestmentRepository};
use crate::application::service_repository::ServiceRepository;
use crate::domain::error::DomainError;
use crate::domain::investment::Month;
use crate::domain::money::{Currency, ExactMoney};
use crate::domain::monthly_expenses::calculate_monthly_expenses;
use crate::domain::service::Service;

#[derive(Debug, Clone)]
pub struct InvestmentMonthlyExpenses {
    month: Month,
    currency: Currency,
    amount: ExactMoney,
}

impl InvestmentMonthlyExpenses {
    pub const fn month(&self) -> Month {
        self.month
    }

    pub const fn currency(&self) -> Currency {
        self.currency
    }

    pub const fn amount(&self) -> &ExactMoney {
        &self.amount
    }
}

pub fn get_monthly_expenses<R>(
    connection: &Connection,
    repository: &R,
    currency: Currency,
) -> Result<ExactMoney, DomainError>
where
    R: ServiceRepository + ?Sized,
{
    let records = repository.list_active(connection)?;
    let services: Vec<Service> = records
        .iter()
        .map(|record| record.service().clone())
        .collect();

    calculate_monthly_expenses(&services, currency)
}

pub fn get_investment_monthly_expenses<S, I>(
    connection: &Connection,
    service_repository: &S,
    investment_repository: &I,
    investment_id: InvestmentId,
) -> Result<InvestmentMonthlyExpenses, DomainError>
where
    S: ServiceRepository + ?Sized,
    I: InvestmentRepository + ?Sized,
{
    let record = investment_repository.get(connection, investment_id)?;
    let investment = record.investment();
    let amount = get_monthly_expenses(connection, service_repository, investment.currency())?;

    Ok(InvestmentMonthlyExpenses {
        month: investment.month(),
        currency: investment.currency(),
        amount,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::application::investment_repository::InvestmentRecord;
    use crate::application::investment_use_cases::{CreateInvestmentInput, create_investment};
    use crate::application::service_repository::ServiceId;
    use crate::application::service_use_cases::{
        CreateServiceInput, DeleteConfirmation, UpdateServiceInput, create_service, delete_service,
        freeze_service, reactivate_service, update_service,
    };
    use crate::domain::calculations::provisional::ProvisionalCalculator;
    use crate::domain::calculations::{
        CalculationReason, CalculationStatus, Coverage, FinancialCalculator, FinancialResult,
    };
    use crate::domain::service::ServiceStatus;
    use crate::infrastructure::database;
    use crate::infrastructure::investment_repository::SqliteInvestmentRepository;
    use crate::infrastructure::service_repository::SqliteServiceRepository;

    const SERVICE_REPOSITORY: SqliteServiceRepository = SqliteServiceRepository;
    const INVESTMENT_REPOSITORY: SqliteInvestmentRepository = SqliteInvestmentRepository;

    fn service_input(
        name: &str,
        total_cost: &str,
        currency: Currency,
        shared_people_count: u32,
    ) -> CreateServiceInput {
        CreateServiceInput {
            name: name.to_owned(),
            total_cost: total_cost.to_owned(),
            currency,
            shared_people_count,
            notes: None,
        }
    }

    fn investment_input(month: &str, currency: Currency) -> CreateInvestmentInput {
        CreateInvestmentInput {
            month: month.to_owned(),
            currency,
            ..CreateInvestmentInput::default()
        }
    }

    fn create_test_investment(
        connection: &Connection,
        month: &str,
        currency: Currency,
    ) -> InvestmentRecord {
        create_investment(
            connection,
            &INVESTMENT_REPOSITORY,
            investment_input(month, currency),
        )
        .unwrap()
    }

    fn unknown_investment_id() -> InvestmentId {
        InvestmentId::parse("00000000-0000-4000-8000-000000000099").unwrap()
    }

    fn assert_exact(
        amount: &ExactMoney,
        numerator_minor: i128,
        denominator: u128,
        currency: Currency,
    ) {
        assert_eq!(amount.numerator_minor(), numerator_minor);
        assert_eq!(amount.denominator(), denominator);
        assert_eq!(amount.currency(), currency);
    }

    fn count_services(connection: &Connection) -> i64 {
        connection
            .query_row("SELECT COUNT(*) FROM service", [], |row| row.get(0))
            .unwrap()
    }

    fn count_investments(connection: &Connection) -> i64 {
        connection
            .query_row("SELECT COUNT(*) FROM investment", [], |row| row.get(0))
            .unwrap()
    }

    fn provisional_net(value: ExactMoney) -> FinancialResult<ExactMoney> {
        FinancialResult::Provisional {
            value,
            reason: CalculationReason::ApprovedDemoPv02,
        }
    }

    struct FailingServiceRepository;

    impl FailingServiceRepository {
        fn error() -> DomainError {
            DomainError::Persistence("repositorio de servicios no disponible".to_owned())
        }
    }

    impl ServiceRepository for FailingServiceRepository {
        fn insert(
            &self,
            _connection: &Connection,
            _service: &Service,
        ) -> Result<crate::application::service_repository::ServiceRecord, DomainError> {
            Err(Self::error())
        }

        fn get(
            &self,
            _connection: &Connection,
            _id: ServiceId,
        ) -> Result<Option<crate::application::service_repository::ServiceRecord>, DomainError>
        {
            Err(Self::error())
        }

        fn update(
            &self,
            _connection: &Connection,
            _id: ServiceId,
            _service: &Service,
        ) -> Result<crate::application::service_repository::ServiceRecord, DomainError> {
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
        ) -> Result<crate::application::service_repository::ServiceRecord, DomainError> {
            Err(Self::error())
        }

        fn list_active(
            &self,
            _connection: &Connection,
        ) -> Result<Vec<crate::application::service_repository::ServiceRecord>, DomainError>
        {
            Err(Self::error())
        }

        fn list_frozen(
            &self,
            _connection: &Connection,
        ) -> Result<Vec<crate::application::service_repository::ServiceRecord>, DomainError>
        {
            Err(Self::error())
        }
    }

    struct FailingInvestmentRepository;

    impl FailingInvestmentRepository {
        fn error() -> DomainError {
            DomainError::Persistence("repositorio de inversiones no disponible".to_owned())
        }
    }

    impl InvestmentRepository for FailingInvestmentRepository {
        fn insert(
            &self,
            _connection: &Connection,
            _investment: &crate::domain::investment::Investment,
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
            _investment: &crate::domain::investment::Investment,
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

    #[test]
    fn consulta_servicios_activos_y_deriva_la_suma_en_la_moneda_solicitada() {
        let connection = database::open_in_memory().unwrap();
        create_service(
            &connection,
            &SERVICE_REPOSITORY,
            service_input("A", "10000,00", Currency::Ars, 4),
        )
        .unwrap();
        create_service(
            &connection,
            &SERVICE_REPOSITORY,
            service_input("B", "6000,00", Currency::Ars, 2),
        )
        .unwrap();
        create_service(
            &connection,
            &SERVICE_REPOSITORY,
            service_input("Congelado", "5000,00", Currency::Ars, 1),
        )
        .unwrap();
        let frozen = SERVICE_REPOSITORY
            .list_active(&connection)
            .unwrap()
            .into_iter()
            .find(|record| record.service().name() == "Congelado")
            .unwrap();
        freeze_service(&connection, &SERVICE_REPOSITORY, frozen.id()).unwrap();

        let expenses =
            get_monthly_expenses(&connection, &SERVICE_REPOSITORY, Currency::Ars).unwrap();

        assert_exact(&expenses, 550_000, 1, Currency::Ars);
    }

    #[test]
    fn derivacion_en_moneda_sin_servicios_devuelve_cero_exacto() {
        let connection = database::open_in_memory().unwrap();
        create_service(
            &connection,
            &SERVICE_REPOSITORY,
            service_input("Luz", "10000,00", Currency::Ars, 4),
        )
        .unwrap();

        let usd_expenses =
            get_monthly_expenses(&connection, &SERVICE_REPOSITORY, Currency::Usd).unwrap();

        assert_exact(&usd_expenses, 0, 1, Currency::Usd);
    }

    #[test]
    fn inversion_inexistente_devuelve_not_found() {
        let connection = database::open_in_memory().unwrap();

        let error = get_investment_monthly_expenses(
            &connection,
            &SERVICE_REPOSITORY,
            &INVESTMENT_REPOSITORY,
            unknown_investment_id(),
        )
        .unwrap_err();

        assert!(matches!(error, DomainError::NotFound));
    }

    #[test]
    fn inversion_ars_con_servicios_ars_devuelve_resultado_tipado() {
        let connection = database::open_in_memory().unwrap();
        let investment = create_test_investment(&connection, "2026-03", Currency::Ars);
        create_service(
            &connection,
            &SERVICE_REPOSITORY,
            service_input("A", "10000,00", Currency::Ars, 4),
        )
        .unwrap();
        create_service(
            &connection,
            &SERVICE_REPOSITORY,
            service_input("B", "6000,00", Currency::Ars, 2),
        )
        .unwrap();

        let result = get_investment_monthly_expenses(
            &connection,
            &SERVICE_REPOSITORY,
            &INVESTMENT_REPOSITORY,
            investment.id(),
        )
        .unwrap();

        assert_eq!(result.month().code(), "2026-03");
        assert_eq!(result.currency(), Currency::Ars);
        assert_exact(result.amount(), 550_000, 1, Currency::Ars);
    }

    #[test]
    fn inversion_usd_sin_servicios_usd_devuelve_cero_usd() {
        let connection = database::open_in_memory().unwrap();
        let investment = create_test_investment(&connection, "2026-03", Currency::Usd);
        create_service(
            &connection,
            &SERVICE_REPOSITORY,
            service_input("Luz", "10000,00", Currency::Ars, 4),
        )
        .unwrap();

        let result = get_investment_monthly_expenses(
            &connection,
            &SERVICE_REPOSITORY,
            &INVESTMENT_REPOSITORY,
            investment.id(),
        )
        .unwrap();

        assert_eq!(result.currency(), Currency::Usd);
        assert_exact(result.amount(), 0, 1, Currency::Usd);
    }

    #[test]
    fn recalcula_tras_congelar_y_reactivar_un_servicio() {
        let connection = database::open_in_memory().unwrap();
        let investment = create_test_investment(&connection, "2026-03", Currency::Ars);
        let first = create_service(
            &connection,
            &SERVICE_REPOSITORY,
            service_input("A", "10000,00", Currency::Ars, 4),
        )
        .unwrap();
        create_service(
            &connection,
            &SERVICE_REPOSITORY,
            service_input("B", "6000,00", Currency::Ars, 2),
        )
        .unwrap();

        let before = get_investment_monthly_expenses(
            &connection,
            &SERVICE_REPOSITORY,
            &INVESTMENT_REPOSITORY,
            investment.id(),
        )
        .unwrap();
        freeze_service(&connection, &SERVICE_REPOSITORY, first.id()).unwrap();
        let frozen = get_investment_monthly_expenses(
            &connection,
            &SERVICE_REPOSITORY,
            &INVESTMENT_REPOSITORY,
            investment.id(),
        )
        .unwrap();
        reactivate_service(&connection, &SERVICE_REPOSITORY, first.id()).unwrap();
        let reactivated = get_investment_monthly_expenses(
            &connection,
            &SERVICE_REPOSITORY,
            &INVESTMENT_REPOSITORY,
            investment.id(),
        )
        .unwrap();

        assert_exact(before.amount(), 550_000, 1, Currency::Ars);
        assert_exact(frozen.amount(), 300_000, 1, Currency::Ars);
        assert_exact(reactivated.amount(), 550_000, 1, Currency::Ars);
    }

    #[test]
    fn recalcula_tras_crear_editar_y_eliminar_servicios() {
        let connection = database::open_in_memory().unwrap();
        let investment = create_test_investment(&connection, "2026-03", Currency::Ars);
        let first = create_service(
            &connection,
            &SERVICE_REPOSITORY,
            service_input("A", "10000,00", Currency::Ars, 4),
        )
        .unwrap();

        let initial = get_investment_monthly_expenses(
            &connection,
            &SERVICE_REPOSITORY,
            &INVESTMENT_REPOSITORY,
            investment.id(),
        )
        .unwrap();
        let second = create_service(
            &connection,
            &SERVICE_REPOSITORY,
            service_input("B", "6000,00", Currency::Ars, 2),
        )
        .unwrap();
        let after_create = get_investment_monthly_expenses(
            &connection,
            &SERVICE_REPOSITORY,
            &INVESTMENT_REPOSITORY,
            investment.id(),
        )
        .unwrap();
        update_service(
            &connection,
            &SERVICE_REPOSITORY,
            first.id(),
            UpdateServiceInput {
                name: "A".to_owned(),
                total_cost: "20000,00".to_owned(),
                shared_people_count: 4,
                notes: None,
            },
        )
        .unwrap();
        let after_update = get_investment_monthly_expenses(
            &connection,
            &SERVICE_REPOSITORY,
            &INVESTMENT_REPOSITORY,
            investment.id(),
        )
        .unwrap();
        delete_service(
            &connection,
            &SERVICE_REPOSITORY,
            second.id(),
            DeleteConfirmation::Confirmed,
        )
        .unwrap();
        let after_delete = get_investment_monthly_expenses(
            &connection,
            &SERVICE_REPOSITORY,
            &INVESTMENT_REPOSITORY,
            investment.id(),
        )
        .unwrap();

        assert_exact(initial.amount(), 250_000, 1, Currency::Ars);
        assert_exact(after_create.amount(), 550_000, 1, Currency::Ars);
        assert_exact(after_update.amount(), 800_000, 1, Currency::Ars);
        assert_exact(after_delete.amount(), 500_000, 1, Currency::Ars);
    }

    #[test]
    fn propaga_error_del_repositorio_de_servicios() {
        let connection = database::open_in_memory().unwrap();

        let error = get_monthly_expenses(&connection, &FailingServiceRepository, Currency::Ars)
            .unwrap_err();

        assert!(matches!(
            error,
            DomainError::Persistence(message) if message == "repositorio de servicios no disponible"
        ));
    }

    #[test]
    fn propaga_error_del_repositorio_de_inversiones() {
        let connection = database::open_in_memory().unwrap();

        let error = get_investment_monthly_expenses(
            &connection,
            &SERVICE_REPOSITORY,
            &FailingInvestmentRepository,
            unknown_investment_id(),
        )
        .unwrap_err();

        assert!(matches!(
            error,
            DomainError::Persistence(message) if message == "repositorio de inversiones no disponible"
        ));
    }

    #[test]
    fn no_persiste_el_resultado_derivado() {
        let connection = database::open_in_memory().unwrap();
        let investment = create_test_investment(&connection, "2026-03", Currency::Ars);
        create_service(
            &connection,
            &SERVICE_REPOSITORY,
            service_input("A", "10000,00", Currency::Ars, 4),
        )
        .unwrap();
        let services_before = count_services(&connection);
        let investments_before = count_investments(&connection);

        get_investment_monthly_expenses(
            &connection,
            &SERVICE_REPOSITORY,
            &INVESTMENT_REPOSITORY,
            investment.id(),
        )
        .unwrap();

        assert_eq!(count_services(&connection), services_before);
        assert_eq!(count_investments(&connection), investments_before);
    }

    #[test]
    fn gastos_derivados_alimentan_la_cobertura_en_la_misma_moneda() {
        let connection = database::open_in_memory().unwrap();
        create_service(
            &connection,
            &SERVICE_REPOSITORY,
            service_input("A", "10000,00", Currency::Ars, 4),
        )
        .unwrap();
        create_service(
            &connection,
            &SERVICE_REPOSITORY,
            service_input("B", "6000,00", Currency::Ars, 2),
        )
        .unwrap();
        let expenses =
            get_monthly_expenses(&connection, &SERVICE_REPOSITORY, Currency::Ars).unwrap();
        let net_yield = provisional_net(ExactMoney::from_parts(800_000, 1, Currency::Ars).unwrap());

        let coverage = ProvisionalCalculator
            .coverage(&net_yield, &expenses)
            .unwrap();

        assert_eq!(coverage.status(), CalculationStatus::Provisional);
        assert!(matches!(coverage.value(), Some(Coverage::Covered)));
    }

    #[test]
    fn cobertura_rechaza_gastos_derivados_en_otra_moneda() {
        let connection = database::open_in_memory().unwrap();
        create_service(
            &connection,
            &SERVICE_REPOSITORY,
            service_input("A", "10000,00", Currency::Ars, 4),
        )
        .unwrap();
        let expenses =
            get_monthly_expenses(&connection, &SERVICE_REPOSITORY, Currency::Ars).unwrap();
        let net_yield = provisional_net(ExactMoney::from_parts(800_000, 1, Currency::Usd).unwrap());

        let error = ProvisionalCalculator
            .coverage(&net_yield, &expenses)
            .unwrap_err();

        assert!(matches!(
            error,
            DomainError::CurrencyMismatch {
                expected: "USD",
                found: "ARS",
            }
        ));
    }

    #[test]
    fn cobertura_no_disponible_no_emite_veredicto_con_gastos_derivados() {
        let connection = database::open_in_memory().unwrap();
        let expenses =
            get_monthly_expenses(&connection, &SERVICE_REPOSITORY, Currency::Ars).unwrap();
        let net_yield = FinancialResult::Unavailable {
            reason: CalculationReason::PendingNetYield,
        };

        let coverage = ProvisionalCalculator
            .coverage(&net_yield, &expenses)
            .unwrap();

        assert_eq!(coverage.status(), CalculationStatus::Unavailable);
        assert!(coverage.value().is_none());
    }
}
