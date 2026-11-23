use std::num::NonZeroU32;

use crate::domain::error::DomainError;
use crate::domain::money::{Currency, ExactMoney, Money};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ServiceStatus {
    Active,
    Frozen,
}

#[derive(Debug, Clone)]
pub struct Service {
    name: String,
    total_cost: Money,
    shared_people_count: NonZeroU32,
    notes: Option<String>,
    status: ServiceStatus,
}

impl Service {
    pub fn new(
        name: impl Into<String>,
        total_cost: &str,
        shared_people_count: u32,
        notes: Option<String>,
    ) -> Result<Self, DomainError> {
        Self::new_with_currency(name, total_cost, Currency::Ars, shared_people_count, notes)
    }

    pub fn new_with_currency(
        name: impl Into<String>,
        total_cost: &str,
        currency: Currency,
        shared_people_count: u32,
        notes: Option<String>,
    ) -> Result<Self, DomainError> {
        let name = name.into();
        validate_name(&name)?;
        let total_cost = parse_total_cost(total_cost, currency)?;
        let shared_people_count = NonZeroU32::new(shared_people_count).ok_or_else(|| {
            DomainError::validation(
                "sharedPeopleCount",
                "La cantidad de personas debe ser mayor o igual a 1.",
            )
        })?;

        Ok(Self {
            name,
            total_cost,
            shared_people_count,
            notes,
            status: ServiceStatus::Active,
        })
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn total_cost(&self) -> Money {
        self.total_cost
    }

    pub fn currency(&self) -> Currency {
        self.total_cost.currency()
    }

    pub fn shared_people_count(&self) -> NonZeroU32 {
        self.shared_people_count
    }

    pub fn notes(&self) -> Option<&str> {
        self.notes.as_deref()
    }

    pub fn status(&self) -> ServiceStatus {
        self.status
    }

    pub fn my_share(&self) -> Result<ExactMoney, DomainError> {
        self.total_cost
            .checked_div_exact(u64::from(self.shared_people_count.get()))
    }
}

#[cfg(test)]
impl Service {
    pub(crate) fn new_frozen_with_currency(
        name: impl Into<String>,
        total_cost: &str,
        currency: Currency,
        shared_people_count: u32,
        notes: Option<String>,
    ) -> Result<Self, DomainError> {
        let mut service =
            Self::new_with_currency(name, total_cost, currency, shared_people_count, notes)?;
        service.status = ServiceStatus::Frozen;
        Ok(service)
    }
}

fn validate_name(name: &str) -> Result<(), DomainError> {
    if name.trim().is_empty() {
        return Err(DomainError::validation(
            "name",
            "El nombre del servicio es obligatorio.",
        ));
    }
    Ok(())
}

fn parse_total_cost(value: &str, currency: Currency) -> Result<Money, DomainError> {
    let total_cost = Money::from_major_decimal(value, currency).map_err(|error| match error {
        DomainError::Validation { message, .. } => DomainError::validation("totalCost", message),
        DomainError::ArithmeticOverflow { operation, .. } => {
            DomainError::arithmetic_overflow("totalCost", operation)
        }
        other => other,
    })?;

    if total_cost.minor_units() < 0 {
        return Err(DomainError::validation(
            "totalCost",
            "El costo total no puede ser negativo.",
        ));
    }

    Ok(total_cost)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn construye_servicio_valido_en_ars() {
        let service = Service::new("Spotify", "10000,10", 4, None).unwrap();

        assert_eq!(service.name(), "Spotify");
        assert_eq!(service.total_cost().minor_units(), 1_000_010);
        assert_eq!(service.currency(), Currency::Ars);
        assert_eq!(service.shared_people_count().get(), 4);
        assert_eq!(service.notes(), None);
        assert_eq!(service.status(), ServiceStatus::Active);
    }

    #[test]
    fn construye_servicio_valido_en_usd() {
        let service = Service::new_with_currency(
            "Netflix",
            "15.99",
            Currency::Usd,
            2,
            Some("familiar".to_string()),
        )
        .unwrap();

        assert_eq!(service.name(), "Netflix");
        assert_eq!(service.total_cost().minor_units(), 1_599);
        assert_eq!(service.currency(), Currency::Usd);
        assert_eq!(service.shared_people_count().get(), 2);
        assert_eq!(service.notes(), Some("familiar"));
        assert_eq!(service.status(), ServiceStatus::Active);
    }

    #[test]
    fn constructor_predeterminado_usa_ars() {
        let service = Service::new("Luz", "100,00", 1, None).unwrap();

        assert_eq!(service.currency(), Currency::Ars);
    }

    #[test]
    fn estado_inicial_es_activo() {
        let service = Service::new("Luz", "100,00", 1, None).unwrap();

        assert_eq!(service.status(), ServiceStatus::Active);
    }

    #[test]
    fn rechaza_nombre_vacio() {
        let error = Service::new("", "100,00", 1, None).unwrap_err();

        assert!(matches!(
            error,
            DomainError::Validation { field: "name", .. }
        ));
    }

    #[test]
    fn rechaza_nombre_compuesto_solo_por_espacios() {
        let error = Service::new("   ", "100,00", 1, None).unwrap_err();

        assert!(matches!(
            error,
            DomainError::Validation { field: "name", .. }
        ));
    }

    #[test]
    fn no_modifica_el_nombre_recibido() {
        let service = Service::new("  Spotify  ", "100,00", 1, None).unwrap();

        assert_eq!(service.name(), "  Spotify  ");
    }

    #[test]
    fn rechaza_costo_negativo() {
        let error = Service::new("Luz", "-100,00", 1, None).unwrap_err();

        assert!(matches!(
            error,
            DomainError::Validation {
                field: "totalCost",
                message,
                ..
            } if message == "El costo total no puede ser negativo."
        ));
    }

    #[test]
    fn rechaza_costo_con_mas_de_dos_decimales() {
        let error = Service::new("Luz", "100,999", 1, None).unwrap_err();

        assert!(matches!(
            error,
            DomainError::Validation {
                field: "totalCost",
                message,
                ..
            } if message.contains("dos decimales")
        ));
    }

    #[test]
    fn rechaza_formato_decimal_invalido() {
        let error = Service::new("Luz", "abc", 1, None).unwrap_err();

        assert!(matches!(
            error,
            DomainError::Validation {
                field: "totalCost",
                ..
            }
        ));
    }

    #[test]
    fn rechaza_cantidad_de_personas_cero() {
        let error = Service::new("Luz", "100,00", 0, None).unwrap_err();

        assert!(matches!(
            error,
            DomainError::Validation {
                field: "sharedPeopleCount",
                ..
            }
        ));
    }

    #[test]
    fn acepta_notas_none() {
        let service = Service::new("Luz", "100,00", 1, None).unwrap();

        assert_eq!(service.notes(), None);
    }

    #[test]
    fn acepta_notas_vacias() {
        let service = Service::new("Luz", "100,00", 1, Some(String::new())).unwrap();

        assert_eq!(service.notes(), Some(""));
    }

    #[test]
    fn conserva_centavos_exactamente() {
        let service = Service::new("Spotify", "10000,10", 4, None).unwrap();

        assert_eq!(service.total_cost().minor_units(), 1_000_010);
    }

    #[test]
    fn conserva_moneda_seleccionada() {
        let service =
            Service::new_with_currency("Netflix", "9,99", Currency::Usd, 1, None).unwrap();

        assert_eq!(service.currency(), Currency::Usd);
        assert_eq!(service.total_cost().currency(), Currency::Usd);
    }

    #[test]
    fn mi_parte_divide_10000_entre_4() {
        let service = Service::new("Spotify", "10000,00", 4, None).unwrap();
        let share = service.my_share().unwrap();

        assert_eq!(share.numerator_minor(), 250_000);
        assert_eq!(share.denominator(), 1);
        assert_eq!(share.currency(), Currency::Ars);
    }

    #[test]
    fn mi_parte_divide_15000_entre_3() {
        let service = Service::new("Netflix", "15000,00", 3, None).unwrap();
        let share = service.my_share().unwrap();

        assert_eq!(share.numerator_minor(), 500_000);
        assert_eq!(share.denominator(), 1);
    }

    #[test]
    fn mi_parte_conserva_fraccion_exacta() {
        let service = Service::new("Luz", "10000,00", 3, None).unwrap();
        let share = service.my_share().unwrap();

        assert_eq!(share.numerator_minor(), 1_000_000);
        assert_eq!(share.denominator(), 3);
    }

    #[test]
    fn mi_parte_conserva_la_moneda_seleccionada() {
        let service =
            Service::new_with_currency("Netflix", "9,99", Currency::Usd, 3, None).unwrap();
        let share = service.my_share().unwrap();

        assert_eq!(share.currency(), Currency::Usd);
    }

    #[test]
    fn mi_parte_funciona_para_servicio_congelado() {
        let service =
            Service::new_frozen_with_currency("Luz", "9000,00", Currency::Ars, 3, None).unwrap();
        let share = service.my_share().unwrap();

        assert_eq!(service.status(), ServiceStatus::Frozen);
        assert_eq!(share.numerator_minor(), 300_000);
        assert_eq!(share.denominator(), 1);
    }
}
