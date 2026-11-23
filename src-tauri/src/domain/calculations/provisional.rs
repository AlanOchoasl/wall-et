use crate::domain::calculations::{
    CalculationReason, CalculationStatus, Coverage, FinancialCalculator, FinancialResult,
    coverage_verdict,
};
use crate::domain::error::DomainError;
use crate::domain::investment::Investment;
use crate::domain::money::{Currency, ExactMoney};
use crate::domain::service_totals::ServiceTotals;

const TNA_SCALE: u128 = 10_000;
const DAYS_BASIS: u128 = 365;
const GROSS_DENOMINATOR: u128 = TNA_SCALE * DAYS_BASIS;

#[derive(Debug, Clone, Copy, Default)]
pub struct ProvisionalCalculator;

impl FinancialCalculator for ProvisionalCalculator {
    fn gross_yield(
        &self,
        investment: &Investment,
    ) -> Result<FinancialResult<ExactMoney>, DomainError> {
        let numerator = gross_numerator(investment)?;
        let value = ExactMoney::from_parts(numerator, GROSS_DENOMINATOR, investment.currency())?;

        Ok(FinancialResult::Provisional {
            value,
            reason: CalculationReason::ApprovedDemoPv01,
        })
    }

    fn net_yield(
        &self,
        investment: &Investment,
    ) -> Result<FinancialResult<ExactMoney>, DomainError> {
        let gross = self.gross_yield(investment)?;
        let gross_value = match gross {
            FinancialResult::Available(value) => value,
            FinancialResult::Provisional { value, .. } => value,
            FinancialResult::Unavailable { reason } => {
                return Ok(FinancialResult::Unavailable { reason });
            }
        };
        let taxes = ExactMoney::from_money(investment.taxes());
        let value = gross_value.checked_sub(&taxes)?;

        Ok(FinancialResult::Provisional {
            value,
            reason: CalculationReason::ApprovedDemoPv02,
        })
    }

    fn coverage(
        &self,
        investment_currency: Currency,
        net_yield: &FinancialResult<ExactMoney>,
        monthly_expenses: &ServiceTotals,
    ) -> Result<FinancialResult<Coverage>, DomainError> {
        let status = net_yield.status();
        let reason = net_yield.reason();
        let net_value = match net_yield {
            FinancialResult::Available(value) => value,
            FinancialResult::Provisional { value, .. } => value,
            FinancialResult::Unavailable { .. } => {
                return Ok(FinancialResult::Unavailable {
                    reason: reason.unwrap_or(CalculationReason::PendingNetYield),
                });
            }
        };

        let verdict = coverage_verdict(investment_currency, net_value, monthly_expenses)?;

        match status {
            CalculationStatus::Available => Ok(FinancialResult::Available(verdict)),
            CalculationStatus::Provisional => Ok(FinancialResult::Provisional {
                value: verdict,
                reason: CalculationReason::ApprovedDemoPv03,
            }),
            CalculationStatus::Unavailable => Ok(FinancialResult::Unavailable {
                reason: reason.unwrap_or(CalculationReason::PendingNetYield),
            }),
        }
    }
}

fn gross_numerator(investment: &Investment) -> Result<i128, DomainError> {
    let capital = i128::from(investment.capital().minor_units());
    let tna_basis_points = i128::from(investment.tna_basis_points());
    let term_days = i128::from(investment.term_days());

    capital
        .checked_mul(tna_basis_points)
        .and_then(|product| product.checked_mul(term_days))
        .ok_or_else(|| DomainError::arithmetic_overflow("yield", "cálculo provisional PV-01"))
}

#[cfg(test)]
mod tests {
    use std::cmp::Ordering;

    use super::*;
    use crate::domain::money::Money;
    use crate::domain::service::Service;
    use crate::domain::service_totals::calculate_service_totals;

