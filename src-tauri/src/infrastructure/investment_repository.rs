use chrono::{DateTime, Duration, SecondsFormat, Utc};
use rusqlite::{Connection, OptionalExtension, Row, params};

use crate::application::investment_repository::{
    InvestmentId, InvestmentRecord, InvestmentRepository,
};
use crate::domain::error::DomainError;
use crate::domain::investment::{Investment, Month};
use crate::domain::money::{Currency, Money};
use crate::infrastructure::database;

const SELECT_BY_ID: &str = "SELECT id, month, capital_minor, currency_code, tna_basis_points, term_days, tax_minor, created_at, updated_at FROM investment WHERE id = ?1";
const SELECT_BY_MONTH: &str = "SELECT id, month, capital_minor, currency_code, tna_basis_points, term_days, tax_minor, created_at, updated_at FROM investment WHERE month = ?1 ORDER BY created_at ASC, id ASC";
const SELECT_TIMESTAMPS_BY_ID: &str = "SELECT created_at, updated_at FROM investment WHERE id = ?1";

#[derive(Debug, Default)]
pub struct SqliteInvestmentRepository;

impl InvestmentRepository for SqliteInvestmentRepository {
    fn insert(
        &self,
        connection: &Connection,
        investment: &Investment,
    ) -> Result<InvestmentRecord, DomainError> {
        let id = InvestmentId::new();
        let id_text = id.to_string();
        let now = now_truncated_to_millis();
        let tna_basis_points =
            tna_to_sqlite(investment.tna_basis_points(), Some(id_text.as_str()))?;

        connection
            .execute(
                "INSERT INTO investment (id, month, capital_minor, currency_code, tna_basis_points, term_days, tax_minor, created_at, updated_at) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)",
                params![
                    id_text.as_str(),
                    investment.month().code(),
                    investment.capital().minor_units(),
                    investment.currency().code(),
                    tna_basis_points,
                    i64::from(investment.term_days()),
                    investment.taxes().minor_units(),
                    format_timestamp(now),
                    format_timestamp(now),
                ],
            )
            .map_err(database::sqlite_error)?;

        Ok(InvestmentRecord {
            id,
            investment: investment.clone(),
            created_at: now,
            updated_at: now,
        })
    }

    fn get(
        &self,
        connection: &Connection,
        id: InvestmentId,
    ) -> Result<InvestmentRecord, DomainError> {
        let id_text = id.to_string();
        let row = connection
            .query_row(
                SELECT_BY_ID,
                params![id_text.as_str()],
                RawInvestmentRow::from_row,
            )
            .optional()
            .map_err(database::sqlite_error)?
            .ok_or(DomainError::NotFound)?;

        row.into_record()
    }

    fn update(
        &self,
        connection: &Connection,
        id: InvestmentId,
        investment: &Investment,
    ) -> Result<InvestmentRecord, DomainError> {
        let id_text = id.to_string();
        let transaction = connection
            .unchecked_transaction()
            .map_err(database::sqlite_error)?;

        let timestamps = transaction
            .query_row(SELECT_TIMESTAMPS_BY_ID, params![id_text.as_str()], |row| {
                Ok((row.get::<_, String>(0)?, row.get::<_, String>(1)?))
            })
            .optional()
            .map_err(database::sqlite_error)?
            .ok_or(DomainError::NotFound)?;
        let created_at = parse_timestamp(&timestamps.0, "created_at", Some(id_text.as_str()))?;
        let updated_at = parse_timestamp(&timestamps.1, "updated_at", Some(id_text.as_str()))?;
        let next_updated_at = next_timestamp(created_at, updated_at);
        let tna_basis_points =
            tna_to_sqlite(investment.tna_basis_points(), Some(id_text.as_str()))?;

        let rows = transaction
            .execute(
                "UPDATE investment SET month = ?1, capital_minor = ?2, currency_code = ?3, tna_basis_points = ?4, term_days = ?5, tax_minor = ?6, updated_at = ?7 WHERE id = ?8",
                params![
                    investment.month().code(),
                    investment.capital().minor_units(),
                    investment.currency().code(),
                    tna_basis_points,
                    i64::from(investment.term_days()),
                    investment.taxes().minor_units(),
                    format_timestamp(next_updated_at),
                    id_text.as_str(),
                ],
            )
            .map_err(database::sqlite_error)?;

        if rows == 0 {
            return Err(DomainError::NotFound);
        }

        let row = transaction
            .query_row(
                SELECT_BY_ID,
                params![id_text.as_str()],
                RawInvestmentRow::from_row,
            )
            .map_err(database::sqlite_error)?;
        let record = row.into_record()?;
        transaction.commit().map_err(database::sqlite_error)?;

        Ok(record)
    }

