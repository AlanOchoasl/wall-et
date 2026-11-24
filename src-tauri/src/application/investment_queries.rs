use rusqlite::Connection;

use crate::application::investment_repository::{
    InvestmentId, InvestmentRecord, InvestmentRepository,
};
use crate::application::investment_use_cases::{
    self, CreateInvestmentInput, UpdateInvestmentInput,
};
use crate::application::monthly_expenses::get_service_totals;
use crate::application::service_repository::ServiceRepository;
use crate::domain::calculations::definitive::DefinitiveCalculator;
use crate::domain::calculations::provisional::ProvisionalCalculator;
use crate::domain::calculations::{Coverage, FinancialCalculator, FinancialResult};
use crate::domain::error::DomainError;
use crate::domain::investment::{Investment, Month};
use crate::domain::money::ExactMoney;
use crate::domain::service_totals::ServiceTotals;

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

#[derive(Debug, Clone)]
pub struct MonthlySummary {
    month: Month,
    service_totals: ServiceTotals,
    investments: Vec<InvestmentDetails>,
}

impl MonthlySummary {
    pub fn month(&self) -> Month {
        self.month
    }

    pub fn service_totals(&self) -> &ServiceTotals {
        &self.service_totals
    }

    pub fn investments(&self) -> &[InvestmentDetails] {
        &self.investments
    }
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

pub fn get_monthly_summary<S, I>(
    connection: &Connection,
    service_repository: &S,
    investment_repository: &I,
    month: &str,
) -> Result<MonthlySummary, DomainError>
where
    S: ServiceRepository + ?Sized,
    I: InvestmentRepository + ?Sized,
{
    let month = Month::parse(month)?;
    let service_totals = get_service_totals(connection, service_repository)?;
    let records = investment_repository.list_by_month(connection, month)?;
    let investments = records
        .iter()
        .map(|record| details_from_record_with_totals(record, &service_totals))
        .collect::<Result<Vec<_>, _>>()?;

    Ok(MonthlySummary {
        month,
        service_totals,
        investments,
    })
}

fn details_from_record<S>(
    connection: &Connection,
    service_repository: &S,
    record: &InvestmentRecord,
) -> Result<InvestmentDetails, DomainError>
where
    S: ServiceRepository + ?Sized,
{
    let service_totals = get_service_totals(connection, service_repository)?;
    details_from_record_with_totals(record, &service_totals)
}

fn details_from_record_with_totals(
    record: &InvestmentRecord,
    service_totals: &ServiceTotals,
) -> Result<InvestmentDetails, DomainError> {
    let derived = derive_details_with_totals(record.investment(), service_totals)?;

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
    let service_totals = get_service_totals(connection, service_repository)?;
    derive_details_with_totals(investment, &service_totals)
}

fn derive_details_with_totals(
    investment: &Investment,
    service_totals: &ServiceTotals,
) -> Result<InvestmentDerived, DomainError> {
    let monthly_expenses = service_totals.user_share(investment.currency()).clone();

    let gross_yield = FinancialResults {
        definitive: DefinitiveCalculator.gross_yield(investment)?,
        provisional: ProvisionalCalculator.gross_yield(investment)?,
    };
    let net_yield = FinancialResults {
        definitive: DefinitiveCalculator.net_yield(investment)?,
        provisional: ProvisionalCalculator.net_yield(investment)?,
    };
    let coverage = FinancialResults {
        definitive: DefinitiveCalculator.coverage(
            investment.currency(),
            &net_yield.definitive,
            service_totals,
        )?,
        provisional: ProvisionalCalculator.coverage(
            investment.currency(),
            &net_yield.provisional,
            service_totals,
        )?,
    };

    Ok(InvestmentDerived {
        monthly_expenses,
        gross_yield,
        net_yield,
        coverage,
    })
}