    fn investment(capital: &str, tna: &str, term_days: &str, taxes: &str) -> Investment {
        Investment::new("2026-01", capital, tna, term_days, taxes).unwrap()
    }

    fn usd_investment(capital: &str, tna: &str, term_days: &str, taxes: &str) -> Investment {
        Investment::new_with_currency("2026-01", capital, Currency::Usd, tna, term_days, taxes)
            .unwrap()
    }

    fn ars(minor_units: i64) -> ExactMoney {
        ExactMoney::from_money(Money::from_minor_units(minor_units, Currency::Ars))
    }

    fn amount(minor_units: i64) -> String {
        let sign = if minor_units < 0 { "-" } else { "" };
        let absolute = minor_units.unsigned_abs();
        format!("{}{},{:02}", sign, absolute / 100, absolute % 100)
    }

    fn totals(ars_minor: i64, usd_minor: i64) -> ServiceTotals {
        let mut services = Vec::new();

        if ars_minor != 0 {
            services.push(
                Service::new_with_currency("ARS", &amount(ars_minor), Currency::Ars, 1, None)
                    .unwrap(),
            );
        }

        if usd_minor != 0 {
            services.push(
                Service::new_with_currency("USD", &amount(usd_minor), Currency::Usd, 1, None)
                    .unwrap(),
            );
        }

        calculate_service_totals(&services).unwrap()
    }

    fn net_result(minor_units: i64) -> FinancialResult<ExactMoney> {
        FinancialResult::Provisional {
            value: ars(minor_units),
            reason: CalculationReason::ApprovedDemoPv02,
        }
    }

    fn net_result_usd(minor_units: i64) -> FinancialResult<ExactMoney> {
        FinancialResult::Provisional {
            value: ExactMoney::from_money(Money::from_minor_units(minor_units, Currency::Usd)),
            reason: CalculationReason::ApprovedDemoPv02,
        }
    }

    fn assert_provisional<T>(result: &FinancialResult<T>, reason: CalculationReason) {
        assert_eq!(result.status(), CalculationStatus::Provisional);
        assert_eq!(result.reason(), Some(reason));
        assert!(result.value().is_some());
    }

    fn assert_exact(value: &ExactMoney, numerator: i128, denominator: u128, currency: Currency) {
        assert_eq!(value.numerator_minor(), numerator);
        assert_eq!(value.denominator(), denominator);
        assert_eq!(value.currency(), currency);
    }

    #[test]
    fn pv01_calculo_de_referencia_entero() {
        let calculator = ProvisionalCalculator;
        let investment = investment("1000", "10", "365", "0");
        let result = calculator.gross_yield(&investment).unwrap();

        assert_provisional(&result, CalculationReason::ApprovedDemoPv01);
        assert_exact(result.value().unwrap(), 10_000, 1, Currency::Ars);
    }

    #[test]
    fn pv01_conserva_fraccion_exacta() {
        let calculator = ProvisionalCalculator;
        let investment = investment("100", "5", "30", "0");
        let result = calculator.gross_yield(&investment).unwrap();

        assert_provisional(&result, CalculationReason::ApprovedDemoPv01);
        assert_exact(result.value().unwrap(), 3_000, 73, Currency::Ars);
    }

    #[test]
    fn pv01_acepta_ceros_y_conserva_usd() {
        let calculator = ProvisionalCalculator;
        let zero = calculator
            .gross_yield(&investment("0", "0", "0", "0"))
            .unwrap();
        let usd = calculator
            .gross_yield(&usd_investment("1000", "10", "365", "0"))
            .unwrap();

        assert_exact(zero.value().unwrap(), 0, 1, Currency::Ars);
        assert_exact(usd.value().unwrap(), 10_000, 1, Currency::Usd);
    }

