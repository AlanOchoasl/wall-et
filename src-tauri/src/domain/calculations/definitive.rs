use crate::domain::calculations::{CalculationReason, FinancialCalculator, FinancialResult};
use crate::domain::error::DomainError;
use crate::domain::investment::Investment;
use crate::domain::money::{Currency, ExactMoney};
use crate::domain::service_totals::ServiceTotals;

#[derive(Debug, Clone, Copy, Default)]
pub struct DefinitiveCalculator;

impl FinancialCalculator for DefinitiveCalculator {
    fn gross_yield(
        &self,
        _investment: &Investment,
    ) -> Result<FinancialResult<ExactMoney>, DomainError> {
        Ok(FinancialResult::Unavailable {
            reason: CalculationReason::PendingGrossYieldFormula,
        })
    }

    fn net_yield(
        &self,
        _investment: &Investment,
    ) -> Result<FinancialResult<ExactMoney>, DomainError> {
        Ok(FinancialResult::Unavailable {
            reason: CalculationReason::PendingNetYieldTreatment,
        })
    }

    fn coverage(
        &self,
        _investment_currency: Currency,
        _net_yield: &FinancialResult<ExactMoney>,
        _monthly_expenses: &ServiceTotals,
    ) -> Result<FinancialResult<crate::domain::calculations::Coverage>, DomainError> {
        Ok(FinancialResult::Unavailable {
            reason: CalculationReason::PendingNetYield,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::calculations::{CalculationStatus, Coverage};
    use crate::domain::service::Service;
    use crate::domain::service_totals::calculate_service_totals;

    fn investment() -> Investment {
        Investment::new("2026-01", "1000,00", "10", "30", "0").unwrap()
    }

    fn totals() -> ServiceTotals {
        let service = Service::new("Gasto", "500,00", 1, None).unwrap();
        calculate_service_totals(std::slice::from_ref(&service)).unwrap()
    }

    fn assert_unavailable<T>(result: FinancialResult<T>, reason: CalculationReason) {
        assert_eq!(result.status(), CalculationStatus::Unavailable);
        assert_eq!(result.reason(), Some(reason));
        assert!(result.value().is_none());
    }

    #[test]
    fn rendimiento_bruto_definitivo_no_disponible() {
        let result = DefinitiveCalculator.gross_yield(&investment()).unwrap();

        assert_unavailable(result, CalculationReason::PendingGrossYieldFormula);
    }

    #[test]
    fn rendimiento_neto_definitivo_no_disponible() {
        let result = DefinitiveCalculator.net_yield(&investment()).unwrap();

        assert_unavailable(result, CalculationReason::PendingNetYieldTreatment);
    }

    #[test]
    fn cobertura_definitiva_no_disponible() {
        let net_yield = DefinitiveCalculator.net_yield(&investment()).unwrap();
        let result = DefinitiveCalculator
            .coverage(Currency::Ars, &net_yield, &totals())
            .unwrap();

        assert_unavailable(result, CalculationReason::PendingNetYield);
    }

    #[test]
    fn ruta_definitiva_no_contiene_formula_alternativa() {
        let source = include_str!("definitive.rs");
        let basis_marker = ["3", "65"].concat();
        let provisional_marker = ["Provis", "ional"].concat();

        assert!(!source.contains(basis_marker.as_str()));
        assert!(!source.contains(provisional_marker.as_str()));
    }

    #[test]
    fn modulo_no_usa_punto_flotante_binario() {
        let source = include_str!("definitive.rs").as_bytes();
        let narrow: &[u8] = &[0x66, 0x33, 0x32];
        let wide: &[u8] = &[0x66, 0x36, 0x34];

        assert!(!source.windows(narrow.len()).any(|window| window == narrow));
        assert!(!source.windows(wide.len()).any(|window| window == wide));
    }

    #[test]
    fn cobertura_no_expone_valor() {
        let net_yield = FinancialResult::Unavailable {
            reason: CalculationReason::PendingNetYield,
        };
        let result = DefinitiveCalculator
            .coverage(Currency::Ars, &net_yield, &totals())
            .unwrap();

        assert!(result.value().is_none());
        assert!(!matches!(result.value(), Some(Coverage::Covered)));
    }
}