    fn list_by_month(
        &self,
        connection: &Connection,
        month: Month,
    ) -> Result<Vec<InvestmentRecord>, DomainError> {
        let mut statement = connection
            .prepare(SELECT_BY_MONTH)
            .map_err(database::sqlite_error)?;
        let rows = statement
            .query_map(params![month.code()], RawInvestmentRow::from_row)
            .map_err(database::sqlite_error)?
            .collect::<rusqlite::Result<Vec<_>>>()
            .map_err(database::sqlite_error)?;

        rows.into_iter()
            .map(RawInvestmentRow::into_record)
            .collect()
    }
}

#[derive(Debug)]
struct RawInvestmentRow {
    id: String,
    month: String,
    capital_minor: i64,
    currency_code: String,
    tna_basis_points: i64,
    term_days: i64,
    tax_minor: i64,
    created_at: String,
    updated_at: String,
}

impl RawInvestmentRow {
    fn from_row(row: &Row<'_>) -> rusqlite::Result<Self> {
        Ok(Self {
            id: row.get(0)?,
            month: row.get(1)?,
            capital_minor: row.get(2)?,
            currency_code: row.get(3)?,
            tna_basis_points: row.get(4)?,
            term_days: row.get(5)?,
            tax_minor: row.get(6)?,
            created_at: row.get(7)?,
            updated_at: row.get(8)?,
        })
    }

    fn into_record(self) -> Result<InvestmentRecord, DomainError> {
        let id = InvestmentId::parse(self.id.as_str())
            .ok_or_else(|| persistence_context("id", self.id.as_str(), Some(self.id.as_str())))?;
        let id_text = id.to_string();
        if id_text.as_str() != self.id.as_str() {
            return Err(persistence_context(
                "id",
                self.id.as_str(),
                Some(id_text.as_str()),
            ));
        }

        let month = Month::parse(self.month.as_str())
            .map_err(|error| map_domain_error(error, "month", Some(id_text.as_str())))?;
        let currency = Currency::from_code(self.currency_code.as_str()).ok_or_else(|| {
            persistence_context(
                "currency_code",
                self.currency_code.as_str(),
                Some(id_text.as_str()),
            )
        })?;
        let tna_basis_points = u64::try_from(self.tna_basis_points).map_err(|_| {
            persistence_context(
                "tna_basis_points",
                self.tna_basis_points.to_string(),
                Some(id_text.as_str()),
            )
        })?;
        let term_days = u32::try_from(self.term_days).map_err(|_| {
            persistence_context(
                "term_days",
                self.term_days.to_string(),
                Some(id_text.as_str()),
            )
        })?;
        let capital = Money::from_minor_units(self.capital_minor, currency);
        let taxes = Money::from_minor_units(self.tax_minor, currency);
        let created_at = parse_timestamp(
            self.created_at.as_str(),
            "created_at",
            Some(id_text.as_str()),
        )?;
        let updated_at = parse_timestamp(
            self.updated_at.as_str(),
            "updated_at",
            Some(id_text.as_str()),
        )?;

        let investment = Investment::from_parts(month, capital, tna_basis_points, term_days, taxes)
            .map_err(|error| map_domain_error(error, "investment", Some(id_text.as_str())))?;

        Ok(InvestmentRecord {
            id,
            investment,
            created_at,
            updated_at,
        })
    }
}

fn tna_to_sqlite(value: u64, id: Option<&str>) -> Result<i64, DomainError> {
    i64::try_from(value).map_err(|_| {
        persistence_context(
            "tna_basis_points",
            format!("{value} fuera del rango de SQLite"),
            id,
        )
    })
}

