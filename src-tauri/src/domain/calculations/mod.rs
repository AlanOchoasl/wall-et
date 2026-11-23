pub mod definitive;
pub mod provisional;
mod result;

pub use result::{CalculationReason, CalculationStatus, Coverage, FinancialResult};

use std::cmp::Ordering;

use crate::domain::error::DomainError;
use crate::domain::investment::Investment;
use crate::domain::money::{Currency, ExactMoney};
use crate::domain::service_totals::ServiceTotals;

pub(crate) fn coverage_verdict(
    investment_currency: Currency,
    net_yield: &ExactMoney,
    monthly_expenses: &ServiceTotals,
) -> Result<Coverage, DomainError> {
    let ars = monthly_expenses.ars().user_share();
    let usd = monthly_expenses.usd().user_share();
    let has_ars = ars.numerator_minor() != 0;
    let has_usd = usd.numerator_minor() != 0;

    if !has_ars && !has_usd {
        return Ok(Coverage::ZeroExpenses);
    }

    let has_other_currency = match investment_currency {
        Currency::Ars => has_usd,
        Currency::Usd => has_ars,
    };

    if has_other_currency {
        return Ok(Coverage::IncompatibleCurrencies);
    }

    if net_yield.currency() != investment_currency {
        return Err(DomainError::currency_mismatch(
            investment_currency.code(),
            net_yield.currency().code(),
        ));
    }

    let expenses = monthly_expenses.user_share(investment_currency);

    match net_yield.cmp_exact(expenses)? {
        Ordering::Less => Ok(Coverage::Deficit {
            amount: expenses.checked_sub(net_yield)?,
        }),
        Ordering::Equal | Ordering::Greater => Ok(Coverage::Covered),
    }
}

pub trait FinancialCalculator {
    fn gross_yield(
        &self,
        investment: &Investment,
    ) -> Result<FinancialResult<ExactMoney>, DomainError>;

    fn net_yield(
        &self,
        investment: &Investment,
    ) -> Result<FinancialResult<ExactMoney>, DomainError>;

    fn coverage(
        &self,
        investment_currency: Currency,
        net_yield: &FinancialResult<ExactMoney>,
        monthly_expenses: &ServiceTotals,
    ) -> Result<FinancialResult<Coverage>, DomainError>;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn calculadoras_cumplen_la_interfaz_comun() {
        fn assert_calculator<T: FinancialCalculator>() {}

        assert_calculator::<definitive::DefinitiveCalculator>();
        assert_calculator::<provisional::ProvisionalCalculator>();
    }
}
