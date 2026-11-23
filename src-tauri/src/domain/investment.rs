use crate::domain::error::DomainError;
use crate::domain::money::{Currency, Money};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Month {
    year: u16,
    month: u8,
}

impl Month {
    pub fn parse(value: &str) -> Result<Self, DomainError> {
        let bytes = value.as_bytes();
        if bytes.len() != 7 {
            return Err(invalid_month("El mes debe tener el formato YYYY-MM."));
        }
        if bytes[4] != b'-' {
            return Err(invalid_month("El mes debe tener el formato YYYY-MM."));
        }
        if !bytes[..4].iter().all(|byte| byte.is_ascii_digit())
            || !bytes[5..].iter().all(|byte| byte.is_ascii_digit())
        {
            return Err(invalid_month("El mes debe tener el formato YYYY-MM."));
        }

        let year = bytes[..4]
            .iter()
            .fold(0_u16, |value, byte| value * 10 + u16::from(byte - b'0'));
        let month = bytes[5..]
            .iter()
            .fold(0_u8, |value, byte| value * 10 + (byte - b'0'));

        validate_month_parts(year, month)?;

        Ok(Self { year, month })
    }

    pub fn code(&self) -> String {
        format!("{:04}-{:02}", self.year, self.month)
    }

    pub const fn year(&self) -> u16 {
        self.year
    }

    pub const fn month(&self) -> u8 {
        self.month
    }
}

#[derive(Debug, Clone)]
pub struct Investment {
    month: Month,
    capital: Money,
    tna_basis_points: u64,
    term_days: u32,
    taxes: Money,
}

impl Investment {
    pub fn new(
        month: &str,
        capital: &str,
        tna: &str,
        term_days: &str,
        taxes: &str,
    ) -> Result<Self, DomainError> {
        Self::new_with_currency(month, capital, Currency::Ars, tna, term_days, taxes)
    }

    pub fn new_with_currency(
        month: &str,
        capital: &str,
        currency: Currency,
        tna: &str,
        term_days: &str,
        taxes: &str,
    ) -> Result<Self, DomainError> {
        let month = Month::parse(month)?;
        let capital = parse_money_field(capital, currency, "capital")?;
        let taxes = parse_money_field(taxes, currency, "taxes")?;
        let tna_basis_points = parse_tna_basis_points(tna)?;
        let term_days = parse_term_days(term_days)?;

        Self::from_parts(month, capital, tna_basis_points, term_days, taxes)
    }

    pub(crate) fn from_parts(
        month: Month,
        capital: Money,
        tna_basis_points: u64,
        term_days: u32,
        taxes: Money,
    ) -> Result<Self, DomainError> {
        validate_month_parts(month.year, month.month)?;
        validate_non_negative_money(capital, "capital")?;
        validate_non_negative_money(taxes, "taxes")?;
        ensure_same_currency(capital, taxes)?;

        Ok(Self {
            month,
            capital,
            tna_basis_points,
            term_days,
            taxes,
        })
    }

    pub const fn month(&self) -> Month {
        self.month
    }

    pub const fn capital(&self) -> Money {
        self.capital
    }

    pub const fn currency(&self) -> Currency {
        self.capital.currency()
    }

    pub const fn tna_basis_points(&self) -> u64 {
        self.tna_basis_points
    }

    pub const fn term_days(&self) -> u32 {
        self.term_days
    }

    pub const fn taxes(&self) -> Money {
        self.taxes
    }
}

fn invalid_month(message: &str) -> DomainError {
    DomainError::validation("month", message)
}

fn validate_month_parts(year: u16, month: u8) -> Result<(), DomainError> {
    if !(1..=9999).contains(&year) {
        return Err(invalid_month("El año debe estar entre 0001 y 9999."));
    }
    if !(1..=12).contains(&month) {
        return Err(invalid_month("El mes debe estar entre 01 y 12."));
    }
    Ok(())
}

