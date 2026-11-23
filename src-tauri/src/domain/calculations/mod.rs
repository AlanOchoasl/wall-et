pub mod definitive;
pub mod provisional;
mod result;

pub use result::{CalculationReason, CalculationStatus, Coverage, FinancialResult};

use crate::domain::error::DomainError;
use crate::domain::investment::Investment;
use crate::domain::money::ExactMoney;

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
        net_yield: &FinancialResult<ExactMoney>,
        monthly_expenses: &ExactMoney,
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
