use std::num::NonZeroU32;

use crate::domain::error::DomainError;
use crate::domain::money::{Currency, ExactMoney, Money};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ServiceStatus {
    Active,
    Frozen,
}

impl ServiceStatus {
    pub(crate) const fn as_str(self) -> &'static str {
        match self {
            Self::Active => "active",
            Self::Frozen => "frozen",
        }
    }

    pub(crate) fn from_code(value: &str) -> Option<Self> {
        match value {
            "active" => Some(Self::Active),
            "frozen" => Some(Self::Frozen),
            _ => None,
        }
    }
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
        let shared_people_count = parse_people_count(shared_people_count)?;

        Ok(Self {
            name,
            total_cost,
            shared_people_count,
            notes,
            status: ServiceStatus::Active,
        })
    }

    pub(crate) fn from_parts(
        name: impl Into<String>,
        total_cost: Money,
        shared_people_count: u32,
        notes: Option<String>,
        status: ServiceStatus,
    ) -> Result<Self, DomainError> {
        let name = name.into();
        validate_name(&name)?;
        validate_total_cost(total_cost)?;
        let shared_people_count = parse_people_count(shared_people_count)?;

        Ok(Self {
            name,
            total_cost,
            shared_people_count,
            notes,
            status,
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

    pub fn edit(
        &self,
        name: impl Into<String>,
        total_cost: &str,
        shared_people_count: u32,
        notes: Option<String>,
    ) -> Result<Self, DomainError> {
        let total_cost = parse_total_cost(total_cost, self.currency())?;
        Self::from_parts(name, total_cost, shared_people_count, notes, self.status)
    }

    pub fn freeze(&self) -> Self {
        let mut frozen = self.clone();
        frozen.status = ServiceStatus::Frozen;
        frozen
    }

    pub fn reactivate(&self) -> Self {
        let mut active = self.clone();
        active.status = ServiceStatus::Active;
        active
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

    validate_total_cost(total_cost)?;

    Ok(total_cost)
}

fn validate_total_cost(total_cost: Money) -> Result<(), DomainError> {
    if total_cost.minor_units() < 0 {
        return Err(DomainError::validation(
            "totalCost",
            "El costo total no puede ser negativo.",
        ));
    }
    Ok(())
}

fn parse_people_count(shared_people_count: u32) -> Result<NonZeroU32, DomainError> {
    NonZeroU32::new(shared_people_count).ok_or_else(|| {
        DomainError::validation(
            "sharedPeopleCount",
            "La cantidad de personas debe ser mayor o igual a 1.",
        )
    })
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
    fn from_parts_reconstruye_servicio_activo() {
        let service = Service::from_parts(
            "Luz",
            Money::from_minor_units(10_000, Currency::Ars),
            2,
            Some("compartido".to_string()),
            ServiceStatus::Active,
        )
        .unwrap();

        assert_eq!(service.name(), "Luz");
        assert_eq!(service.total_cost().minor_units(), 10_000);
        assert_eq!(service.currency(), Currency::Ars);
        assert_eq!(service.shared_people_count().get(), 2);
        assert_eq!(service.notes(), Some("compartido"));
        assert_eq!(service.status(), ServiceStatus::Active);
    }

    #[test]
    fn from_parts_reconstruye_servicio_congelado() {
        let service = Service::from_parts(
            "Luz",
            Money::from_minor_units(10_000, Currency::Usd),
            3,
            None,
            ServiceStatus::Frozen,
        )
        .unwrap();

        assert_eq!(service.currency(), Currency::Usd);
        assert_eq!(service.status(), ServiceStatus::Frozen);
    }

    #[test]
    fn from_parts_rechaza_nombre_vacio() {
        let error = Service::from_parts(
            "",
            Money::from_minor_units(10_000, Currency::Ars),
            2,
            None,
            ServiceStatus::Active,
        )
        .unwrap_err();

        assert!(matches!(
            error,
            DomainError::Validation { field: "name", .. }
        ));
    }

    #[test]
    fn from_parts_rechaza_cantidad_de_personas_cero() {
        let error = Service::from_parts(
            "Luz",
            Money::from_minor_units(10_000, Currency::Ars),
            0,
            None,
            ServiceStatus::Active,
        )
        .unwrap_err();

        assert!(matches!(
            error,
            DomainError::Validation {
                field: "sharedPeopleCount",
                ..
            }
        ));
    }

    #[test]
    fn from_parts_rechaza_costo_negativo() {
        let error = Service::from_parts(
            "Luz",
            Money::from_minor_units(-1, Currency::Ars),
            2,
            None,
            ServiceStatus::Active,
        )
        .unwrap_err();

        assert!(matches!(
            error,
            DomainError::Validation {
                field: "totalCost",
                ..
            }
        ));
    }

    #[test]
    fn codigos_de_estado_hacen_round_trip() {
        for status in [ServiceStatus::Active, ServiceStatus::Frozen] {
            assert_eq!(ServiceStatus::from_code(status.as_str()), Some(status));
        }
        assert_eq!(ServiceStatus::from_code("paused"), None);
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

    #[test]
    fn edit_preserva_moneda_y_estado_y_recalcula() {
        let service = Service::new_frozen_with_currency(
            "Spotify",
            "10000,00",
            Currency::Usd,
            4,
            Some("antigua".to_string()),
        )
        .unwrap();

        let edited = service
            .edit("Spotify Premium", "12000,00", 3, Some("nueva".to_string()))
            .unwrap();
        let share = edited.my_share().unwrap();

        assert_eq!(edited.name(), "Spotify Premium");
        assert_eq!(edited.total_cost().minor_units(), 1_200_000);
        assert_eq!(edited.currency(), Currency::Usd);
        assert_eq!(edited.shared_people_count().get(), 3);
        assert_eq!(edited.notes(), Some("nueva"));
        assert_eq!(edited.status(), ServiceStatus::Frozen);
        assert_eq!(share.numerator_minor(), 400_000);
        assert_eq!(share.denominator(), 1);
    }

    #[test]
    fn freeze_cambia_estado_y_conserva_datos() {
        let service = Service::new_with_currency(
            "Spotify",
            "10000,00",
            Currency::Usd,
            4,
            Some("familiar".to_string()),
        )
        .unwrap();

        let frozen = service.freeze();

        assert_eq!(frozen.status(), ServiceStatus::Frozen);
        assert_eq!(frozen.name(), "Spotify");
        assert_eq!(frozen.total_cost().minor_units(), 1_000_000);
        assert_eq!(frozen.currency(), Currency::Usd);
        assert_eq!(frozen.shared_people_count().get(), 4);
        assert_eq!(frozen.notes(), Some("familiar"));
        assert_eq!(service.status(), ServiceStatus::Active);
    }

    #[test]
    fn reactivate_cambia_estado_y_conserva_datos() {
        let service = Service::new_frozen_with_currency(
            "Luz",
            "9000,00",
            Currency::Ars,
            3,
            Some("compartida".to_string()),
        )
        .unwrap();

        let active = service.reactivate();

        assert_eq!(active.status(), ServiceStatus::Active);
        assert_eq!(active.name(), "Luz");
        assert_eq!(active.total_cost().minor_units(), 900_000);
        assert_eq!(active.currency(), Currency::Ars);
        assert_eq!(active.shared_people_count().get(), 3);
        assert_eq!(active.notes(), Some("compartida"));
        assert_eq!(service.status(), ServiceStatus::Frozen);
    }
}
