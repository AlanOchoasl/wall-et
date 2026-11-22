use thiserror::Error;

#[derive(Debug, Error)]
pub enum DomainError {
    #[error("{message}")]
    Validation {
        field: &'static str,
        message: String,
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

    pub fn persistence(message: impl Into<String>) -> Self {
        Self::Persistence(message.into())
    }

    pub fn code(&self) -> &'static str {
        match self {
            Self::Validation { .. } => "validation",
            Self::NotFound => "not_found",
            Self::Persistence(_) => "persistence",
            Self::Internal(_) => "internal",
        }
    }
}
