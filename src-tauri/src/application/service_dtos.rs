use chrono::{DateTime, SecondsFormat, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::application::service_repository::{ServiceId, ServiceRecord};
use crate::application::service_use_cases::{CreateServiceInput, UpdateServiceInput};
use crate::application::service_use_cases::{
    DeleteServiceOutcome, FreezeServiceOutcome, ReactivateServiceOutcome,
};
use crate::domain::error::DomainError;
use crate::domain::money::{Currency, ExactMoney, Money};
use crate::domain::service::ServiceStatus;
use crate::domain::service_totals::ServiceTotals;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MoneyDto {
    pub minor_units: String,
    pub currency_code: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExactMoneyDto {
    pub numerator_minor: String,
    pub denominator: String,
    pub currency_code: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ServiceStatusDto {
    Active,
    Frozen,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ServiceRecordDto {
    pub id: String,
    pub name: String,
    pub total_cost: MoneyDto,
    pub my_share: ExactMoneyDto,
    pub shared_people_count: u32,
    pub notes: Option<String>,
    pub status: ServiceStatusDto,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateServiceInputDto {
    pub name: String,
    pub total_cost: String,
    pub currency_code: String,
    pub shared_people_count: Value,
    #[serde(default)]
    pub notes: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateServiceInputDto {
    pub name: String,
    pub total_cost: String,
    pub shared_people_count: Value,
    #[serde(default)]
    pub notes: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DeleteServiceOutcomeDto {
    Deleted,
    Cancelled,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ServiceMutationOutcomeDto {
    Frozen,
    AlreadyFrozen,
    Reactivated,
    AlreadyActive,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeleteServiceResultDto {
    pub outcome: DeleteServiceOutcomeDto,
    pub id: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ServiceMutationResultDto {
    pub outcome: ServiceMutationOutcomeDto,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CurrencyServiceTotalsDto {
    pub total_cost: MoneyDto,
    pub user_share: ExactMoneyDto,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ServiceTotalsDto {
    pub ars: CurrencyServiceTotalsDto,
    pub usd: CurrencyServiceTotalsDto,
}

impl From<Money> for MoneyDto {
    fn from(value: Money) -> Self {
        Self {
            minor_units: value.minor_units().to_string(),
            currency_code: value.currency().code().to_owned(),
        }
    }
}

impl From<&ExactMoney> for ExactMoneyDto {
    fn from(value: &ExactMoney) -> Self {
        Self {
            numerator_minor: value.numerator_minor().to_string(),
            denominator: value.denominator().to_string(),
            currency_code: value.currency().code().to_owned(),
        }
    }
}

impl From<ServiceStatus> for ServiceStatusDto {
    fn from(value: ServiceStatus) -> Self {
        match value {
            ServiceStatus::Active => Self::Active,
            ServiceStatus::Frozen => Self::Frozen,
        }
    }
}

impl From<DeleteServiceOutcome> for DeleteServiceOutcomeDto {
    fn from(value: DeleteServiceOutcome) -> Self {
        match value {
            DeleteServiceOutcome::Deleted => Self::Deleted,
            DeleteServiceOutcome::Cancelled => Self::Cancelled,
        }
    }
}

impl From<&FreezeServiceOutcome> for ServiceMutationOutcomeDto {
    fn from(value: &FreezeServiceOutcome) -> Self {
        match value {
            FreezeServiceOutcome::Frozen(_) => Self::Frozen,
            FreezeServiceOutcome::AlreadyFrozen(_) => Self::AlreadyFrozen,
        }
    }
}

impl From<&ReactivateServiceOutcome> for ServiceMutationOutcomeDto {
    fn from(value: &ReactivateServiceOutcome) -> Self {
        match value {
            ReactivateServiceOutcome::Reactivated(_) => Self::Reactivated,
            ReactivateServiceOutcome::AlreadyActive(_) => Self::AlreadyActive,
        }
    }
}

impl TryFrom<&ServiceRecord> for ServiceRecordDto {
    type Error = DomainError;

    fn try_from(value: &ServiceRecord) -> Result<Self, Self::Error> {
        let service = value.service();

        Ok(Self {
            id: value.id().to_string(),
            name: service.name().to_owned(),
            total_cost: MoneyDto::from(service.total_cost()),
            my_share: ExactMoneyDto::from(&service.my_share()?),
            shared_people_count: service.shared_people_count().get(),
            notes: service.notes().map(|notes| notes.to_owned()),
            status: ServiceStatusDto::from(service.status()),
            created_at: format_timestamp(value.created_at()),
            updated_at: format_timestamp(value.updated_at()),
        })
    }
}

impl TryFrom<&ServiceTotals> for ServiceTotalsDto {
    type Error = DomainError;

    fn try_from(value: &ServiceTotals) -> Result<Self, Self::Error> {
        Ok(Self {
            ars: CurrencyServiceTotalsDto {
                total_cost: MoneyDto::from(value.ars().total_cost()),
                user_share: ExactMoneyDto::from(value.ars().user_share()),
            },
            usd: CurrencyServiceTotalsDto {
                total_cost: MoneyDto::from(value.usd().total_cost()),
                user_share: ExactMoneyDto::from(value.usd().user_share()),
            },
        })
    }
}

impl TryFrom<CreateServiceInputDto> for CreateServiceInput {
    type Error = DomainError;

    fn try_from(value: CreateServiceInputDto) -> Result<Self, Self::Error> {
        let currency = parse_currency_code(&value.currency_code)?;
        let shared_people_count = parse_shared_people_count(&value.shared_people_count)?;

        Ok(Self {
            name: value.name,
            total_cost: value.total_cost,
            currency,
            shared_people_count,
            notes: value.notes,
        })
    }
}

impl TryFrom<UpdateServiceInputDto> for UpdateServiceInput {
    type Error = DomainError;

    fn try_from(value: UpdateServiceInputDto) -> Result<Self, Self::Error> {
        let shared_people_count = parse_shared_people_count(&value.shared_people_count)?;

        Ok(Self {
            name: value.name,
            total_cost: value.total_cost,
            shared_people_count,
            notes: value.notes,
        })
    }
}

fn parse_shared_people_count(value: &Value) -> Result<u32, DomainError> {
    value
        .as_u64()
        .and_then(|count| u32::try_from(count).ok())
        .ok_or_else(|| {
            DomainError::validation(
                "sharedPeopleCount",
                "La cantidad de personas debe ser un entero mayor o igual a 1.",
            )
        })
}

pub fn parse_service_id(value: &str) -> Result<ServiceId, DomainError> {
    ServiceId::parse(value)
        .ok_or_else(|| DomainError::validation("id", "El identificador del servicio no es válido."))
}

fn parse_currency_code(value: &str) -> Result<Currency, DomainError> {
    Currency::from_code(value)
        .ok_or_else(|| DomainError::validation("currencyCode", "La moneda debe ser ARS o USD."))
}

fn format_timestamp(value: DateTime<Utc>) -> String {
    value.to_rfc3339_opts(SecondsFormat::Millis, true)
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;
    use crate::application::service_repository::ServiceRepository;
    use crate::domain::money::Currency;
    use crate::domain::service::Service;
    use crate::domain::service_totals::calculate_service_totals;
    use crate::infrastructure::database;
    use crate::infrastructure::service_repository::SqliteServiceRepository;

    #[test]
    fn serializa_registro_en_camel_case_con_importes_como_cadenas() {
        let connection = database::open_in_memory().unwrap();
        let repository = SqliteServiceRepository;
        let input = CreateServiceInput {
            name: "Luz".to_owned(),
            total_cost: "10000,00".to_owned(),
            currency: Currency::Ars,
            shared_people_count: 4,
            notes: None,
        };
        let service = Service::new_with_currency(
            input.name.clone(),
            &input.total_cost,
            input.currency,
            input.shared_people_count,
            input.notes.clone(),
        )
        .unwrap();
        let record = repository.insert(&connection, &service).unwrap();

        let dto = ServiceRecordDto::try_from(&record).unwrap();

        assert_eq!(
            serde_json::to_value(&dto).unwrap(),
            json!({
                "id": record.id().to_string(),
                "name": "Luz",
                "totalCost": {
                    "minorUnits": "1000000",
                    "currencyCode": "ARS"
                },
                "myShare": {
                    "numeratorMinor": "250000",
                    "denominator": "1",
                    "currencyCode": "ARS"
                },
                "sharedPeopleCount": 4,
                "notes": null,
                "status": "active",
                "createdAt": record.created_at().to_rfc3339_opts(SecondsFormat::Millis, true),
                "updatedAt": record.updated_at().to_rfc3339_opts(SecondsFormat::Millis, true)
            })
        );
    }

    #[test]
    fn serializa_fraccion_exacta_sin_redondeo() {
        let connection = database::open_in_memory().unwrap();
        let repository = SqliteServiceRepository;
        let service = Service::new("A", "100,00", 3, None).unwrap();
        let record = repository.insert(&connection, &service).unwrap();

        let dto = ServiceRecordDto::try_from(&record).unwrap();

        assert_eq!(dto.my_share.numerator_minor, "10000");
        assert_eq!(dto.my_share.denominator, "3");
    }

    #[test]
    fn deserializa_entrada_de_alta_en_camel_case() {
        let dto: CreateServiceInputDto = serde_json::from_value(json!({
            "name": "Netflix",
            "totalCost": "15.99",
            "currencyCode": "USD",
            "sharedPeopleCount": 2,
            "notes": "familiar"
        }))
        .unwrap();

        let input = CreateServiceInput::try_from(dto).unwrap();

        assert_eq!(input.name, "Netflix");
        assert_eq!(input.total_cost, "15.99");
        assert_eq!(input.currency, Currency::Usd);
        assert_eq!(input.shared_people_count, 2);
        assert_eq!(input.notes.as_deref(), Some("familiar"));
    }

    #[test]
    fn rechaza_moneda_no_admitida_con_campo_currency_code() {
        let dto = CreateServiceInputDto {
            name: "Luz".to_owned(),
            total_cost: "100,00".to_owned(),
            currency_code: "EUR".to_owned(),
            shared_people_count: json!(1),
            notes: None,
        };

        let error = CreateServiceInput::try_from(dto).unwrap_err();

        assert!(matches!(
            error,
            DomainError::Validation {
                field: "currencyCode",
                ..
            }
        ));
    }

    #[test]
    fn rechaza_cantidad_de_personas_no_entera_o_negativa_con_campo() {
        let valid = CreateServiceInputDto {
            name: "Luz".to_owned(),
            total_cost: "100,00".to_owned(),
            currency_code: "ARS".to_owned(),
            shared_people_count: json!(1),
            notes: None,
        };
        let negative = CreateServiceInputDto {
            shared_people_count: json!(-1),
            ..valid.clone()
        };
        let non_integer = CreateServiceInputDto {
            shared_people_count: json!(1.5),
            ..valid
        };

        for error in [
            CreateServiceInput::try_from(negative).unwrap_err(),
            CreateServiceInput::try_from(non_integer).unwrap_err(),
        ] {
            assert!(matches!(
                error,
                DomainError::Validation {
                    field: "sharedPeopleCount",
                    ..
                }
            ));
        }
    }

    #[test]
    fn serializa_totales_separados_por_moneda() {
        let ars = Service::new("A", "100,00", 3, None).unwrap();
        let usd = Service::new_with_currency("B", "20,00", Currency::Usd, 2, None).unwrap();
        let totals = calculate_service_totals(&[ars, usd]).unwrap();

        let dto = ServiceTotalsDto::try_from(&totals).unwrap();

        assert_eq!(
            serde_json::to_value(&dto).unwrap(),
            json!({
                "ars": {
                    "totalCost": { "minorUnits": "10000", "currencyCode": "ARS" },
                    "userShare": {
                        "numeratorMinor": "10000",
                        "denominator": "3",
                        "currencyCode": "ARS"
                    }
                },
                "usd": {
                    "totalCost": { "minorUnits": "2000", "currencyCode": "USD" },
                    "userShare": {
                        "numeratorMinor": "1000",
                        "denominator": "1",
                        "currencyCode": "USD"
                    }
                }
            })
        );
    }

    #[test]
    fn serializa_resultados_en_snake_case() {
        assert_eq!(
            serde_json::to_value(DeleteServiceOutcomeDto::Cancelled).unwrap(),
            json!("cancelled")
        );
        assert_eq!(
            serde_json::to_value(ServiceMutationOutcomeDto::AlreadyFrozen).unwrap(),
            json!("already_frozen")
        );
    }
}
