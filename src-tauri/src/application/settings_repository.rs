use std::fmt;

use chrono::{DateTime, Utc};

use crate::domain::error::DomainError;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SettingKey(String);

impl SettingKey {
    pub fn parse(value: &str) -> Result<Self, DomainError> {
        if value.is_empty() {
            return Err(DomainError::validation(
                "key",
                "la clave de la preferencia no puede estar vacía",
            ));
        }

        Ok(Self(value.to_owned()))
    }

    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

impl fmt::Display for SettingKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SettingRecord {
    pub(crate) key: SettingKey,
    pub(crate) value: String,
    pub(crate) updated_at: DateTime<Utc>,
}

impl SettingRecord {
    pub fn key(&self) -> &SettingKey {
        &self.key
    }

    pub fn value(&self) -> &str {
        self.value.as_str()
    }

    pub fn updated_at(&self) -> DateTime<Utc> {
        self.updated_at
    }
}

pub trait SettingsRepository {
    fn set(
        &self,
        connection: &rusqlite::Connection,
        key: &SettingKey,
        value: &str,
    ) -> Result<SettingRecord, DomainError>;

    fn get(
        &self,
        connection: &rusqlite::Connection,
        key: &SettingKey,
    ) -> Result<Option<SettingRecord>, DomainError>;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn setting_key_rechaza_clave_vacia() {
        let error = SettingKey::parse("").unwrap_err();

        assert!(matches!(
            error,
            DomainError::Validation { field: "key", .. }
        ));
    }

    #[test]
    fn setting_key_acepta_clave_no_vacia() {
        let key = SettingKey::parse("theme").unwrap();

        assert_eq!(key.as_str(), "theme");
        assert_eq!(key.to_string(), "theme");
    }
}
