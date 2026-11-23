use thiserror::Error;

#[derive(Debug, Error)]
pub enum DomainError {
    #[error("{message}")]
    Validation {
        field: &'static str,
        message: String,
    },
    #[error("las monedas no son compatibles: se esperaba {expected} y se recibió {found}")]
    CurrencyMismatch {
        expected: &'static str,
        found: &'static str,
    },
    #[error("desbordamiento aritmético en {operation} para {field}")]
    ArithmeticOverflow {
        field: &'static str,
        operation: &'static str,
    },
    #[error("no se encontró el recurso solicitado")]
    NotFound,
    #[error("error de persistencia: {0}")]
    Persistence(String),
    #[error("error interno: {0}")]
    Internal(String),
}

impl DomainError {
    pub fn validation(field: &'static str, message: impl Into<String>) -> Self {
        Self::Validation {
            field,
            message: message.into(),
        }
    }

    pub fn currency_mismatch(expected: &'static str, found: &'static str) -> Self {
        Self::CurrencyMismatch { expected, found }
    }

    pub fn arithmetic_overflow(field: &'static str, operation: &'static str) -> Self {
        Self::ArithmeticOverflow { field, operation }
    }

    pub fn persistence(message: impl Into<String>) -> Self {
        Self::Persistence(message.into())
    }

    pub fn code(&self) -> &'static str {
        match self {
            Self::Validation { .. } => "validation",
            Self::CurrencyMismatch { .. } => "currency_mismatch",
            Self::ArithmeticOverflow { .. } => "arithmetic_overflow",
            Self::NotFound => "not_found",
            Self::Persistence(_) => "persistence",
            Self::Internal(_) => "internal",
        }
    }
}