fn map_domain_error(error: DomainError, fallback_column: &str, id: Option<&str>) -> DomainError {
    let column = match &error {
        DomainError::Validation { field, .. } => domain_column(field),
        DomainError::ArithmeticOverflow { field, .. } => domain_column(field),
        DomainError::CurrencyMismatch { .. } => "currency_code",
        _ => fallback_column,
    };
    persistence_context(column, error.to_string(), id)
}

fn parse_timestamp(
    value: &str,
    column: &str,
    id: Option<&str>,
) -> Result<DateTime<Utc>, DomainError> {
    DateTime::parse_from_rfc3339(value)
        .map(|timestamp| timestamp.with_timezone(&Utc))
        .map_err(|error| persistence_context(column, format!("{value} ({error})"), id))
}

fn next_timestamp(created_at: DateTime<Utc>, updated_at: DateTime<Utc>) -> DateTime<Utc> {
    created_at
        .max(updated_at)
        .checked_add_signed(Duration::milliseconds(1))
        .expect("el siguiente timestamp debe ser válido")
}

fn now_truncated_to_millis() -> DateTime<Utc> {
    DateTime::from_timestamp_millis(Utc::now().timestamp_millis())
        .expect("el timestamp actual debe ser válido")
}

fn format_timestamp(value: DateTime<Utc>) -> String {
    value.to_rfc3339_opts(SecondsFormat::Millis, true)
}

fn domain_column(field: &str) -> &'static str {
    match field {
        "month" => "month",
        "capital" => "capital_minor",
        "taxes" => "tax_minor",
        "currency" => "currency_code",
        "tna" => "tna_basis_points",
        "termDays" => "term_days",
        _ => "investment",
    }
}