    #[test]
    fn pv01_es_determinista() {
        let calculator = ProvisionalCalculator;
        let investment = investment("100", "5", "30", "0");
        let first = calculator.gross_yield(&investment).unwrap();
        let second = calculator.gross_yield(&investment).unwrap();
        let first_value = first.value().unwrap();
        let second_value = second.value().unwrap();

        assert_exact(first_value, 3_000, 73, Currency::Ars);
        assert_eq!(
            first_value.numerator_minor(),
            second_value.numerator_minor()
        );
        assert_eq!(first_value.denominator(), second_value.denominator());
        assert_eq!(first.status(), second.status());
        assert_eq!(first.reason(), second.reason());
    }

    #[test]
    fn pv01_detecta_desbordamiento() {
        let calculator = ProvisionalCalculator;
        let investment = Investment::new(
            "2026-01",
            "92233720368547758.07",
            "184467440737095516",
            "4294967295",
            "0",
        )
        .unwrap();
        let error = calculator.gross_yield(&investment).unwrap_err();

        assert!(matches!(error, DomainError::ArithmeticOverflow { .. }));
    }

    #[test]
    fn pv02_resta_impuestos_absolutos() {
        let calculator = ProvisionalCalculator;
        let investment = investment("1000", "10", "365", "25,50");
        let result = calculator.net_yield(&investment).unwrap();

        assert_provisional(&result, CalculationReason::ApprovedDemoPv02);
        assert_exact(result.value().unwrap(), 7_450, 1, Currency::Ars);
    }

    #[test]
    fn pv02_conserva_neto_negativo_sin_limite() {
        let calculator = ProvisionalCalculator;
        let investment = investment("1000", "10", "365", "200");
        let result = calculator.net_yield(&investment).unwrap();

        assert_exact(result.value().unwrap(), -10_000, 1, Currency::Ars);
    }

    #[test]
    fn pv03_cubre_gastos_en_ars() {
        let calculator = ProvisionalCalculator;
        let result = calculator
            .coverage(Currency::Ars, &net_result(800_000), &totals(550_000, 0))
            .unwrap();

        assert_provisional(&result, CalculationReason::ApprovedDemoPv03);
        assert!(matches!(result.value(), Some(Coverage::Covered)));
    }

    #[test]
    fn pv03_calcula_deficit_en_ars() {
        let calculator = ProvisionalCalculator;
        let result = calculator
            .coverage(Currency::Ars, &net_result(400_000), &totals(550_000, 0))
            .unwrap();

        assert_provisional(&result, CalculationReason::ApprovedDemoPv03);
        match result.value() {
            Some(Coverage::Deficit { amount }) => {
                assert_eq!(amount.cmp_exact(&ars(150_000)).unwrap(), Ordering::Equal);
                assert_eq!(amount.currency(), Currency::Ars);
            }
            other => panic!("unexpected coverage: {:?}", other),
        }
    }

    #[test]
    fn pv03_cubre_gastos_en_usd() {
        let calculator = ProvisionalCalculator;
        let result = calculator
            .coverage(Currency::Usd, &net_result_usd(800_000), &totals(0, 550_000))
            .unwrap();

        assert_provisional(&result, CalculationReason::ApprovedDemoPv03);
        assert!(matches!(result.value(), Some(Coverage::Covered)));
    }

    #[test]
    fn pv03_calcula_deficit_en_usd() {
        let calculator = ProvisionalCalculator;
        let result = calculator
            .coverage(Currency::Usd, &net_result_usd(400_000), &totals(0, 550_000))
            .unwrap();

        assert_provisional(&result, CalculationReason::ApprovedDemoPv03);
        match result.value() {
            Some(Coverage::Deficit { amount }) => {
                assert_eq!(amount.currency(), Currency::Usd);
                assert_eq!(amount.numerator_minor(), 150_000);
                assert_eq!(amount.denominator(), 1);
            }
            other => panic!("unexpected coverage: {:?}", other),
        }
    }