fn parse_money_field(
    value: &str,
    currency: Currency,
    field: &'static str,
) -> Result<Money, DomainError> {
    let money = Money::from_major_decimal(value, currency).map_err(|error| match error {
        DomainError::Validation { message, .. } => DomainError::validation(field, message),
        DomainError::ArithmeticOverflow { operation, .. } => {
            DomainError::arithmetic_overflow(field, operation)
        }
        other => other,
    })?;
    validate_non_negative_money(money, field)?;
    Ok(money)
}

fn invalid_non_negative_message(field: &'static str) -> &'static str {
    match field {
        "capital" => "El capital no puede ser negativo.",
        "taxes" => "Los impuestos no pueden ser negativos.",
        _ => "El importe no puede ser negativo.",
    }
}

fn validate_non_negative_money(money: Money, field: &'static str) -> Result<(), DomainError> {
    if money.minor_units() < 0 {
        return Err(DomainError::validation(
            field,
            invalid_non_negative_message(field),
        ));
    }
    Ok(())
}

fn ensure_same_currency(capital: Money, taxes: Money) -> Result<(), DomainError> {
    if capital.currency() == taxes.currency() {
        Ok(())
    } else {
        Err(DomainError::currency_mismatch(
            capital.currency().code(),
            taxes.currency().code(),
        ))
    }
}

fn invalid_tna(message: &str) -> DomainError {
    DomainError::validation("tna", message)
}

fn tna_overflow() -> DomainError {
    DomainError::arithmetic_overflow("tna", "análisis de la TNA")
}

fn parse_tna_basis_points(value: &str) -> Result<u64, DomainError> {
    let bytes = value.as_bytes();
    if bytes.is_empty() {
        return Err(invalid_tna("La TNA no puede estar vacía."));
    }

    let mut major = 0_u128;
    let mut fraction = 0_u128;
    let mut fraction_digits = 0_u8;
    let mut major_digits = 0_usize;
    let mut has_separator = false;

    for &byte in bytes {
        match byte {
            b'0'..=b'9' => {
                let digit = u128::from(byte - b'0');
                if has_separator {
                    if fraction_digits >= 2 {
                        return Err(invalid_tna("La TNA admite como máximo dos decimales."));
                    }
                    fraction = fraction
                        .checked_mul(10)
                        .and_then(|value| value.checked_add(digit))
                        .ok_or_else(tna_overflow)?;
                    fraction_digits += 1;
                } else {
                    major = major
                        .checked_mul(10)
                        .and_then(|value| value.checked_add(digit))
                        .ok_or_else(tna_overflow)?;
                    major_digits += 1;
                }
            }
            b',' | b'.' => {
                if has_separator {
                    return Err(invalid_tna("La TNA contiene más de un separador decimal."));
                }
                if major_digits == 0 {
                    return Err(invalid_tna("La TNA debe comenzar con dígitos."));
                }
                has_separator = true;
            }
            _ => {
                return Err(invalid_tna("La TNA contiene caracteres no permitidos."));
            }
        }
    }

    if has_separator && fraction_digits == 0 {
        return Err(invalid_tna(
            "El separador decimal debe tener al menos un dígito.",
        ));
    }

    let fraction_scale = match fraction_digits {
        0 => 1,
        1 => 10,
        _ => 1,
    };
    let scaled_fraction = fraction
        .checked_mul(fraction_scale)
        .ok_or_else(tna_overflow)?;
    let basis_points = major
        .checked_mul(100)
        .and_then(|value| value.checked_add(scaled_fraction))
        .ok_or_else(tna_overflow)?;

    u64::try_from(basis_points).map_err(|_| tna_overflow())
}

fn invalid_term(message: &str) -> DomainError {
    DomainError::validation("termDays", message)
}

fn term_overflow() -> DomainError {
    DomainError::arithmetic_overflow("termDays", "análisis del plazo")
}

fn parse_term_days(value: &str) -> Result<u32, DomainError> {
    let bytes = value.as_bytes();
    if bytes.is_empty() {
        return Err(invalid_term("El plazo no puede estar vacío."));
    }

    let mut result = 0_u128;
    for &byte in bytes {
        if !byte.is_ascii_digit() {
            return Err(invalid_term("El plazo solo admite dígitos."));
        }
        let digit = u128::from(byte - b'0');
        result = result
            .checked_mul(10)
            .and_then(|value| value.checked_add(digit))
            .ok_or_else(term_overflow)?;
    }

    u32::try_from(result).map_err(|_| term_overflow())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn ars(minor_units: i64) -> Money {
        Money::from_minor_units(minor_units, Currency::Ars)
    }

    fn usd(minor_units: i64) -> Money {
        Money::from_minor_units(minor_units, Currency::Usd)
    }

    fn assert_validation_field(error: DomainError, expected: &'static str) {
        assert!(matches!(error, DomainError::Validation { field, .. } if field == expected));
    }

    fn assert_overflow_field(error: DomainError, expected: &'static str) {
        assert!(
            matches!(error, DomainError::ArithmeticOverflow { field, .. } if field == expected)
        );
    }

    #[test]
    fn month_parse_valido() {
        let month = Month::parse("2026-01").unwrap();

        assert_eq!(month.year(), 2026);
        assert_eq!(month.month(), 1);
        assert_eq!(month.code(), "2026-01");
    }

    #[test]
    fn month_parse_acepta_anio_maximo() {
        let month = Month::parse("9999-12").unwrap();

        assert_eq!(month.year(), 9999);
        assert_eq!(month.month(), 12);
        assert_eq!(month.code(), "9999-12");
    }

    #[test]
    fn month_code_formatea_con_ceros() {
        let month = Month::parse("0001-02").unwrap();

        assert_eq!(month.code(), "0001-02");
    }

    #[test]
    fn month_parse_rechaza_vacio() {
        let error = Month::parse("").unwrap_err();

        assert_validation_field(error, "month");
    }

    #[test]
    fn month_parse_rechaza_longitud_incorrecta() {
        for value in ["2026-1", "2026-001", "026-01", "2026-01-02"] {
            let error = Month::parse(value).unwrap_err();
            assert_validation_field(error, "month");
        }
    }

    #[test]
    fn month_parse_rechaza_separador_incorrecto() {
        let error = Month::parse("2026/01").unwrap_err();

        assert_validation_field(error, "month");
    }

    #[test]
    fn month_parse_rechaza_mes_cero() {
        let error = Month::parse("2026-00").unwrap_err();

        assert_validation_field(error, "month");
    }

    #[test]
    fn month_parse_rechaza_mes_13() {
        let error = Month::parse("2026-13").unwrap_err();

        assert_validation_field(error, "month");
    }

    #[test]
    fn month_parse_rechaza_anio_cero() {
        let error = Month::parse("0000-01").unwrap_err();

        assert_validation_field(error, "month");
    }

    #[test]
    fn month_parse_rechaza_caracteres_no_digitos() {
        for value in ["ab26-01", "2026-0a", "2026-1a", "20-6-01"] {
            let error = Month::parse(value).unwrap_err();
            assert_validation_field(error, "month");
        }
    }

    #[test]
    fn construye_inversion_valida_en_ars() {
        let investment = Investment::new("2026-01", "10000,00", "25,50", "30", "100,00").unwrap();

        assert_eq!(investment.month(), Month::parse("2026-01").unwrap());
        assert_eq!(investment.capital().minor_units(), 1_000_000);
        assert_eq!(investment.taxes().minor_units(), 10_000);
        assert_eq!(investment.currency(), Currency::Ars);
        assert_eq!(investment.tna_basis_points(), 2550);
        assert_eq!(investment.term_days(), 30);
    }

    #[test]
    fn construye_inversion_valida_en_usd() {
        let investment = Investment::new_with_currency(
            "2026-02",
            "10000.00",
            Currency::Usd,
            "25.5",
            "30",
            "100.00",
        )
        .unwrap();

        assert_eq!(investment.capital().minor_units(), 1_000_000);
        assert_eq!(investment.taxes().minor_units(), 10_000);
        assert_eq!(investment.currency(), Currency::Usd);
        assert_eq!(investment.capital().currency(), Currency::Usd);
        assert_eq!(investment.taxes().currency(), Currency::Usd);
        assert_eq!(investment.tna_basis_points(), 2550);
    }

    #[test]
    fn constructor_predeterminado_usa_ars() {
        let investment = Investment::new("2026-01", "100,00", "25", "30", "0").unwrap();

        assert_eq!(investment.currency(), Currency::Ars);
    }

    #[test]
    fn rechaza_capital_e_impuestos_con_monedas_distintas() {
        let error = Investment::from_parts(
            Month::parse("2026-01").unwrap(),
            ars(100_000),
            2500,
            30,
            usd(10_000),
        )
        .unwrap_err();

        assert!(matches!(
            error,
            DomainError::CurrencyMismatch {
                expected: "ARS",
                found: "USD"
            }
        ));
    }

    #[test]
    fn acepta_capital_positivo_y_cero() {
        let positive = Investment::new("2026-01", "10000,10", "25", "30", "0").unwrap();
        let zero = Investment::new("2026-01", "0", "25", "30", "0").unwrap();

        assert_eq!(positive.capital().minor_units(), 1_000_010);
        assert_eq!(zero.capital().minor_units(), 0);
    }

    #[test]
    fn acepta_impuestos_positivos_y_cero() {
        let positive = Investment::new("2026-01", "100,00", "25", "30", "100,50").unwrap();
        let zero = Investment::new("2026-01", "100,00", "25", "30", "0,00").unwrap();

        assert_eq!(positive.taxes().minor_units(), 10_050);
        assert_eq!(zero.taxes().minor_units(), 0);
    }

    #[test]
    fn rechaza_capital_negativo() {
        let error = Investment::new("2026-01", "-1,00", "25", "30", "0").unwrap_err();

        assert_validation_field(error, "capital");
    }

    #[test]
    fn rechaza_impuestos_negativos() {
        let error = Investment::new("2026-01", "100,00", "25", "30", "-1,00").unwrap_err();

        assert_validation_field(error, "taxes");
    }

    #[test]
    fn rechaza_capital_vacio() {
        let error = Investment::new("2026-01", "", "25", "30", "0").unwrap_err();

        assert_validation_field(error, "capital");
    }

    #[test]
    fn rechaza_impuestos_no_numericos() {
        let error = Investment::new("2026-01", "100,00", "25", "30", "abc").unwrap_err();

        assert_validation_field(error, "taxes");
    }

    #[test]
    fn rechaza_capital_con_mas_de_dos_decimales() {
        let error = Investment::new("2026-01", "100,999", "25", "30", "0").unwrap_err();

        assert_validation_field(error, "capital");
    }

    #[test]
    fn rechaza_impuestos_con_mas_de_dos_decimales() {
        let error = Investment::new("2026-01", "100,00", "25", "30", "1,999").unwrap_err();

        assert_validation_field(error, "taxes");
    }

    #[test]
    fn desbordamiento_de_capital_remapea_campo_y_operacion() {
        let error =
            Investment::new("2026-01", "92233720368547758.08", "25", "30", "0").unwrap_err();

        match error {
            DomainError::ArithmeticOverflow { field, operation } => {
                assert_eq!(field, "capital");
                assert_eq!(operation, "análisis del importe");
            }
            other => panic!("unexpected error: {:?}", other),
        }
    }

    #[test]
    fn desbordamiento_de_impuestos_remapea_campo() {
        let error =
            Investment::new("2026-01", "100,00", "25", "30", "92233720368547758.08").unwrap_err();

        assert_overflow_field(error, "taxes");
    }

    #[test]
    fn conserva_centavos_exactamente() {
        let investment = Investment::new("2026-01", "10000,10", "25", "30", "0,01").unwrap();

        assert_eq!(investment.capital().minor_units(), 1_000_010);
        assert_eq!(investment.taxes().minor_units(), 1);
    }

    #[test]
    fn tna_entero_convertido_a_basis_points() {
        let investment = Investment::new("2026-01", "100,00", "25", "30", "0").unwrap();

        assert_eq!(investment.tna_basis_points(), 2500);
    }

    #[test]
    fn tna_un_decimal_con_coma() {
        let investment = Investment::new("2026-01", "100,00", "25,5", "30", "0").unwrap();

        assert_eq!(investment.tna_basis_points(), 2550);
    }

    #[test]
    fn tna_dos_decimales_con_punto() {
        let investment = Investment::new("2026-01", "100,00", "25.50", "30", "0").unwrap();

        assert_eq!(investment.tna_basis_points(), 2550);
    }

    #[test]
    fn tna_cero() {
        let integer = Investment::new("2026-01", "100,00", "0", "30", "0").unwrap();
        let decimal = Investment::new("2026-01", "100,00", "0,00", "30", "0").unwrap();

        assert_eq!(integer.tna_basis_points(), 0);
        assert_eq!(decimal.tna_basis_points(), 0);
    }

    #[test]
    fn rechaza_tna_con_tres_decimales() {
        let error = Investment::new("2026-01", "100,00", "25,505", "30", "0").unwrap_err();

        assert_validation_field(error, "tna");
    }

    #[test]
    fn rechaza_tna_vacio() {
        let error = Investment::new("2026-01", "100,00", "", "30", "0").unwrap_err();

        assert_validation_field(error, "tna");
    }

    #[test]
    fn rechaza_tna_no_numerico() {
        let error = Investment::new("2026-01", "100,00", "abc", "30", "0").unwrap_err();

        assert_validation_field(error, "tna");
    }

    #[test]
    fn rechaza_tna_con_signo() {
        for value in ["-1", "+1"] {
            let error = Investment::new("2026-01", "100,00", value, "30", "0").unwrap_err();
            assert_validation_field(error, "tna");
        }
    }

    #[test]
    fn rechaza_tna_cientifica() {
        let error = Investment::new("2026-01", "100,00", "1e2", "30", "0").unwrap_err();

        assert_validation_field(error, "tna");
    }

    #[test]
    fn rechaza_tna_con_miles() {
        let error = Investment::new("2026-01", "100,00", "1.000,50", "30", "0").unwrap_err();

        assert_validation_field(error, "tna");
    }

    #[test]
    fn rechaza_tna_con_multiples_separadores() {
        let error = Investment::new("2026-01", "100,00", "25,5.0", "30", "0").unwrap_err();

        assert_validation_field(error, "tna");
    }

    #[test]
    fn rechaza_tna_con_espacios() {
        for value in [" 25", "25 "] {
            let error = Investment::new("2026-01", "100,00", value, "30", "0").unwrap_err();
            assert_validation_field(error, "tna");
        }
    }

    #[test]
    fn desbordamiento_de_tna() {
        let error =
            Investment::new("2026-01", "100,00", "18446744073709551616", "30", "0").unwrap_err();

        assert_overflow_field(error, "tna");
    }

    #[test]
    fn desbordamiento_de_tna_en_u128() {
        let error = Investment::new(
            "2026-01",
            "100,00",
            "99999999999999999999999999999999",
            "30",
            "0",
        )
        .unwrap_err();

        assert_overflow_field(error, "tna");
    }

    #[test]
    fn plazo_valido() {
        let investment = Investment::new("2026-01", "100,00", "25", "30", "0").unwrap();

        assert_eq!(investment.term_days(), 30);
    }

    #[test]
    fn plazo_cero() {
        let investment = Investment::new("2026-01", "100,00", "25", "0", "0").unwrap();

        assert_eq!(investment.term_days(), 0);
    }

    #[test]
    fn plazo_maximo_u32() {
        let investment = Investment::new("2026-01", "100,00", "25", "4294967295", "0").unwrap();

        assert_eq!(investment.term_days(), u32::MAX);
    }

    #[test]
    fn rechaza_plazo_vacio() {
        let error = Investment::new("2026-01", "100,00", "25", "", "0").unwrap_err();

        assert_validation_field(error, "termDays");
    }

    #[test]
    fn rechaza_plazo_no_numerico() {
        let error = Investment::new("2026-01", "100,00", "25", "abc", "0").unwrap_err();

        assert_validation_field(error, "termDays");
    }

    #[test]
    fn rechaza_plazo_decimal() {
        let error = Investment::new("2026-01", "100,00", "25", "30,5", "0").unwrap_err();

        assert_validation_field(error, "termDays");
    }

    #[test]
    fn rechaza_plazo_negativo() {
        let error = Investment::new("2026-01", "100,00", "25", "-1", "0").unwrap_err();

        assert_validation_field(error, "termDays");
    }

    #[test]
    fn desbordamiento_de_plazo() {
        let error = Investment::new("2026-01", "100,00", "25", "4294967296", "0").unwrap_err();

        assert_overflow_field(error, "termDays");
    }

    #[test]
    fn desbordamiento_de_plazo_con_digitos_largos() {
        let error = Investment::new(
            "2026-01",
            "100,00",
            "25",
            "999999999999999999999999999999",
            "0",
        )
        .unwrap_err();

        assert_overflow_field(error, "termDays");
    }

    #[test]
    fn from_parts_reconstruye_inversion() {
        let investment = Investment::from_parts(
            Month::parse("2026-03").unwrap(),
            ars(1_000_010),
            2550,
            30,
            ars(10_000),
        )
        .unwrap();

        assert_eq!(investment.month(), Month::parse("2026-03").unwrap());
        assert_eq!(investment.capital().minor_units(), 1_000_010);
        assert_eq!(investment.currency(), Currency::Ars);
        assert_eq!(investment.tna_basis_points(), 2550);
        assert_eq!(investment.term_days(), 30);
        assert_eq!(investment.taxes().minor_units(), 10_000);
    }

    #[test]
    fn from_parts_rechaza_mes_invalido() {
        let error = Investment::from_parts(
            Month { year: 0, month: 1 },
            ars(100_000),
            2500,
            30,
            ars(10_000),
        )
        .unwrap_err();

        assert_validation_field(error, "month");
    }

    #[test]
    fn from_parts_rechaza_capital_negativo() {
        let error =
            Investment::from_parts(Month::parse("2026-01").unwrap(), ars(-1), 2500, 30, ars(0))
                .unwrap_err();

        assert_validation_field(error, "capital");
    }

    #[test]
    fn from_parts_rechaza_impuestos_negativos() {
        let error = Investment::from_parts(
            Month::parse("2026-01").unwrap(),
            ars(100_000),
            2500,
            30,
            ars(-1),
        )
        .unwrap_err();

        assert_validation_field(error, "taxes");
    }

    #[test]
    fn from_parts_rechaza_monedas_distintas_en_usd() {
        let error = Investment::from_parts(
            Month::parse("2026-01").unwrap(),
            usd(100_000),
            2500,
            30,
            ars(10_000),
        )
        .unwrap_err();

        assert!(matches!(
            error,
            DomainError::CurrencyMismatch {
                expected: "USD",
                found: "ARS"
            }
        ));
    }

    #[test]
    fn los_getters_devuelven_los_valores_de_la_entidad() {
        let investment = Investment::new_with_currency(
            "2026-04",
            "250,75",
            Currency::Usd,
            "12,25",
            "45",
            "10,50",
        )
        .unwrap();

        assert_eq!(investment.month().code(), "2026-04");
        assert_eq!(investment.capital().minor_units(), 25_075);
        assert_eq!(investment.currency(), Currency::Usd);
        assert_eq!(investment.tna_basis_points(), 1225);
        assert_eq!(investment.term_days(), 45);
        assert_eq!(investment.taxes().minor_units(), 1_050);
    }

    #[test]
    fn no_usa_tipos_de_punto_flotante() {
        let source = include_str!("investment.rs").as_bytes();
        let narrow: &[u8] = &[0x66, 0x33, 0x32];
        let wide: &[u8] = &[0x66, 0x36, 0x34];

        assert!(!source.windows(narrow.len()).any(|window| window == narrow));
        assert!(!source.windows(wide.len()).any(|window| window == wide));
    }
}
