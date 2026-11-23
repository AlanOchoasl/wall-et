use rusqlite::Connection;

use crate::application::investment_repository::{
    InvestmentId, InvestmentRecord, InvestmentRepository,
};
use crate::application::investment_use_cases::{
    self, CreateInvestmentInput, UpdateInvestmentInput,
};
use crate::application::monthly_expenses::get_monthly_expenses;
use crate::application::service_repository::ServiceRepository;
use crate::domain::calculations::definitive::DefinitiveCalculator;
use crate::domain::calculations::provisional::ProvisionalCalculator;
use crate::domain::calculations::{Coverage, FinancialCalculator, FinancialResult};
use crate::domain::error::DomainError;
use crate::domain::investment::{Investment, Month};
use crate::domain::money::ExactMoney;

#[derive(Debug, Clone)]
pub struct FinancialResults<T> {
    pub definitive: FinancialResult<T>,
    pub provisional: FinancialResult<T>,
}

#[derive(Debug, Clone)]
pub struct InvestmentDerived {
    pub monthly_expenses: ExactMoney,
    pub gross_yield: FinancialResults<ExactMoney>,
    pub net_yield: FinancialResults<ExactMoney>,
    pub coverage: FinancialResults<Coverage>,
}

#[derive(Debug, Clone)]
pub struct InvestmentDetails {
    pub record: InvestmentRecord,
    pub derived: InvestmentDerived,
}

pub fn create_investment_details<S, I>(
    connection: &Connection,
    service_repository: &S,
    investment_repository: &I,
    input: CreateInvestmentInput,
) -> Result<InvestmentDetails, DomainError>
where
    S: ServiceRepository + ?Sized,
    I: InvestmentRepository + ?Sized,
{
    let investment = investment_use_cases::prepare_create_investment(input)?;
    let derived = derive_details(connection, service_repository, &investment)?;
    let record = investment_repository.insert(connection, &investment)?;

    Ok(InvestmentDetails { record, derived })
}

pub fn update_investment_details<S, I>(
    connection: &Connection,
    service_repository: &S,
    investment_repository: &I,
    id: InvestmentId,
    input: UpdateInvestmentInput,
) -> Result<InvestmentDetails, DomainError>
where
    S: ServiceRepository + ?Sized,
    I: InvestmentRepository + ?Sized,
{
    let current = investment_repository.get(connection, id)?;
    let investment = investment_use_cases::prepare_update_investment(current, input)?;
    let derived = derive_details(connection, service_repository, &investment)?;
    let record = investment_repository.update(connection, id, &investment)?;

    Ok(InvestmentDetails { record, derived })
}

pub fn list_investment_details_by_month<S, I>(
    connection: &Connection,
    service_repository: &S,
    investment_repository: &I,
    month: &str,
) -> Result<Vec<InvestmentDetails>, DomainError>
where
    S: ServiceRepository + ?Sized,
    I: InvestmentRepository + ?Sized,
{
    let month = Month::parse(month)?;
    let records = investment_repository.list_by_month(connection, month)?;

    records
        .iter()
        .map(|record| details_from_record(connection, service_repository, record))
        .collect()
}

fn details_from_record<S>(
    connection: &Connection,
    service_repository: &S,
    record: &InvestmentRecord,
) -> Result<InvestmentDetails, DomainError>
where
    S: ServiceRepository + ?Sized,
{
    let derived = derive_details(connection, service_repository, record.investment())?;

    Ok(InvestmentDetails {
        record: record.clone(),
        derived,
    })
}

fn derive_details<S>(
    connection: &Connection,
    service_repository: &S,
    investment: &Investment,
) -> Result<InvestmentDerived, DomainError>
where
    S: ServiceRepository + ?Sized,
{
    let monthly_expenses =
        get_monthly_expenses(connection, service_repository, investment.currency())?;

    let gross_yield = FinancialResults {
        definitive: DefinitiveCalculator.gross_yield(investment)?,
        provisional: ProvisionalCalculator.gross_yield(investment)?,
    };
    let net_yield = FinancialResults {
        definitive: DefinitiveCalculator.net_yield(investment)?,
        provisional: ProvisionalCalculator.net_yield(investment)?,
    };
    let coverage = FinancialResults {
        definitive: DefinitiveCalculator.coverage(&net_yield.definitive, &monthly_expenses)?,
        provisional: ProvisionalCalculator.coverage(&net_yield.provisional, &monthly_expenses)?,
    };

    Ok(InvestmentDerived {
        monthly_expenses,
        gross_yield,
        net_yield,
        coverage,
    })
}
