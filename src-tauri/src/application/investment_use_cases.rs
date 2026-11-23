use rusqlite::Connection;

use crate::application::investment_repository::{
    InvestmentId, InvestmentRecord, InvestmentRepository,
};
use crate::domain::error::DomainError;
use crate::domain::investment::Investment;
use crate::domain::money::Currency;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CreateInvestmentInput {
    pub month: String,
    pub currency: Currency,
    pub capital: String,
    pub tna: String,
    pub term_days: String,
    pub taxes: String,
}

impl Default for CreateInvestmentInput {
    fn default() -> Self {
        Self {
            month: String::new(),
            currency: Currency::Ars,
            capital: "0".to_owned(),
            tna: "0".to_owned(),
            term_days: "0".to_owned(),
            taxes: "0".to_owned(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UpdateInvestmentInput {
    pub month: String,
    pub capital: String,
    pub tna: String,
    pub term_days: String,
    pub taxes: String,
}

pub fn prepare_create_investment(input: CreateInvestmentInput) -> Result<Investment, DomainError> {
    Investment::new_with_currency(
        input.month.as_str(),
        input.capital.as_str(),
        input.currency,
        input.tna.as_str(),
        input.term_days.as_str(),
        input.taxes.as_str(),
    )
}

pub fn prepare_update_investment(
    current: InvestmentRecord,
    input: UpdateInvestmentInput,
) -> Result<Investment, DomainError> {
    Investment::new_with_currency(
        input.month.as_str(),
        input.capital.as_str(),
        current.investment().currency(),
        input.tna.as_str(),
        input.term_days.as_str(),
        input.taxes.as_str(),
    )
}

pub fn create_investment<R>(
    connection: &Connection,
    repository: &R,
    input: CreateInvestmentInput,
) -> Result<InvestmentRecord, DomainError>
where
    R: InvestmentRepository + ?Sized,
{
    let investment = prepare_create_investment(input)?;
    repository.insert(connection, &investment)
}

pub fn update_investment<R>(
    connection: &Connection,
    repository: &R,
    id: InvestmentId,
    input: UpdateInvestmentInput,
) -> Result<InvestmentRecord, DomainError>
where
    R: InvestmentRepository + ?Sized,
{
    let current = repository.get(connection, id)?;
    let replacement = prepare_update_investment(current, input)?;
    repository.update(connection, id, &replacement)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::infrastructure::database;
    use crate::infrastructure::investment_repository::SqliteInvestmentRepository;

    const REPOSITORY: SqliteInvestmentRepository = SqliteInvestmentRepository;

    fn ars_input(
        month: &str,
        capital: &str,
        tna: &str,
        term_days: &str,
        taxes: &str,
    ) -> CreateInvestmentInput {
        CreateInvestmentInput {
            month: month.to_owned(),
            currency: Currency::Ars,
            capital: capital.to_owned(),
            tna: tna.to_owned(),
            term_days: term_days.to_owned(),
            taxes: taxes.to_owned(),
        }
    }

    fn usd_input(
        month: &str,
        capital: &str,
        tna: &str,
        term_days: &str,
        taxes: &str,
    ) -> CreateInvestmentInput {
        CreateInvestmentInput {
            month: month.to_owned(),
            currency: Currency::Usd,
            capital: capital.to_owned(),
            tna: tna.to_owned(),
            term_days: term_days.to_owned(),
            taxes: taxes.to_owned(),
        }
    }

    fn update_input(
        month: &str,
        capital: &str,
        tna: &str,
        term_days: &str,
        taxes: &str,
    ) -> UpdateInvestmentInput {
        UpdateInvestmentInput {
            month: month.to_owned(),
            capital: capital.to_owned(),
            tna: tna.to_owned(),
            term_days: term_days.to_owned(),
            taxes: taxes.to_owned(),
        }
    }

    fn unknown_id() -> InvestmentId {
        InvestmentId::parse("00000000-0000-4000-8000-000000000099").unwrap()
    }

    fn investment_count(connection: &Connection) -> i64 {
        connection
            .query_row("SELECT COUNT(*) FROM investment", [], |row| row.get(0))
            .unwrap()
    }

    fn assert_validation(error: DomainError, expected_field: &'static str) {
        match error {
            DomainError::Validation { field, .. } => assert_eq!(field, expected_field),
            other => panic!("se esperaba error de validación, se obtuvo {other:?}"),
        }
    }

    fn assert_validation_message(
        error: DomainError,
        expected_field: &'static str,
        expected_message: &str,
    ) {
        match error {
            DomainError::Validation { field, message } => {
                assert_eq!(field, expected_field);
                assert_eq!(message, expected_message);
            }
            other => panic!("se esperaba error de validación, se obtuvo {other:?}"),
        }
    }

    #[test]
    fn alta_valida_devuelve_registro_en_ars() {
        let connection = database::open_in_memory().unwrap();

        let record = create_investment(
            &connection,
            &REPOSITORY,
            ars_input("2026-03", "1000000,00", "30", "30", "500,00"),
        )
        .unwrap();
        let investment = record.investment();

        assert_eq!(investment.month().code(), "2026-03");
        assert_eq!(investment.currency(), Currency::Ars);
        assert_eq!(investment.capital().minor_units(), 100_000_000);
        assert_eq!(investment.tna_basis_points(), 3000);
        assert_eq!(investment.term_days(), 30);
        assert_eq!(investment.taxes().minor_units(), 50_000);
    }

    #[test]
    fn alta_valida_en_usd_no_convierte() {
        let connection = database::open_in_memory().unwrap();

        let record = create_investment(
            &connection,
            &REPOSITORY,
            usd_input("2026-03", "1000.00", "30", "15", "0.00"),
        )
        .unwrap();
        let investment = record.investment();

        assert_eq!(investment.currency(), Currency::Usd);
        assert_eq!(investment.capital().minor_units(), 100_000);
        assert_eq!(investment.taxes().minor_units(), 0);
    }

    #[test]
    fn defaults_son_ars_y_campos_numericos_en_cero() {
        let defaults = CreateInvestmentInput::default();

        assert_eq!(defaults.currency, Currency::Ars);
        assert_eq!(defaults.capital, "0");
        assert_eq!(defaults.tna, "0");
        assert_eq!(defaults.term_days, "0");
        assert_eq!(defaults.taxes, "0");

        let connection = database::open_in_memory().unwrap();
        let input = CreateInvestmentInput {
            month: "2026-03".to_owned(),
            ..CreateInvestmentInput::default()
        };

        let record = create_investment(&connection, &REPOSITORY, input).unwrap();
        let investment = record.investment();

        assert_eq!(investment.currency(), Currency::Ars);
        assert_eq!(investment.capital().minor_units(), 0);
        assert_eq!(investment.tna_basis_points(), 0);
        assert_eq!(investment.term_days(), 0);
        assert_eq!(investment.taxes().minor_units(), 0);
    }

    #[test]
    fn mes_vacio_rechazado_sin_persistencia() {
        let connection = database::open_in_memory().unwrap();
        let input = CreateInvestmentInput::default();
        let before = investment_count(&connection);

        let error = create_investment(&connection, &REPOSITORY, input).unwrap_err();

        assert_validation_message(error, "month", "El mes es obligatorio.");
        assert_eq!(investment_count(&connection), before);
    }

    #[test]
    fn mes_con_formato_invalido_rechazado_sin_persistencia() {
        let connection = database::open_in_memory().unwrap();
        let input = ars_input("2026/03", "100,00", "25", "30", "0");
        let before = investment_count(&connection);

        let error = create_investment(&connection, &REPOSITORY, input).unwrap_err();

        assert_validation(error, "month");
        assert_eq!(investment_count(&connection), before);
    }

    #[test]
    fn mes_con_numero_invalido_rechazado_sin_persistencia() {
        let connection = database::open_in_memory().unwrap();
        let input = ars_input("2026-13", "100,00", "25", "30", "0");
        let before = investment_count(&connection);

        let error = create_investment(&connection, &REPOSITORY, input).unwrap_err();

        assert_validation(error, "month");
        assert_eq!(investment_count(&connection), before);
    }

    #[test]
    fn capital_negativo_rechazado_sin_persistencia() {
        let connection = database::open_in_memory().unwrap();
        let input = ars_input("2026-03", "-1,00", "25", "30", "0");
        let before = investment_count(&connection);

        let error = create_investment(&connection, &REPOSITORY, input).unwrap_err();

        assert_validation(error, "capital");
        assert_eq!(investment_count(&connection), before);
    }

    #[test]
    fn impuestos_negativos_rechazados_sin_persistencia() {
        let connection = database::open_in_memory().unwrap();
        let input = ars_input("2026-03", "100,00", "25", "30", "-1,00");
        let before = investment_count(&connection);

        let error = create_investment(&connection, &REPOSITORY, input).unwrap_err();

        assert_validation(error, "taxes");
        assert_eq!(investment_count(&connection), before);
    }

    #[test]
    fn tna_invalida_rechazada_sin_persistencia() {
        let connection = database::open_in_memory().unwrap();
        let input = ars_input("2026-03", "100,00", "treinta", "30", "0");
        let before = investment_count(&connection);

        let error = create_investment(&connection, &REPOSITORY, input).unwrap_err();

        assert_validation(error, "tna");
        assert_eq!(investment_count(&connection), before);
    }

    #[test]
    fn plazo_invalido_rechazado_sin_persistencia() {
        let connection = database::open_in_memory().unwrap();
        let input = ars_input("2026-03", "100,00", "25", "30,5", "0");
        let before = investment_count(&connection);

        let error = create_investment(&connection, &REPOSITORY, input).unwrap_err();

        assert_validation(error, "termDays");
        assert_eq!(investment_count(&connection), before);
    }

    #[test]
    fn edicion_valida_actualiza_datos_y_conserva_identidad() {
        let connection = database::open_in_memory().unwrap();
        let created = create_investment(
            &connection,
            &REPOSITORY,
            usd_input("2026-03", "1000.00", "30", "15", "0.00"),
        )
        .unwrap();

        let updated = update_investment(
            &connection,
            &REPOSITORY,
            created.id(),
            update_input("2026-04", "2000.50", "40,25", "45", "10.25"),
        )
        .unwrap();
        let investment = updated.investment();

        assert_eq!(updated.id(), created.id());
        assert_eq!(updated.created_at(), created.created_at());
        assert!(updated.updated_at() > created.updated_at());
        assert_eq!(investment.currency(), Currency::Usd);
        assert_eq!(investment.month().code(), "2026-04");
        assert_eq!(investment.capital().minor_units(), 200_050);
        assert_eq!(investment.tna_basis_points(), 4025);
        assert_eq!(investment.term_days(), 45);
        assert_eq!(investment.taxes().minor_units(), 1_025);
    }

    #[test]
    fn edicion_invalida_no_modifica_registro_previo() {
        let connection = database::open_in_memory().unwrap();
        let created = create_investment(
            &connection,
            &REPOSITORY,
            ars_input("2026-03", "1000000,00", "30", "30", "500,00"),
        )
        .unwrap();
        let before = investment_count(&connection);

        let error = update_investment(
            &connection,
            &REPOSITORY,
            created.id(),
            update_input("2026-04", "-1,00", "40", "45", "600,00"),
        )
        .unwrap_err();

        assert_validation(error, "capital");
        assert_eq!(investment_count(&connection), before);

        let loaded = REPOSITORY.get(&connection, created.id()).unwrap();
        assert_eq!(loaded.investment().month().code(), "2026-03");
        assert_eq!(loaded.investment().capital().minor_units(), 100_000_000);
        assert_eq!(loaded.investment().tna_basis_points(), 3000);
        assert_eq!(loaded.investment().term_days(), 30);
        assert_eq!(loaded.investment().taxes().minor_units(), 50_000);
        assert_eq!(loaded.created_at(), created.created_at());
        assert_eq!(loaded.updated_at(), created.updated_at());
    }

    #[test]
    fn edicion_de_id_inexistente_devuelve_not_found() {
        let connection = database::open_in_memory().unwrap();
        let before = investment_count(&connection);

        let error = update_investment(
            &connection,
            &REPOSITORY,
            unknown_id(),
            update_input("2026-03", "100,00", "25", "30", "0"),
        )
        .unwrap_err();

        assert!(matches!(error, DomainError::NotFound));
        assert_eq!(investment_count(&connection), before);
    }
}
