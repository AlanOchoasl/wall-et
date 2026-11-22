use std::path::Path;

use serde::{Deserialize, Serialize};

use crate::domain::error::DomainError;
use crate::infrastructure::{database, migrations};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppStatusDto {
    pub app_name: String,
    pub version: String,
    pub database_ready: bool,
    pub schema_version: i64,
    pub currency_code: String,
}

pub fn get(database_path: &Path) -> Result<AppStatusDto, DomainError> {
    let connection = database::open(database_path)?;
    let schema_version = migrations::current_version(&connection)?;

    Ok(AppStatusDto {
        app_name: "App de Finanzas".to_string(),
        version: env!("CARGO_PKG_VERSION").to_string(),
        database_ready: true,
        schema_version,
        currency_code: "ARS".to_string(),
    })
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;

    #[test]
    fn informa_estado_con_esquema_aplicado() {
        let directory = tempfile::tempdir().unwrap();
        let path = database::resolve_path(directory.path());

        let status = get(&path).unwrap();

        assert_eq!(status.app_name, "App de Finanzas");
        assert_eq!(status.currency_code, "ARS");
        assert!(status.database_ready);
        assert_eq!(status.schema_version, migrations::latest_version());
        assert!(path.exists());
    }

    #[test]
    fn serializa_en_camel_case() {
        let status = AppStatusDto {
            app_name: "App de Finanzas".to_string(),
            version: "0.1.0".to_string(),
            database_ready: true,
            schema_version: 1,
            currency_code: "ARS".to_string(),
        };

        assert_eq!(
            serde_json::to_value(&status).unwrap(),
            json!({
                "appName": "App de Finanzas",
                "version": "0.1.0",
                "databaseReady": true,
                "schemaVersion": 1,
                "currencyCode": "ARS"
            })
        );
    }
}
