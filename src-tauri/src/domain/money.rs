use std::cmp::Ordering;

use crate::domain::error::DomainError;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Currency {
    Ars,
    Usd,
}

impl Currency {
    pub const fn code(self) -> &'static str {
        match self {
            Self::Ars => "ARS",
            Self::Usd => "USD",
        }
    }

    pub const fn minor_unit_exponent(self) -> u8 {
        2
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Money {
    minor_units: i64,
    currency: Currency,
}

impl Money {
    pub const fn from_minor_units(minor_units: i64, currency: Currency) -> Self {
        Self {
            minor_units,
            currency,
        }
    }

    pub const fn zero(currency: Currency) -> Self {
        Self::from_minor_units(0, currency)
    }

    pub const fn minor_units(&self) -> i64 {
        self.minor_units
    }

    pub const fn currency(&self) -> Currency {
        self.currency
    }

    pub fn from_major_decimal(value: &str, currency: Currency) -> Result<Self, DomainError> {
        let bytes = value.as_bytes();
        let (negative, start) = match bytes.first() {
            Some(b'-') => (true, 1),
            Some(_) => (false, 0),
            None => return Err(invalid_amount("El importe no puede estar vacío.")),
        };

        if start >= bytes.len() {
            return Err(invalid_amount("El importe no contiene dígitos."));
        }

        let mut major = 0_u128;
        let mut fraction = 0_u128;
        let mut fraction_digits = 0_u8;
        let mut major_digits = 0_usize;
        let mut has_separator = false;

        for &byte in &bytes[start..] {
            match byte {
                b'0'..=b'9' => {
                    let digit = u128::from(byte - b'0');
                    if has_separator {
                        if fraction_digits >= 2 {
                            return Err(invalid_amount(
                                "El importe admite como máximo dos decimales.",
                            ));
                        }
                        fraction = fraction
                            .checked_mul(10)
                            .and_then(|value| value.checked_add(digit))
                            .ok_or_else(|| overflow_at("amount", "análisis del importe"))?;
                        fraction_digits += 1;
                    } else {
                        major = major
                            .checked_mul(10)
                            .and_then(|value| value.checked_add(digit))
                            .ok_or_else(|| overflow_at("amount", "análisis del importe"))?;
                        major_digits += 1;
                    }
                }
                b',' | b'.' => {
                    if has_separator {
                        return Err(invalid_amount(
                            "El importe contiene más de un separador decimal.",
                        ));
                    }
                    if major_digits == 0 {
                        return Err(invalid_amount("El importe debe comenzar con dígitos."));
                    }
                    has_separator = true;
                }
                _ => {
                    return Err(invalid_amount(
                        "El importe contiene caracteres no permitidos.",
                    ));
                }
            }
        }

        if has_separator && fraction_digits == 0 {
            return Err(invalid_amount(
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
            .ok_or_else(|| overflow_at("amount", "análisis del importe"))?;
        let magnitude = major
            .checked_mul(100)
            .and_then(|value| value.checked_add(scaled_fraction))
            .ok_or_else(|| overflow_at("amount", "análisis del importe"))?;
        let minor_units = signed_minor_units_from_magnitude(magnitude, negative)?;

        Ok(Self::from_minor_units(minor_units, currency))
    }

    pub fn checked_add(&self, rhs: &Money) -> Result<Self, DomainError> {
        self.ensure_same_currency(rhs)?;
        let minor_units = self
            .minor_units
            .checked_add(rhs.minor_units)
            .ok_or_else(|| overflow_at("amount", "suma"))?;
        Ok(Self::from_minor_units(minor_units, self.currency))
    }

    pub fn checked_sub(&self, rhs: &Money) -> Result<Self, DomainError> {
        self.ensure_same_currency(rhs)?;
        let minor_units = self
            .minor_units
            .checked_sub(rhs.minor_units)
            .ok_or_else(|| overflow_at("amount", "resta"))?;
        Ok(Self::from_minor_units(minor_units, self.currency))
    }

    pub fn checked_mul_scalar(&self, scalar: u64) -> Result<Self, DomainError> {
        let product = i128::from(self.minor_units)
            .checked_mul(i128::from(scalar))
            .ok_or_else(|| overflow_at("amount", "multiplicación"))?;
        let minor_units =
            i64::try_from(product).map_err(|_| overflow_at("amount", "multiplicación"))?;
        Ok(Self::from_minor_units(minor_units, self.currency))
    }

    pub fn checked_div_exact(&self, divisor: u64) -> Result<ExactMoney, DomainError> {
        if divisor == 0 {
            return Err(invalid_divisor());
        }
        ExactMoney::from_parts(
            i128::from(self.minor_units),
            u128::from(divisor),
            self.currency,
        )
    }

    pub fn checked_neg(&self) -> Result<Self, DomainError> {
        let minor_units = self
            .minor_units
            .checked_neg()
            .ok_or_else(|| overflow_at("amount", "negación"))?;
        Ok(Self::from_minor_units(minor_units, self.currency))
    }

    pub fn cmp_exact(&self, rhs: &Money) -> Result<Ordering, DomainError> {
        self.ensure_same_currency(rhs)?;
        Ok(self.minor_units.cmp(&rhs.minor_units))
    }

    pub fn eq_exact(&self, rhs: &Money) -> Result<bool, DomainError> {
        Ok(self.cmp_exact(rhs)? == Ordering::Equal)
    }

    fn ensure_same_currency(&self, rhs: &Money) -> Result<(), DomainError> {
        if self.currency == rhs.currency {
            Ok(())
        } else {
            Err(DomainError::currency_mismatch(
                self.currency.code(),
                rhs.currency.code(),
            ))
        }
    }
}

#[derive(Debug, Clone)]
pub struct ExactMoney {
    numerator_minor: i128,
    denominator: u128,
    currency: Currency,
}

impl ExactMoney {
    pub fn from_parts(
        numerator_minor: i128,
        denominator: u128,
        currency: Currency,
    ) -> Result<Self, DomainError> {
        if denominator == 0 {
            return Err(DomainError::validation(
                "denominator",
                "El denominador debe ser mayor que cero.",
            ));
        }

        let divisor = gcd(numerator_minor.unsigned_abs(), denominator);
        let numerator = divide_signed_by_unsigned(numerator_minor, divisor, "normalización")?;
        let denominator = denominator / divisor;

        Ok(Self {
            numerator_minor: numerator,
            denominator,
            currency,
        })
    }

    pub fn from_money(money: Money) -> Self {
        Self {
            numerator_minor: i128::from(money.minor_units()),
            denominator: 1,
            currency: money.currency(),
        }
    }

    pub const fn numerator_minor(&self) -> i128 {
        self.numerator_minor
    }

    pub const fn denominator(&self) -> u128 {
        self.denominator
    }

    pub const fn currency(&self) -> Currency {
        self.currency
    }

    pub fn checked_add(&self, rhs: &Self) -> Result<Self, DomainError> {
        let (left, right, denominator) = self.cross_products(rhs)?;
        let numerator = left
            .checked_add(right)
            .ok_or_else(|| overflow_at("amount", "suma de fracciones"))?;
        Self::from_parts(numerator, denominator, self.currency)
    }

    pub fn checked_sub(&self, rhs: &Self) -> Result<Self, DomainError> {
        let (left, right, denominator) = self.cross_products(rhs)?;
        let numerator = left
            .checked_sub(right)
            .ok_or_else(|| overflow_at("amount", "resta de fracciones"))?;
        Self::from_parts(numerator, denominator, self.currency)
    }

    pub fn checked_mul_scalar(&self, scalar: u64) -> Result<Self, DomainError> {
        let numerator = multiply_signed_by_unsigned(
            self.numerator_minor,
            u128::from(scalar),
            "multiplicación de fracciones",
        )?;
        Self::from_parts(numerator, self.denominator, self.currency)
    }

    pub fn checked_neg(&self) -> Result<Self, DomainError> {
        let numerator = self
            .numerator_minor
            .checked_neg()
            .ok_or_else(|| overflow_at("amount", "negación de fracciones"))?;
        Self::from_parts(numerator, self.denominator, self.currency)
    }

    pub fn cmp_exact(&self, rhs: &Self) -> Result<Ordering, DomainError> {
        let (left, right, _) = self.cross_products(rhs)?;
        Ok(left.cmp(&right))
    }

    pub fn eq_exact(&self, rhs: &Self) -> Result<bool, DomainError> {
        Ok(self.cmp_exact(rhs)? == Ordering::Equal)
    }

    fn ensure_same_currency(&self, rhs: &Self) -> Result<(), DomainError> {
        if self.currency == rhs.currency {
            Ok(())
        } else {
            Err(DomainError::currency_mismatch(
                self.currency.code(),
                rhs.currency.code(),
            ))
        }
    }

    fn cross_products(&self, rhs: &Self) -> Result<(i128, i128, u128), DomainError> {
        self.ensure_same_currency(rhs)?;
        let common = gcd(self.denominator, rhs.denominator);
        let left_factor = rhs.denominator / common;
        let right_factor = self.denominator / common;
        let denominator = self
            .denominator
            .checked_mul(left_factor)
            .ok_or_else(|| overflow_at("denominator", "multiplicación de denominadores"))?;
        let left = multiply_signed_by_unsigned(
            self.numerator_minor,
            left_factor,
            "multiplicación cruzada",
        )?;
        let right = multiply_signed_by_unsigned(
            rhs.numerator_minor,
            right_factor,
            "multiplicación cruzada",
        )?;
        Ok((left, right, denominator))
    }
}

fn invalid_amount(message: &str) -> DomainError {
    DomainError::validation("amount", message)
}

fn invalid_divisor() -> DomainError {
    DomainError::validation("divisor", "El divisor debe ser mayor que cero.")
}

fn overflow_at(field: &'static str, operation: &'static str) -> DomainError {
    DomainError::arithmetic_overflow(field, operation)
}

fn signed_minor_units_from_magnitude(magnitude: u128, negative: bool) -> Result<i64, DomainError> {
    let operation = "análisis del importe";
    let max_negative_magnitude = u128::from(i64::MAX.unsigned_abs()) + 1;

    if negative {
        if magnitude > max_negative_magnitude {
            return Err(overflow_at("amount", operation));
        }
        if magnitude == max_negative_magnitude {
            return Ok(i64::MIN);
        }
        let value = i64::try_from(magnitude).map_err(|_| overflow_at("amount", operation))?;
        Ok(-value)
    } else {
        i64::try_from(magnitude).map_err(|_| overflow_at("amount", operation))
    }
}

fn gcd(mut left: u128, mut right: u128) -> u128 {
    while right != 0 {
        let remainder = left % right;
        left = right;
        right = remainder;
    }
    left
}

fn signed_from_magnitude(
    magnitude: u128,
    negative: bool,
    operation: &'static str,
) -> Result<i128, DomainError> {
    if negative {
        if magnitude == (i128::MAX as u128) + 1 {
            return Ok(i128::MIN);
        }
        let value = i128::try_from(magnitude).map_err(|_| overflow_at("amount", operation))?;
        Ok(-value)
    } else {
        i128::try_from(magnitude).map_err(|_| overflow_at("amount", operation))
    }
}

fn multiply_signed_by_unsigned(
    value: i128,
    factor: u128,
    operation: &'static str,
) -> Result<i128, DomainError> {
    let magnitude = value
        .unsigned_abs()
        .checked_mul(factor)
        .ok_or_else(|| overflow_at("amount", operation))?;
    signed_from_magnitude(magnitude, value < 0, operation)
}

fn divide_signed_by_unsigned(
    value: i128,
    divisor: u128,
    operation: &'static str,
) -> Result<i128, DomainError> {
    if divisor == 0 {
        return Err(overflow_at("amount", operation));
    }
    let magnitude = value.unsigned_abs() / divisor;
    signed_from_magnitude(magnitude, value < 0, operation)
}

#[cfg(test)]
mod tests {
    use std::cmp::Ordering;

    use super::*;

    fn ars(minor_units: i64) -> Money {
        Money::from_minor_units(minor_units, Currency::Ars)
    }

    fn usd(minor_units: i64) -> Money {
        Money::from_minor_units(minor_units, Currency::Usd)
    }

    #[test]
    fn currency_expone_codigo_y_exponente() {
        assert_eq!(Currency::Ars.code(), "ARS");
        assert_eq!(Currency::Usd.code(), "USD");
        assert_eq!(Currency::Ars.minor_unit_exponent(), 2);
        assert_eq!(Currency::Usd.minor_unit_exponent(), 2);
    }

    #[test]
    fn parsea_importe_con_coma_decimal_sin_punto_flotante() {
        let money = Money::from_major_decimal("10000,10", Currency::Ars).unwrap();

        assert_eq!(money.minor_units(), 1_000_010);
        assert_eq!(money.currency(), Currency::Ars);
    }

    #[test]
    fn parsea_importe_con_punto_decimal() {
        let money = Money::from_major_decimal("10000.10", Currency::Usd).unwrap();

        assert_eq!(money.minor_units(), 1_000_010);
        assert_eq!(money.currency(), Currency::Usd);
    }

    #[test]
    fn parsea_enteros_un_decimal_y_dos_decimales() {
        assert_eq!(
            Money::from_major_decimal("100", Currency::Ars)
                .unwrap()
                .minor_units(),
            10_000
        );
        assert_eq!(
            Money::from_major_decimal("100,5", Currency::Ars)
                .unwrap()
                .minor_units(),
            10_050
        );
        assert_eq!(
            Money::from_major_decimal("100.05", Currency::Ars)
                .unwrap()
                .minor_units(),
            10_005
        );
    }

    #[test]
    fn parsea_negativos_y_limites() {
        let negative = Money::from_major_decimal("-10000,10", Currency::Usd).unwrap();
        assert_eq!(negative.minor_units(), -1_000_010);

        let maximum = Money::from_major_decimal("92233720368547758.07", Currency::Ars).unwrap();
        assert_eq!(maximum.minor_units(), i64::MAX);
    }

    #[test]
    fn parsea_valor_minimo_negativo() {
        let money = Money::from_major_decimal("-92233720368547758.08", Currency::Ars).unwrap();

        assert_eq!(money.minor_units(), i64::MIN);
        assert_eq!(money.currency(), Currency::Ars);
    }

    #[test]
    fn parsea_valor_maximo_positivo() {
        let money = Money::from_major_decimal("92233720368547758.07", Currency::Usd).unwrap();

        assert_eq!(money.minor_units(), i64::MAX);
        assert_eq!(money.currency(), Currency::Usd);
    }

    #[test]
    fn rechaza_valor_negativo_menor_al_minimo() {
        assert!(matches!(
            Money::from_major_decimal("-92233720368547758.09", Currency::Ars),
            Err(DomainError::ArithmeticOverflow {
                field: "amount",
                ..
            })
        ));
        assert!(matches!(
            Money::from_major_decimal("-9223372036854775808", Currency::Ars),
            Err(DomainError::ArithmeticOverflow {
                field: "amount",
                ..
            })
        ));
    }

    #[test]
    fn rechaza_parser_con_overflow() {
        assert!(matches!(
            Money::from_major_decimal("92233720368547758.08", Currency::Ars),
            Err(DomainError::ArithmeticOverflow {
                field: "amount",
                ..
            })
        ));
        assert!(matches!(
            Money::from_major_decimal("9223372036854775808", Currency::Ars),
            Err(DomainError::ArithmeticOverflow {
                field: "amount",
                ..
            })
        ));
    }

    #[test]
    fn parser_rechaza_formatos_ambiguos() {
        let invalid = [
            "", "-", "+100,00", "100,", ",10", "100,000", "100.000", "1.000,00", "1,000.00",
            "1 000,00", "$100,00", "100,00 ", "100e2", "abc",
        ];

        for value in invalid {
            assert!(
                matches!(
                    Money::from_major_decimal(value, Currency::Ars),
                    Err(DomainError::Validation { .. })
                ),
                "se aceptó un formato inválido: {value}"
            );
        }
    }

    #[test]
    fn suma_y_resta_conservan_centavos() {
        let sum = ars(1_000_010).checked_add(&ars(1)).unwrap();
        let difference = ars(1_000_010).checked_sub(&ars(10)).unwrap();

        assert_eq!(sum.minor_units(), 1_000_011);
        assert_eq!(difference.minor_units(), 1_000_000);
        assert_eq!(sum.currency(), Currency::Ars);
    }

    #[test]
    fn rechaza_operaciones_entre_monedas_distintas() {
        assert!(matches!(
            ars(100).checked_add(&usd(100)),
            Err(DomainError::CurrencyMismatch {
                expected: "ARS",
                found: "USD"
            })
        ));
        assert!(matches!(
            ars(100).checked_sub(&usd(50)),
            Err(DomainError::CurrencyMismatch { .. })
        ));
        assert!(matches!(
            ars(100).cmp_exact(&usd(100)),
            Err(DomainError::CurrencyMismatch { .. })
        ));
    }

    #[test]
    fn detecta_overflow_en_suma_y_resta() {
        assert!(matches!(
            ars(i64::MAX).checked_add(&ars(1)),
            Err(DomainError::ArithmeticOverflow {
                field: "amount",
                ..
            })
        ));
        assert!(matches!(
            ars(i64::MIN).checked_sub(&ars(1)),
            Err(DomainError::ArithmeticOverflow {
                field: "amount",
                ..
            })
        ));
    }

    #[test]
    fn multiplicacion_por_escalar_es_checked() {
        assert_eq!(
            ars(250_000).checked_mul_scalar(4).unwrap().minor_units(),
            1_000_000
        );
        assert_eq!(ars(1_000).checked_mul_scalar(0).unwrap().minor_units(), 0);
        assert!(matches!(
            ars(i64::MAX).checked_mul_scalar(2),
            Err(DomainError::ArithmeticOverflow { .. })
        ));
    }

    #[test]
    fn division_exacta_conserva_fraccion() {
        let exact = Money::from_major_decimal("10000,10", Currency::Ars)
            .unwrap()
            .checked_div_exact(3)
            .unwrap();
        let integer = ars(1_000_000).checked_div_exact(4).unwrap();

        assert_eq!(exact.numerator_minor(), 1_000_010);
        assert_eq!(exact.denominator(), 3);
        assert_eq!(integer.numerator_minor(), 250_000);
        assert_eq!(integer.denominator(), 1);
    }

    #[test]
    fn division_rechaza_denominador_cero() {
        assert!(matches!(
            ars(100).checked_div_exact(0),
            Err(DomainError::Validation {
                field: "divisor",
                ..
            })
        ));
    }

    #[test]
    fn normaliza_fracciones() {
        let positive = ExactMoney::from_parts(2, 4, Currency::Ars).unwrap();
        let negative = ExactMoney::from_parts(-2, 4, Currency::Ars).unwrap();
        let zero = ExactMoney::from_parts(0, 5, Currency::Usd).unwrap();
        let maximum_numerator =
            ExactMoney::from_parts(i128::MAX, u128::MAX, Currency::Ars).unwrap();
        let minimum_numerator = ExactMoney::from_parts(i128::MIN, 1, Currency::Ars).unwrap();
        let maximum_denominator = ExactMoney::from_parts(1, u128::MAX, Currency::Usd).unwrap();

        assert_eq!((positive.numerator_minor(), positive.denominator()), (1, 2));
        assert_eq!(
            (negative.numerator_minor(), negative.denominator()),
            (-1, 2)
        );
        assert_eq!((zero.numerator_minor(), zero.denominator()), (0, 1));
        assert_eq!(maximum_numerator.numerator_minor(), i128::MAX);
        assert_eq!(maximum_numerator.denominator(), u128::MAX);
        assert_eq!(minimum_numerator.numerator_minor(), i128::MIN);
        assert_eq!(minimum_numerator.denominator(), 1);
        assert_eq!(maximum_denominator.numerator_minor(), 1);
        assert_eq!(maximum_denominator.denominator(), u128::MAX);
    }

    #[test]
    fn rechaza_denominador_cero_en_fracciones() {
        assert!(matches!(
            ExactMoney::from_parts(1, 0, Currency::Ars),
            Err(DomainError::Validation {
                field: "denominator",
                ..
            })
        ));
    }

    #[test]
    fn compara_fracciones_exactamente() {
        let half = ExactMoney::from_parts(1, 2, Currency::Ars).unwrap();
        let equivalent = ExactMoney::from_parts(2, 4, Currency::Ars).unwrap();
        let third = ExactMoney::from_parts(1, 3, Currency::Ars).unwrap();
        let negative_half = ExactMoney::from_parts(-1, 2, Currency::Ars).unwrap();
        let negative_one = ExactMoney::from_parts(-1, 1, Currency::Ars).unwrap();
        let other_currency = ExactMoney::from_parts(1, 2, Currency::Usd).unwrap();

        assert!(half.eq_exact(&equivalent).unwrap());
        assert_eq!(third.cmp_exact(&half).unwrap(), Ordering::Less);
        assert_eq!(
            negative_half.cmp_exact(&negative_one).unwrap(),
            Ordering::Greater
        );
        assert!(matches!(
            half.cmp_exact(&other_currency),
            Err(DomainError::CurrencyMismatch { .. })
        ));
    }

    #[test]
    fn suma_y_resta_fracciones_exactamente() {
        let half = ExactMoney::from_parts(1, 2, Currency::Ars).unwrap();
        let third = ExactMoney::from_parts(1, 3, Currency::Ars).unwrap();
        let sum = half.checked_add(&third).unwrap();
        let difference = half.checked_sub(&third).unwrap();
        let other_currency = ExactMoney::from_parts(1, 3, Currency::Usd).unwrap();

        assert_eq!((sum.numerator_minor(), sum.denominator()), (5, 6));
        assert_eq!(
            (difference.numerator_minor(), difference.denominator()),
            (1, 6)
        );
        assert!(matches!(
            half.checked_add(&other_currency),
            Err(DomainError::CurrencyMismatch { .. })
        ));
    }

    #[test]
    fn detecta_overflow_en_multiplicaciones_cruzadas() {
        let maximum = ExactMoney::from_parts(i128::MAX, 1, Currency::Ars).unwrap();
        let half = ExactMoney::from_parts(1, 2, Currency::Ars).unwrap();
        let large_denominator = ExactMoney::from_parts(1, u128::MAX, Currency::Ars).unwrap();
        let previous_denominator = ExactMoney::from_parts(1, u128::MAX - 1, Currency::Ars).unwrap();
        let minimum = ExactMoney::from_parts(i128::MIN, 1, Currency::Ars).unwrap();

        assert!(matches!(
            maximum.checked_add(&half),
            Err(DomainError::ArithmeticOverflow { .. })
        ));
        assert!(matches!(
            large_denominator.checked_add(&previous_denominator),
            Err(DomainError::ArithmeticOverflow {
                field: "denominator",
                ..
            })
        ));
        assert!(matches!(
            minimum.checked_mul_scalar(2),
            Err(DomainError::ArithmeticOverflow { .. })
        ));
    }

    #[test]
    fn permite_valores_negativos_internos() {
        let negative_money = usd(-1);
        let subtracted = ars(1).checked_sub(&ars(2)).unwrap();
        let minimum = ars(i64::MIN);
        let fraction = ExactMoney::from_parts(-1, 2, Currency::Ars).unwrap();

        assert_eq!(negative_money.minor_units(), -1);
        assert_eq!(subtracted.minor_units(), -1);
        assert!(matches!(
            minimum.checked_neg(),
            Err(DomainError::ArithmeticOverflow { .. })
        ));
        assert_eq!(fraction.checked_neg().unwrap().numerator_minor(), 1);
    }

    #[test]
    fn crea_fraccion_desde_importe() {
        let exact = ExactMoney::from_money(ars(1_000_010));

        assert_eq!(exact.numerator_minor(), 1_000_010);
        assert_eq!(exact.denominator(), 1);
        assert_eq!(exact.currency(), Currency::Ars);
    }

    #[test]
    fn modulo_no_usa_punto_flotante_binario() {
        let source = include_str!("money.rs");
        let first_marker = ["f", "3", "2"].concat();
        let second_marker = ["f", "6", "4"].concat();

        assert!(!source.contains(first_marker.as_str()));
        assert!(!source.contains(second_marker.as_str()));
    }
}
