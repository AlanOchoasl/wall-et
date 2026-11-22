use chrono::{SecondsFormat, Utc};
use rusqlite::{Connection, params};

use crate::domain::error::DomainError;
use crate::infrastructure::database::sqlite_error;

pub struct Migration {
    pub version: i64,
    pub sql: &'static str,
}

pub const MIGRATIONS: &[Migration] = &[Migration {
    version: 1,
    sql: include_str!("migrations/0001_initial.sql"),
}];

pub fn latest_version() -> i64 {
    MIGRATIONS
        .last()
        .map(|migration| migration.version)
        .unwrap_or(0)
}

pub fn apply(connection: &mut Connection) -> Result<(), DomainError> {
    connection
        .execute_batch(
            "CREATE TABLE IF NOT EXISTS schema_migrations (
                version INTEGER PRIMARY KEY,
                applied_at TEXT NOT NULL
            );",
        )
        .map_err(sqlite_error)?;

    for migration in MIGRATIONS {
        if is_applied(connection, migration.version)? {
            continue;
        }

        let transaction = connection.transaction().map_err(sqlite_error)?;
        transaction
            .execute_batch(migration.sql)
            .map_err(sqlite_error)?;
        transaction
            .execute(
                "INSERT INTO schema_migrations (version, applied_at) VALUES (?1, ?2)",
                params![
                    migration.version,
                    Utc::now().to_rfc3339_opts(SecondsFormat::Millis, true)
                ],
            )
            .map_err(sqlite_error)?;
        transaction.commit().map_err(sqlite_error)?;
    }

    Ok(())
}

pub fn current_version(connection: &Connection) -> Result<i64, DomainError> {
    connection
        .query_row(
            "SELECT COALESCE(MAX(version), 0) FROM schema_migrations",
            [],
            |row| row.get(0),
        )
        .map_err(sqlite_error)
}

fn is_applied(connection: &Connection, version: i64) -> Result<bool, DomainError> {
    connection
        .query_row(
            "SELECT EXISTS(SELECT 1 FROM schema_migrations WHERE version = ?1)",
            [version],
            |row| row.get(0),
        )
        .map_err(sqlite_error)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn aplicar_migraciones_dos_veces_no_duplica_ni_falla() {
        let mut connection = Connection::open_in_memory().unwrap();

        apply(&mut connection).unwrap();
        apply(&mut connection).unwrap();

        let applied: i64 = connection
            .query_row("SELECT COUNT(*) FROM schema_migrations", [], |row| {
                row.get(0)
            })
            .unwrap();

        assert_eq!(applied, i64::try_from(MIGRATIONS.len()).unwrap());
        assert_eq!(current_version(&connection).unwrap(), latest_version());
    }

    #[test]
    fn crea_las_tablas_de_la_primera_version() {
        let mut connection = Connection::open_in_memory().unwrap();
        apply(&mut connection).unwrap();

        for table in ["service", "investment", "app_setting"] {
            let exists: bool = connection
                .query_row(
                    "SELECT EXISTS(SELECT 1 FROM sqlite_master WHERE type = 'table' AND name = ?1)",
                    [table],
                    |row| row.get(0),
                )
                .unwrap();

            assert!(exists, "falta la tabla {table}");
        }
    }
}