    #[test]
    fn pv03_deficit_conserva_fraccion_exacta() {
        let calculator = ProvisionalCalculator;
        let shared = Service::new("Fraccion", "100,00", 3, None).unwrap();
        let expenses = calculate_service_totals(std::slice::from_ref(&shared)).unwrap();
        let result = calculator
            .coverage(Currency::Ars, &net_result(0), &expenses)
            .unwrap();

        match result.value() {
            Some(Coverage::Deficit { amount }) => {
                assert_eq!(amount.numerator_minor(), 10_000);
                assert_eq!(amount.denominator(), 3);
                assert_eq!(amount.currency(), Currency::Ars);
            }
            other => panic!("unexpected coverage: {:?}", other),
        }
    }

    #[test]
    fn pv03_gastos_en_cero_son_estado_diferenciado() {
        let calculator = ProvisionalCalculator;
        let result = calculator
            .coverage(Currency::Ars, &net_result(0), &totals(0, 0))
            .unwrap();

        assert_provisional(&result, CalculationReason::ApprovedDemoPv03);
        assert!(matches!(result.value(), Some(Coverage::ZeroExpenses)));
    }

    #[test]
    fn pv03_gastos_solo_en_otra_moneda_son_incompatibles() {
        let calculator = ProvisionalCalculator;
        let result = calculator
            .coverage(Currency::Ars, &net_result(800_000), &totals(0, 550_000))
            .unwrap();

        assert_provisional(&result, CalculationReason::ApprovedDemoPv03);
        assert!(matches!(
            result.value(),
            Some(Coverage::IncompatibleCurrencies)
        ));
    }

    #[test]
    fn pv03_gastos_en_ambas_monedas_son_incompatibles() {
        let calculator = ProvisionalCalculator;
        let result = calculator
            .coverage(
                Currency::Ars,
                &net_result(800_000),
                &totals(550_000, 550_000),
            )
            .unwrap();

        assert_provisional(&result, CalculationReason::ApprovedDemoPv03);
        assert!(matches!(
            result.value(),
            Some(Coverage::IncompatibleCurrencies)
        ));
    }

    #[test]
    fn pv03_rechaza_neto_en_otra_moneda_con_gastos_compatibles() {
        let calculator = ProvisionalCalculator;
        let error = calculator
            .coverage(Currency::Ars, &net_result_usd(800_000), &totals(550_000, 0))
            .unwrap_err();

        assert!(matches!(
            error,
            DomainError::CurrencyMismatch {
                expected: "ARS",
                found: "USD",
            }
        ));
    }

    #[test]
    fn pv03_no_emite_veredicto_sin_neto() {
        let calculator = ProvisionalCalculator;
        let net_yield = FinancialResult::Unavailable {
            reason: CalculationReason::PendingNetYield,
        };
        let result = calculator
            .coverage(Currency::Ars, &net_yield, &totals(550_000, 0))
            .unwrap();

        assert_eq!(result.status(), CalculationStatus::Unavailable);
        assert_eq!(result.reason(), Some(CalculationReason::PendingNetYield));
        assert!(result.value().is_none());
    }

    #[test]
    fn pv03_no_convierte_unavailable_en_gastos_cero() {
        let calculator = ProvisionalCalculator;
        let net_yield = FinancialResult::Unavailable {
            reason: CalculationReason::PendingNetYield,
        };
        let result = calculator
            .coverage(Currency::Ars, &net_yield, &totals(0, 0))
            .unwrap();

        assert_eq!(result.status(), CalculationStatus::Unavailable);
        assert!(result.value().is_none());
    }

    #[test]
    fn modulo_no_usa_punto_flotante_binario() {
        let source = include_str!("provisional.rs").as_bytes();
        let narrow: &[u8] = &[0x66, 0x33, 0x32];
        let wide: &[u8] = &[0x66, 0x36, 0x34];

        assert!(!source.windows(narrow.len()).any(|window| window == narrow));
        assert!(!source.windows(wide.len()).any(|window| window == wide));
    }
}
