use chrono::{DateTime, SecondsFormat, Utc};
use serde::{Deserialize, Serialize};

use crate::application::investment_queries::{
    FinancialResults, InvestmentDerived, InvestmentDetails,
};
use crate::application::investment_repository::{InvestmentId, InvestmentRecord};
use crate::application::investment_use_cases::{CreateInvestmentInput, UpdateInvestmentInput};
use crate::application::service_dtos::{ExactMoneyDto, MoneyDto};
use crate::domain::calculations::{
    CalculationReason, CalculationStatus, Coverage, FinancialResult,
};
use crate::domain::error::DomainError;
use crate::domain::money::{Currency, ExactMoney};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CalculationStatusDto {
    Disponible,
    Provisional,
    NoDisponible,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CalculationReasonDto {
    PendingGrossYieldFormula,
    PendingNetYieldTreatment,
    PendingNetYield,
    ApprovedDemoPv01,
    ApprovedDemoPv02,
    ApprovedDemoPv03,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FinancialResultDto<T> {
    pub status: CalculationStatusDto,
    pub reason: Option<CalculationReasonDto>,
    pub value: Option<T>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FinancialRoutesDto<T> {
    pub definitive: FinancialResultDto<T>,
    pub provisional: FinancialResultDto<T>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "outcome", rename_all = "snake_case")]
pub enum CoverageOutcomeDto {
    Covered,
    Deficit { amount: ExactMoneyDto },
    IncompatibleCurrencies,
    ZeroExpenses,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InvestmentRecordDto {
    pub id: String,
    pub month: String,
    pub currency_code: String,
    pub capital: MoneyDto,
    pub tna: String,
    pub term_days: u32,
    pub taxes: MoneyDto,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InvestmentDerivedDto {
    pub monthly_expenses: ExactMoneyDto,
    pub gross_yield: FinancialRoutesDto<ExactMoneyDto>,
    pub net_yield: FinancialRoutesDto<ExactMoneyDto>,
    pub coverage: FinancialRoutesDto<CoverageOutcomeDto>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InvestmentSummaryDto {
    pub record: InvestmentRecordDto,
    pub derived: InvestmentDerivedDto,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateInvestmentInputDto {
    pub month: String,
    pub currency_code: String,
    pub capital: String,
    pub tna: String,
    pub term_days: String,
    pub taxes: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateInvestmentInputDto {
    pub month: String,
    pub capital: String,
    pub tna: String,
    pub term_days: String,
    pub taxes: String,
}

impl From<CalculationStatus> for CalculationStatusDto {
    fn from(value: CalculationStatus) -> Self {
        match value {
            CalculationStatus::Available => Self::Disponible,
            CalculationStatus::Provisional => Self::Provisional,
            CalculationStatus::Unavailable => Self::NoDisponible,
        }
    }
}

impl From<CalculationReason> for CalculationReasonDto {
    fn from(value: CalculationReason) -> Self {
        match value {
            CalculationReason::PendingGrossYieldFormula => Self::PendingGrossYieldFormula,
            CalculationReason::PendingNetYieldTreatment => Self::PendingNetYieldTreatment,
            CalculationReason::PendingNetYield => Self::PendingNetYield,
            CalculationReason::ApprovedDemoPv01 => Self::ApprovedDemoPv01,
            CalculationReason::ApprovedDemoPv02 => Self::ApprovedDemoPv02,
            CalculationReason::ApprovedDemoPv03 => Self::ApprovedDemoPv03,
        }
    }
}

fn financial_result_dto<T, D>(
    value: &FinancialResult<T>,
    map: impl FnOnce(&T) -> D,
) -> FinancialResultDto<D> {
    FinancialResultDto {
        status: CalculationStatusDto::from(value.status()),
        reason: value.reason().map(CalculationReasonDto::from),
        value: value.value().map(map),
    }
}

impl From<&FinancialResults<ExactMoney>> for FinancialRoutesDto<ExactMoneyDto> {
    fn from(value: &FinancialResults<ExactMoney>) -> Self {
        Self {
            definitive: financial_result_dto(&value.definitive, |value| ExactMoneyDto::from(value)),
            provisional: financial_result_dto(&value.provisional, |value| {
                ExactMoneyDto::from(value)
            }),
        }
    }
}

impl From<&Coverage> for CoverageOutcomeDto {
    fn from(value: &Coverage) -> Self {
        match value {
            Coverage::Covered => Self::Covered,
            Coverage::Deficit { amount } => Self::Deficit {
                amount: ExactMoneyDto::from(amount),
            },
            Coverage::IncompatibleCurrencies => Self::IncompatibleCurrencies,
            Coverage::ZeroExpenses => Self::ZeroExpenses,
        }
    }
}

impl From<&FinancialResults<Coverage>> for FinancialRoutesDto<CoverageOutcomeDto> {
    fn from(value: &FinancialResults<Coverage>) -> Self {
        Self {
            definitive: financial_result_dto(&value.definitive, |value| {
                CoverageOutcomeDto::from(value)
            }),
            provisional: financial_result_dto(&value.provisional, |value| {
                CoverageOutcomeDto::from(value)
            }),
        }
    }
}

impl TryFrom<&InvestmentRecord> for InvestmentRecordDto {
    type Error = DomainError;

    fn try_from(value: &InvestmentRecord) -> Result<Self, Self::Error> {
        let investment = value.investment();

        Ok(Self {
            id: value.id().to_string(),
            month: investment.month().code(),
            currency_code: investment.currency().code().to_owned(),
            capital: MoneyDto::from(investment.capital()),
            tna: format_tna(investment.tna_basis_points()),
            term_days: investment.term_days(),
            taxes: MoneyDto::from(investment.taxes()),
            created_at: format_timestamp(value.created_at()),
            updated_at: format_timestamp(value.updated_at()),
        })
    }
}

impl From<&InvestmentDerived> for InvestmentDerivedDto {
    fn from(value: &InvestmentDerived) -> Self {
        Self {
            monthly_expenses: ExactMoneyDto::from(&value.monthly_expenses),
            gross_yield: FinancialRoutesDto::from(&value.gross_yield),
            net_yield: FinancialRoutesDto::from(&value.net_yield),
            coverage: FinancialRoutesDto::from(&value.coverage),
        }
    }
}

impl TryFrom<&InvestmentDetails> for InvestmentSummaryDto {
    type Error = DomainError;

    fn try_from(value: &InvestmentDetails) -> Result<Self, Self::Error> {
        Ok(Self {
            record: InvestmentRecordDto::try_from(&value.record)?,
            derived: InvestmentDerivedDto::from(&value.derived),
        })
    }
}

impl TryFrom<CreateInvestmentInputDto> for CreateInvestmentInput {
    type Error = DomainError;

    fn try_from(value: CreateInvestmentInputDto) -> Result<Self, Self::Error> {
        let currency = parse_currency_code(&value.currency_code)?;

        Ok(Self {
            month: value.month,
            currency,
            capital: value.capital,
            tna: value.tna,
            term_days: value.term_days,
            taxes: value.taxes,
        })
    }
}

impl TryFrom<UpdateInvestmentInputDto> for UpdateInvestmentInput {
    type Error = DomainError;

    fn try_from(value: UpdateInvestmentInputDto) -> Result<Self, Self::Error> {
        Ok(Self {
            month: value.month,
            capital: value.capital,
            tna: value.tna,
            term_days: value.term_days,
            taxes: value.taxes,
        })
    }
}

pub fn parse_investment_id(value: &str) -> Result<InvestmentId, DomainError> {
    InvestmentId::parse(value).ok_or_else(|| {
        DomainError::validation("id", "El identificador de la inversión no es válido.")
    })
}

fn parse_currency_code(value: &str) -> Result<Currency, DomainError> {
    Currency::from_code(value)
        .ok_or_else(|| DomainError::validation("currencyCode", "La moneda debe ser ARS o USD."))
}

fn format_tna(basis_points: u64) -> String {
    format!("{},{:02}", basis_points / 100, basis_points % 100)
}

fn format_timestamp(value: DateTime<Utc>) -> String {
    value.to_rfc3339_opts(SecondsFormat::Millis, true)
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;
    use crate::application::investment_repository::InvestmentRepository;
    use crate::domain::calculations::CalculationReason;
    use crate::domain::investment::Investment;
    use crate::domain::money::Money;
    use crate::infrastructure::database;
    use crate::infrastructure::investment_repository::SqliteInvestmentRepository;

    const REPOSITORY: SqliteInvestmentRepository = SqliteInvestmentRepository;

    #[test]
    fn serializa_registro_en_camel_case_con_importes_y_tna_como_cadenas() {
        let connection = database::open_in_memory().unwrap();
        let investment = Investment::new_with_currency(
            "2026-03",
            "1000000,00",
            Currency::Usd,
            "25,50",
            "30",
            "500,00",
        )
        .unwrap();
        let record = REPOSITORY.insert(&connection, &investment).unwrap();

        let dto = InvestmentRecordDto::try_from(&record).unwrap();

        assert_eq!(
            serde_json::to_value(&dto).unwrap(),
            json!({
                "id": record.id().to_string(),
                "month": "2026-03",
                "currencyCode": "USD",
                "capital": {
                    "minorUnits": "100000000",
                    "currencyCode": "USD"
                },
                "tna": "25,50",
                "termDays": 30,
                "taxes": {
                    "minorUnits": "50000",
                    "currencyCode": "USD"
                },
                "createdAt": record.created_at().to_rfc3339_opts(SecondsFormat::Millis, true),
                "updatedAt": record.updated_at().to_rfc3339_opts(SecondsFormat::Millis, true)
            })
        );
    }

    #[test]
    fn deserializa_entrada_de_alta_en_camel_case() {
        let dto: CreateInvestmentInputDto = serde_json::from_value(json!({
            "month": "2026-03",
            "currencyCode": "USD",
            "capital": "1000.00",
            "tna": "25.50",
            "termDays": "15",
            "taxes": "0.00"
        }))
        .unwrap();

        let input = CreateInvestmentInput::try_from(dto).unwrap();

        assert_eq!(input.month, "2026-03");
        assert_eq!(input.currency, Currency::Usd);
        assert_eq!(input.capital, "1000.00");
        assert_eq!(input.tna, "25.50");
        assert_eq!(input.term_days, "15");
        assert_eq!(input.taxes, "0.00");
    }

    #[test]
    fn rechaza_moneda_no_admitida_con_campo_currency_code() {
        let dto = CreateInvestmentInputDto {
            month: "2026-03".to_owned(),
            currency_code: "EUR".to_owned(),
            capital: "100,00".to_owned(),
            tna: "25".to_owned(),
            term_days: "30".to_owned(),
            taxes: "0".to_owned(),
        };

        let error = CreateInvestmentInput::try_from(dto).unwrap_err();

        assert!(matches!(
            error,
            DomainError::Validation {
                field: "currencyCode",
                ..
            }
        ));
    }

    #[test]
    fn serializa_derivados_con_estados_motivos_y_fracciones_exactas() {
        let ars = Currency::Ars;
        let derived = InvestmentDerived {
            monthly_expenses: ExactMoney::from_money(Money::from_minor_units(10_000, ars)),
            gross_yield: FinancialResults {
                definitive: FinancialResult::Unavailable {
                    reason: CalculationReason::PendingGrossYieldFormula,
                },
                provisional: FinancialResult::Provisional {
                    value: ExactMoney::from_money(Money::from_minor_units(36_500, ars)),
                    reason: CalculationReason::ApprovedDemoPv01,
                },
            },
            net_yield: FinancialResults {
                definitive: FinancialResult::Unavailable {
                    reason: CalculationReason::PendingNetYieldTreatment,
                },
                provisional: FinancialResult::Provisional {
                    value: ExactMoney::from_money(Money::from_minor_units(30_000, ars)),
                    reason: CalculationReason::ApprovedDemoPv02,
                },
            },
            coverage: FinancialResults {
                definitive: FinancialResult::Unavailable {
                    reason: CalculationReason::PendingNetYield,
                },
                provisional: FinancialResult::Provisional {
                    value: Coverage::Deficit {
                        amount: ExactMoney::from_money(Money::from_minor_units(20_000, ars)),
                    },
                    reason: CalculationReason::ApprovedDemoPv03,
                },
            },
        };

        let dto = InvestmentDerivedDto::from(&derived);

        assert_eq!(
            serde_json::to_value(&dto).unwrap(),
            json!({
                "monthlyExpenses": {
                    "numeratorMinor": "10000",
                    "denominator": "1",
                    "currencyCode": "ARS"
                },
                "grossYield": {
                    "definitive": {
                        "status": "no_disponible",
                        "reason": "pending_gross_yield_formula",
                        "value": null
                    },
                    "provisional": {
                        "status": "provisional",
                        "reason": "approved_demo_pv01",
                        "value": {
                            "numeratorMinor": "36500",
                            "denominator": "1",
                            "currencyCode": "ARS"
                        }
                    }
                },
                "netYield": {
                    "definitive": {
                        "status": "no_disponible",
                        "reason": "pending_net_yield_treatment",
                        "value": null
                    },
                    "provisional": {
                        "status": "provisional",
                        "reason": "approved_demo_pv02",
                        "value": {
                            "numeratorMinor": "30000",
                            "denominator": "1",
                            "currencyCode": "ARS"
                        }
                    }
                },
                "coverage": {
                    "definitive": {
                        "status": "no_disponible",
                        "reason": "pending_net_yield",
                        "value": null
                    },
                    "provisional": {
                        "status": "provisional",
                        "reason": "approved_demo_pv03",
                        "value": {
                            "outcome": "deficit",
                            "amount": {
                                "numeratorMinor": "20000",
                                "denominator": "1",
                                "currencyCode": "ARS"
                            }
                        }
                    }
                }
            })
        );
    }

    #[test]
    fn serializa_estados_de_cobertura_sin_veredicto() {
        assert_eq!(
            serde_json::to_value(CoverageOutcomeDto::IncompatibleCurrencies).unwrap(),
            json!({ "outcome": "incompatible_currencies" })
        );
        assert_eq!(
            serde_json::to_value(CoverageOutcomeDto::ZeroExpenses).unwrap(),
            json!({ "outcome": "zero_expenses" })
        );
        assert_eq!(
            CoverageOutcomeDto::from(&Coverage::IncompatibleCurrencies),
            CoverageOutcomeDto::IncompatibleCurrencies
        );
        assert_eq!(
            CoverageOutcomeDto::from(&Coverage::ZeroExpenses),
            CoverageOutcomeDto::ZeroExpenses
        );
    }

    #[test]
    fn identificador_malformado_devuelve_validacion_con_campo_id() {
        let error = parse_investment_id("no-uuid").unwrap_err();

        assert!(matches!(error, DomainError::Validation { field: "id", .. }));
    }
}
