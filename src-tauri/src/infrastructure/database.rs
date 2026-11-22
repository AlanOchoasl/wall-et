use std::fs;
use std::path::{Path, PathBuf};

use rusqlite::Connection;

use crate::domain::error::DomainError;
use crate::infrastructure::migrations;

pub const DATABASE_FILE_NAME: &str = "wall-et.sqlite3";

pub fn resolve_path(data_dir: &Path) -> PathBuf {
    data_dir.join(DATABASE_FILE_NAME)
}

pub fn open(path: &Path) -> Result<Connection, DomainError> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|error| {
            DomainError::persistence(format!("no se pudo crear el directorio de datos: {error}"))
        })?;
    }

    let mut connection = Connection::open(path).map_err(sqlite_error)?;
    configure(&connection, true)?;
    migrations::apply(&mut connection)?;

    Ok(connection)
}

pub fn open_in_memory() -> Result<Connection, DomainError> {
    let mut connection = Connection::open_in_memory().map_err(sqlite_error)?;
    configure(&connection, false)?;
    migrations::apply(&mut connection)?;

    Ok(connection)
}

fn configure(connection: &Connection, persistent: bool) -> Result<(), DomainError> {
    connection
        .execute_batch("PRAGMA foreign_keys = ON;")
        .map_err(sqlite_error)?;

    if persistent {
        connection
            .pragma_update(None, "journal_mode", "WAL")
            .map_err(sqlite_error)?;
    }

    Ok(())
}

pub fn sqlite_error(error: rusqlite::Error) -> DomainError {
    DomainError::persistence(error.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn abre_una_base_en_memoria_y_ejecuta_una_consulta() {
        let connection = open_in_memory().unwrap();

        let value: i64 = connection
            .query_row("SELECT 1", [], |row| row.get(0))
            .unwrap();

        assert_eq!(value, 1);
    }

    #[test]
    fn resuelve_la_ruta_dentro_del_directorio_de_datos_y_crea_el_archivo() {
        let directory = tempfile::tempdir().unwrap();
        let path = resolve_path(directory.path());

        assert_eq!(path.parent(), Some(directory.path()));

        let connection = open(&path).unwrap();
        drop(connection);

        assert!(path.exists());
    }
}