fn persistence_context(column: &str, detail: impl AsRef<str>, id: Option<&str>) -> DomainError {
    let detail = detail.as_ref();
    match id {
        Some(id) => DomainError::persistence(format!(
            "registro corrupto en investment.id={id}: columna {column} inválida ({detail})"
        )),
        None => DomainError::persistence(format!(
            "registro corrupto: columna {column} inválida ({detail})"
        )),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const REPOSITORY: SqliteInvestmentRepository = SqliteInvestmentRepository;
    const VALID_TIMESTAMP: &str = "2026-01-01T00:00:00.000Z";

    fn ars(capital: &str, tna: &str, term_days: &str, taxes: &str) -> Investment {
        Investment::new("2026-03", capital, tna, term_days, taxes).unwrap()
    }

    fn usd(capital: &str, tna: &str, term_days: &str, taxes: &str) -> Investment {
        Investment::new_with_currency("2026-03", capital, Currency::Usd, tna, term_days, taxes)
            .unwrap()
    }

    fn typed(
        month: &str,
        currency: Currency,
        capital_minor: i64,
        tna_basis_points: u64,
        term_days: u32,
        tax_minor: i64,
    ) -> Investment {
        Investment::from_parts(
            Month::parse(month).unwrap(),
            Money::from_minor_units(capital_minor, currency),
            tna_basis_points,
            term_days,
            Money::from_minor_units(tax_minor, currency),
        )
        .unwrap()
    }

    fn unknown_id() -> InvestmentId {
        InvestmentId::parse("00000000-0000-4000-8000-000000000000").unwrap()
    }

    fn assert_persistence(error: &DomainError, fragment: &str) {
        match error {
            DomainError::Persistence(message) => {
                assert!(message.contains(fragment), "mensaje inesperado: {message}")
            }
            other => panic!("se esperaba Persistence, se obtuvo {other:?}"),
        }
    }

    #[allow(clippy::too_many_arguments)]
    fn insert_raw_full(
        connection: &Connection,
        id: &str,
        month: &str,
        capital_minor: i64,
        currency_code: &str,
        tna_basis_points: i64,
        term_days: i64,
        tax_minor: i64,
        created_at: &str,
        updated_at: &str,
    ) {
        connection
            .execute(
                "INSERT INTO investment (id, month, capital_minor, currency_code, tna_basis_points, term_days, tax_minor, created_at, updated_at) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)",
                params![
                    id,
                    month,
                    capital_minor,
                    currency_code,
                    tna_basis_points,
                    term_days,
                    tax_minor,
                    created_at,
                    updated_at,
                ],
            )
            .unwrap();
    }

    #[test]
    fn inserta_y_lee_inversion_en_ars() {
        let connection = database::open_in_memory().unwrap();
        let investment = ars("1000000,00", "25,50", "30", "500,00");

        let record = REPOSITORY.insert(&connection, &investment).unwrap();
        let loaded = REPOSITORY.get(&connection, record.id()).unwrap();

        assert_eq!(loaded.id(), record.id());
        assert_eq!(loaded.investment().month().code(), "2026-03");
        assert_eq!(loaded.investment().capital().minor_units(), 100_000_000);
        assert_eq!(loaded.investment().currency(), Currency::Ars);
        assert_eq!(loaded.investment().tna_basis_points(), 2550);
        assert_eq!(loaded.investment().term_days(), 30);
        assert_eq!(loaded.investment().taxes().minor_units(), 50_000);
    }

    #[test]
    fn inserta_y_lee_inversion_en_usd() {
        let connection = database::open_in_memory().unwrap();
        let investment = usd("1000.00", "30", "15", "0.00");

        let record = REPOSITORY.insert(&connection, &investment).unwrap();
        let loaded = REPOSITORY.get(&connection, record.id()).unwrap();

        assert_eq!(loaded.id(), record.id());
        assert_eq!(loaded.investment().capital().minor_units(), 100_000);
        assert_eq!(loaded.investment().currency(), Currency::Usd);
        assert_eq!(loaded.investment().tna_basis_points(), 3000);
        assert_eq!(loaded.investment().term_days(), 15);
        assert_eq!(loaded.investment().taxes().minor_units(), 0);
    }

    #[test]
    fn insert_devuelve_timestamps_identicos_a_get() {
        let connection = database::open_in_memory().unwrap();
        let record = REPOSITORY
            .insert(&connection, &ars("100000,00", "25", "30", "0"))
            .unwrap();

        let loaded = REPOSITORY.get(&connection, record.id()).unwrap();

        assert_eq!(loaded.created_at(), record.created_at());
        assert_eq!(loaded.updated_at(), record.updated_at());
        assert_eq!(loaded.created_at(), loaded.updated_at());
    }

    #[test]
    fn get_de_id_inexistente_devuelve_not_found() {
        let connection = database::open_in_memory().unwrap();

        let error = REPOSITORY.get(&connection, unknown_id()).unwrap_err();

        assert!(matches!(error, DomainError::NotFound));
    }

    #[test]
    fn update_modifica_todos_los_datos_de_origen() {
        let connection = database::open_in_memory().unwrap();
        let record = REPOSITORY
            .insert(&connection, &ars("100000,00", "25", "30", "0"))
            .unwrap();
        let replacement = typed("2026-04", Currency::Usd, 250_000, 4_050, 45, 1_500);

        let updated = REPOSITORY
            .update(&connection, record.id(), &replacement)
            .unwrap();

        assert_eq!(updated.id(), record.id());
        assert_eq!(updated.created_at(), record.created_at());
        assert_eq!(updated.investment().month().code(), "2026-04");
        assert_eq!(updated.investment().capital().minor_units(), 250_000);
        assert_eq!(updated.investment().currency(), Currency::Usd);
        assert_eq!(updated.investment().tna_basis_points(), 4_050);
        assert_eq!(updated.investment().term_days(), 45);
        assert_eq!(updated.investment().taxes().minor_units(), 1_500);
    }

    #[test]
    fn update_inmediato_avanza_updated_at() {
        let connection = database::open_in_memory().unwrap();
        let record = REPOSITORY
            .insert(&connection, &ars("100000,00", "25", "30", "0"))
            .unwrap();

        let updated = REPOSITORY
            .update(
                &connection,
                record.id(),
                &ars("200000,00", "30", "60", "100,00"),
            )
            .unwrap();

        assert!(updated.updated_at() > record.updated_at());
        assert!(updated.updated_at() > record.created_at());
        assert_eq!(updated.created_at(), record.created_at());
    }

    #[test]
    fn update_de_id_inexistente_devuelve_not_found() {
        let connection = database::open_in_memory().unwrap();

        let error = REPOSITORY
            .update(
                &connection,
                unknown_id(),
                &ars("100000,00", "25", "30", "0"),
            )
            .unwrap_err();

        assert!(matches!(error, DomainError::NotFound));
        assert!(
            REPOSITORY
                .list_by_month(&connection, Month::parse("2026-03").unwrap())
                .unwrap()
                .is_empty()
        );
    }

    #[test]
    fn list_by_month_filtra_y_excluye_otros_meses() {
        let connection = database::open_in_memory().unwrap();
        REPOSITORY
            .insert(&connection, &ars("100000,00", "25", "30", "0"))
            .unwrap();
        REPOSITORY
            .insert(&connection, &ars("200000,00", "30", "60", "0"))
            .unwrap();
        REPOSITORY
            .insert(
                &connection,
                &typed("2026-04", Currency::Ars, 300_000, 3_500, 20, 0),
            )
            .unwrap();

        let march = REPOSITORY
            .list_by_month(&connection, Month::parse("2026-03").unwrap())
            .unwrap();
        let april = REPOSITORY
            .list_by_month(&connection, Month::parse("2026-04").unwrap())
            .unwrap();

        assert_eq!(march.len(), 2);
        assert!(
            march
                .iter()
                .all(|record| record.investment().month().code() == "2026-03")
        );
        assert_eq!(april.len(), 1);
        assert_eq!(april[0].investment().month().code(), "2026-04");
    }

    #[test]
    fn list_by_month_permite_multiples_inversiones_del_mismo_mes() {
        let connection = database::open_in_memory().unwrap();
        REPOSITORY
            .insert(&connection, &ars("100000,00", "25", "30", "0"))
            .unwrap();
        REPOSITORY
            .insert(&connection, &usd("1000.00", "30", "15", "0.00"))
            .unwrap();

        let records = REPOSITORY
            .list_by_month(&connection, Month::parse("2026-03").unwrap())
            .unwrap();

        assert_eq!(records.len(), 2);
    }

    #[test]
    fn list_by_month_usa_orden_determinista() {
        let connection = database::open_in_memory().unwrap();
        let first_by_id = "00000000-0000-4000-8000-000000000001";
        let second_by_id = "00000000-0000-4000-8000-000000000002";
        let first_by_time = "00000000-0000-4000-8000-000000000003";

        insert_raw_full(
            &connection,
            second_by_id,
            "2026-03",
            100_000,
            "ARS",
            2_500,
            30,
            0,
            "2026-01-01T00:00:00.001Z",
            VALID_TIMESTAMP,
        );
        insert_raw_full(
            &connection,
            first_by_id,
            "2026-03",
            100_000,
            "ARS",
            2_500,
            30,
            0,
            "2026-01-01T00:00:00.001Z",
            VALID_TIMESTAMP,
        );
        insert_raw_full(
            &connection,
            first_by_time,
            "2026-03",
            100_000,
            "ARS",
            2_500,
            30,
            0,
            "2025-12-31T00:00:00.000Z",
            VALID_TIMESTAMP,
        );
        insert_raw_full(
            &connection,
            "00000000-0000-4000-8000-000000000004",
            "2026-04",
            100_000,
            "ARS",
            2_500,
            30,
            0,
            "2025-01-01T00:00:00.000Z",
            VALID_TIMESTAMP,
        );

        let records = REPOSITORY
            .list_by_month(&connection, Month::parse("2026-03").unwrap())
            .unwrap();
        let ids: Vec<String> = records
            .iter()
            .map(|record| record.id().to_string())
            .collect();

        assert_eq!(ids, vec![first_by_time, first_by_id, second_by_id]);
    }

    #[test]
    fn conserva_moneda_y_basis_points_despues_de_cerrar_y_reabrir() {
        let directory = tempfile::tempdir().unwrap();
        let path = database::resolve_path(directory.path());

        {
            let connection = database::open(&path).unwrap();
            REPOSITORY
                .insert(&connection, &ars("1000000,00", "25,50", "30", "500,00"))
                .unwrap();
            REPOSITORY
                .insert(&connection, &usd("1000.00", "40,25", "15", "0.00"))
                .unwrap();
        }

        let connection = database::open(&path).unwrap();
        let records = REPOSITORY
            .list_by_month(&connection, Month::parse("2026-03").unwrap())
            .unwrap();
        let ars_record = records
            .iter()
            .find(|record| record.investment().currency() == Currency::Ars)
            .unwrap();
        let usd_record = records
            .iter()
            .find(|record| record.investment().currency() == Currency::Usd)
            .unwrap();

        assert_eq!(ars_record.investment().tna_basis_points(), 2550);
        assert_eq!(ars_record.investment().capital().minor_units(), 100_000_000);
        assert_eq!(usd_record.investment().tna_basis_points(), 4_025);
        assert_eq!(usd_record.investment().capital().minor_units(), 100_000);
    }

    #[test]
    fn tna_fuera_de_rango_sqlite_devuelve_persistence() {
        let connection = database::open_in_memory().unwrap();
        let investment = typed(
            "2026-03",
            Currency::Ars,
            100_000,
            (i64::MAX as u64) + 1,
            30,
            0,
        );

        let error = REPOSITORY.insert(&connection, &investment).unwrap_err();

        assert_persistence(&error, "tna_basis_points");
    }

    #[test]
    fn mes_corruptas_devuelve_persistence() {
        let connection = database::open_in_memory().unwrap();
        let id = "00000000-0000-4000-8000-000000000010";
        insert_raw_full(
            &connection,
            id,
            "2026-13",
            100_000,
            "ARS",
            2_500,
            30,
            0,
            VALID_TIMESTAMP,
            VALID_TIMESTAMP,
        );

        let error = REPOSITORY
            .get(&connection, InvestmentId::parse(id).unwrap())
            .unwrap_err();

        assert_persistence(&error, "month");
    }

    #[test]
    fn moneda_corruptas_devuelve_persistence() {
        let connection = database::open_in_memory().unwrap();
        let id = "00000000-0000-4000-8000-000000000011";
        insert_raw_full(
            &connection,
            id,
            "2026-03",
            100_000,
            "EUR",
            2_500,
            30,
            0,
            VALID_TIMESTAMP,
            VALID_TIMESTAMP,
        );

        let error = REPOSITORY
            .get(&connection, InvestmentId::parse(id).unwrap())
            .unwrap_err();

        assert_persistence(&error, "currency_code");
    }

    #[test]
    fn tna_corruptas_devuelve_persistence() {
        let connection = database::open_in_memory().unwrap();
        let id = "00000000-0000-4000-8000-000000000012";
        insert_raw_full(
            &connection,
            id,
            "2026-03",
            100_000,
            "ARS",
            -1,
            30,
            0,
            VALID_TIMESTAMP,
            VALID_TIMESTAMP,
        );

        let error = REPOSITORY
            .get(&connection, InvestmentId::parse(id).unwrap())
            .unwrap_err();

        assert_persistence(&error, "tna_basis_points");
    }

    #[test]
    fn plazo_corruptas_devuelve_persistence() {
        let connection = database::open_in_memory().unwrap();
        let negative_id = "00000000-0000-4000-8000-000000000013";
        let overflow_id = "00000000-0000-4000-8000-000000000014";
        insert_raw_full(
            &connection,
            negative_id,
            "2026-03",
            100_000,
            "ARS",
            2_500,
            -1,
            0,
            VALID_TIMESTAMP,
            VALID_TIMESTAMP,
        );
        insert_raw_full(
            &connection,
            overflow_id,
            "2026-03",
            100_000,
            "ARS",
            2_500,
            i64::from(u32::MAX) + 1,
            0,
            VALID_TIMESTAMP,
            VALID_TIMESTAMP,
        );

        let negative_error = REPOSITORY
            .get(&connection, InvestmentId::parse(negative_id).unwrap())
            .unwrap_err();
        let overflow_error = REPOSITORY
            .get(&connection, InvestmentId::parse(overflow_id).unwrap())
            .unwrap_err();

        assert_persistence(&negative_error, "term_days");
        assert_persistence(&overflow_error, "term_days");
    }

    #[test]
    fn importes_corruptos_devuelven_persistence() {
        let connection = database::open_in_memory().unwrap();
        let capital_id = "00000000-0000-4000-8000-000000000015";
        let taxes_id = "00000000-0000-4000-8000-000000000016";
        insert_raw_full(
            &connection,
            capital_id,
            "2026-03",
            -1,
            "ARS",
            2_500,
            30,
            0,
            VALID_TIMESTAMP,
            VALID_TIMESTAMP,
        );
        insert_raw_full(
            &connection,
            taxes_id,
            "2026-03",
            100_000,
            "ARS",
            2_500,
            30,
            -1,
            VALID_TIMESTAMP,
            VALID_TIMESTAMP,
        );

        let capital_error = REPOSITORY
            .get(&connection, InvestmentId::parse(capital_id).unwrap())
            .unwrap_err();
        let taxes_error = REPOSITORY
            .get(&connection, InvestmentId::parse(taxes_id).unwrap())
            .unwrap_err();

        assert_persistence(&capital_error, "capital_minor");
        assert_persistence(&taxes_error, "tax_minor");
    }

    #[test]
    fn timestamps_corruptos_devuelven_persistence() {
        let connection = database::open_in_memory().unwrap();
        let bad_created_id = "00000000-0000-4000-8000-000000000017";
        let bad_updated_id = "00000000-0000-4000-8000-000000000018";
        insert_raw_full(
            &connection,
            bad_created_id,
            "2026-03",
            100_000,
            "ARS",
            2_500,
            30,
            0,
            "no-es-fecha",
            VALID_TIMESTAMP,
        );
        insert_raw_full(
            &connection,
            bad_updated_id,
            "2026-03",
            100_000,
            "ARS",
            2_500,
            30,
            0,
            VALID_TIMESTAMP,
            "no-es-fecha",
        );

        let created_error = REPOSITORY
            .get(&connection, InvestmentId::parse(bad_created_id).unwrap())
            .unwrap_err();
        let updated_error = REPOSITORY
            .get(&connection, InvestmentId::parse(bad_updated_id).unwrap())
            .unwrap_err();

        assert_persistence(&created_error, "created_at");
        assert_persistence(&updated_error, "updated_at");
    }

    #[test]
    fn id_corrupto_devuelve_persistence_al_listar() {
        let connection = database::open_in_memory().unwrap();
        insert_raw_full(
            &connection,
            "no-es-uuid",
            "2026-03",
            100_000,
            "ARS",
            2_500,
            30,
            0,
            VALID_TIMESTAMP,
            VALID_TIMESTAMP,
        );

        let error = REPOSITORY
            .list_by_month(&connection, Month::parse("2026-03").unwrap())
            .unwrap_err();

        assert_persistence(&error, "id");
    }

    #[test]
    fn id_valido_no_canonico_se_rechaza_como_registro_corrupto() {
        let connection = database::open_in_memory().unwrap();
        let raw_id = "00000000000040008000000000000001";
        assert!(InvestmentId::parse(raw_id).is_some());
        insert_raw_full(
            &connection,
            raw_id,
            "2026-03",
            100_000,
            "ARS",
            2_500,
            30,
            0,
            VALID_TIMESTAMP,
            VALID_TIMESTAMP,
        );

        let error = REPOSITORY
            .list_by_month(&connection, Month::parse("2026-03").unwrap())
            .unwrap_err();

        assert_persistence(&error, "id");
    }

    #[test]
    fn month_se_almacena_como_texto_yyyy_mm() {
        let connection = database::open_in_memory().unwrap();
        let record = REPOSITORY
            .insert(&connection, &ars("100000,00", "25", "30", "0"))
            .unwrap();

        let raw_month: String = connection
            .query_row(
                "SELECT month FROM investment WHERE id = ?1",
                params![record.id().to_string()],
                |row| row.get(0),
            )
            .unwrap();

        assert_eq!(raw_month, "2026-03");
    }

    #[test]
    fn no_persiste_calculos_derivados() {
        let connection = database::open_in_memory().unwrap();
        REPOSITORY
            .insert(&connection, &ars("100000,00", "25", "30", "0"))
            .unwrap();

        let mut statement = connection.prepare("PRAGMA table_info(investment)").unwrap();
        let columns: Vec<String> = statement
            .query_map([], |row| row.get::<_, String>(1))
            .unwrap()
            .map(|value| value.unwrap())
            .collect();

        assert_eq!(
            columns,
            vec![
                "id",
                "month",
                "capital_minor",
                "currency_code",
                "tna_basis_points",
                "term_days",
                "tax_minor",
                "created_at",
                "updated_at",
            ]
        );
    }
}
