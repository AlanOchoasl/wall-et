use serde::{Deserialize, Serialize};

use crate::domain::error::DomainError;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppError {
    pub code: String,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub field: Option<String>,
}

impl AppError {
    pub fn new(code: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            code: code.into(),
            message: message.into(),
            field: None,
        }
    }

    pub fn with_field(mut self, field: impl Into<String>) -> Self {
        self.field = Some(field.into());
        self
    }
}

impl From<DomainError> for AppError {
    fn from(error: DomainError) -> Self {
        match error {
            DomainError::Validation { field, message } => {
                Self::new("validation", message).with_field(field)
            }
            DomainError::CurrencyMismatch { expected, found } => Self::new(
                "currency_mismatch",
                format!(
                    "Las monedas no son compatibles: se esperaba {expected} y se recibió {found}."
                ),
            )
            .with_field("currency"),
            DomainError::ArithmeticOverflow { field, operation } => Self::new(
                "arithmetic_overflow",
                format!("La operación {operation} superó el rango permitido."),
            )
            .with_field(field),
            DomainError::NotFound => {
                Self::new("not_found", "No se encontró el recurso solicitado.")
            }
            DomainError::Persistence(message) => Self::new(
                "persistence",
                format!("No se pudo acceder a los datos: {message}"),
            ),
            DomainError::Internal(message) => Self::new("internal", message),
        }
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;

    #[test]
    fn serializa_error_con_campo_en_camel_case() {
        let error = AppError::from(DomainError::validation(
            "totalCost",
            "El costo no es válido.",
        ));

        assert_eq!(
            serde_json::to_value(&error).unwrap(),
            json!({
                "code": "validation",
                "message": "El costo no es válido.",
                "field": "totalCost"
            })
        );
    }

    #[test]
    fn omite_el_campo_cuando_no_existe() {
        let error = AppError::new("persistence", "No se pudo acceder a los datos.");

        assert_eq!(
            serde_json::to_value(&error).unwrap(),
            json!({
                "code": "persistence",
                "message": "No se pudo acceder a los datos."
            })
        );
    }

    #[test]
    fn deserializa_un_error_de_la_interfaz() {
        let error: AppError = serde_json::from_value(json!({
            "code": "validation",
            "message": "La cantidad debe ser mayor o igual a 1.",
            "field": "sharedPeopleCount"
        }))
        .unwrap();

        assert_eq!(error.code, "validation");
        assert_eq!(error.field.as_deref(), Some("sharedPeopleCount"));
    }

    #[test]
    fn mapea_moneda_incompatible_con_campo() {
        let error = AppError::from(DomainError::currency_mismatch("ARS", "USD"));

        assert_eq!(error.code, "currency_mismatch");
        assert_eq!(error.field.as_deref(), Some("currency"));
        assert_eq!(
            error.message,
            "Las monedas no son compatibles: se esperaba ARS y se recibió USD."
        );
    }

    #[test]
    fn mapea_desbordamiento_con_campo() {
        let error = AppError::from(DomainError::arithmetic_overflow("amount", "suma"));

        assert_eq!(error.code, "arithmetic_overflow");
        assert_eq!(error.field.as_deref(), Some("amount"));
        assert_eq!(
            error.message,
            "La operación suma superó el rango permitido."
        );
    }
}
