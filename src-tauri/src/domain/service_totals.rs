use crate::domain::error::DomainError;
use crate::domain::money::{Currency, ExactMoney, Money};
use crate::domain::service::{Service, ServiceStatus};

#[derive(Debug, Clone)]
pub struct CurrencyServiceTotals {
    total_cost: Money,
    user_share: ExactMoney,
}

impl CurrencyServiceTotals {
    pub fn total_cost(&self) -> Money {
        self.total_cost
    }

    pub fn user_share(&self) -> &ExactMoney {
        &self.user_share
    }
}

#[derive(Debug, Clone)]
pub struct ServiceTotals {
    ars: CurrencyServiceTotals,
    usd: CurrencyServiceTotals,
}

impl ServiceTotals {
    pub fn ars(&self) -> &CurrencyServiceTotals {
        &self.ars
    }

    pub fn usd(&self) -> &CurrencyServiceTotals {
        &self.usd
    }
}

pub fn calculate_service_totals(services: &[Service]) -> Result<ServiceTotals, DomainError> {
    let mut ars = CurrencyServiceTotals {
        total_cost: Money::zero(Currency::Ars),
        user_share: ExactMoney::from_money(Money::zero(Currency::Ars)),
    };
    let mut usd = CurrencyServiceTotals {
        total_cost: Money::zero(Currency::Usd),
        user_share: ExactMoney::from_money(Money::zero(Currency::Usd)),
    };

    for service in services
        .iter()
        .filter(|service| service.status() == ServiceStatus::Active)
    {
        match service.currency() {
            Currency::Ars => {
                ars.total_cost = ars.total_cost.checked_add(&service.total_cost())?;
                ars.user_share = ars.user_share.checked_add(&service.my_share()?)?;
            }
            Currency::Usd => {
                usd.total_cost = usd.total_cost.checked_add(&service.total_cost())?;
                usd.user_share = usd.user_share.checked_add(&service.my_share()?)?;
            }
        }
    }

    Ok(ServiceTotals { ars, usd })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sin_servicios_devuelve_totales_en_cero() {
        let totals = calculate_service_totals(&[]).unwrap();

        assert_eq!(totals.ars().total_cost().minor_units(), 0);
        assert_eq!(totals.ars().total_cost().currency(), Currency::Ars);
        assert_eq!(totals.ars().user_share().numerator_minor(), 0);
        assert_eq!(totals.ars().user_share().denominator(), 1);
        assert_eq!(totals.ars().user_share().currency(), Currency::Ars);

        assert_eq!(totals.usd().total_cost().minor_units(), 0);
        assert_eq!(totals.usd().total_cost().currency(), Currency::Usd);
        assert_eq!(totals.usd().user_share().numerator_minor(), 0);
        assert_eq!(totals.usd().user_share().denominator(), 1);
        assert_eq!(totals.usd().user_share().currency(), Currency::Usd);
    }

    #[test]
    fn sin_servicios_activos_devuelve_totales_en_cero() {
        let frozen =
            Service::new_frozen_with_currency("Luz", "10000,00", Currency::Ars, 4, None).unwrap();
        let totals = calculate_service_totals(std::slice::from_ref(&frozen)).unwrap();

        assert_eq!(totals.ars().total_cost().minor_units(), 0);
        assert_eq!(totals.ars().user_share().numerator_minor(), 0);
        assert_eq!(totals.ars().user_share().denominator(), 1);
        assert_eq!(totals.usd().total_cost().minor_units(), 0);
        assert_eq!(totals.usd().user_share().numerator_minor(), 0);
    }

    #[test]
    fn totales_de_referencia_en_ars() {
        let service_a = Service::new("A", "10000,00", 4, None).unwrap();
        let service_b = Service::new("B", "6000,00", 2, None).unwrap();
        let totals = calculate_service_totals(&[service_a, service_b]).unwrap();

        assert_eq!(totals.ars().total_cost().minor_units(), 1_600_000);
        assert_eq!(totals.ars().user_share().numerator_minor(), 550_000);
        assert_eq!(totals.ars().user_share().denominator(), 1);
        assert_eq!(totals.usd().total_cost().minor_units(), 0);
        assert_eq!(totals.usd().user_share().numerator_minor(), 0);
    }

    #[test]
    fn totales_separados_por_moneda() {
        let ars = Service::new("Luz", "10000,00", 4, None).unwrap();
        let usd = Service::new_with_currency("Netflix", "20,00", Currency::Usd, 2, None).unwrap();
        let totals = calculate_service_totals(&[ars, usd]).unwrap();

        assert_eq!(totals.ars().total_cost().minor_units(), 1_000_000);
        assert_eq!(totals.ars().total_cost().currency(), Currency::Ars);
        assert_eq!(totals.ars().user_share().numerator_minor(), 250_000);
        assert_eq!(totals.ars().user_share().denominator(), 1);
        assert_eq!(totals.ars().user_share().currency(), Currency::Ars);

        assert_eq!(totals.usd().total_cost().minor_units(), 2_000);
        assert_eq!(totals.usd().total_cost().currency(), Currency::Usd);
        assert_eq!(totals.usd().user_share().numerator_minor(), 1_000);
        assert_eq!(totals.usd().user_share().denominator(), 1);
        assert_eq!(totals.usd().user_share().currency(), Currency::Usd);
    }

    #[test]
    fn excluye_servicios_congelados() {
        let active = Service::new("Spotify", "10000,00", 4, None).unwrap();
        let frozen =
            Service::new_frozen_with_currency("Luz", "5000,00", Currency::Ars, 2, None).unwrap();
        let totals = calculate_service_totals(&[active, frozen]).unwrap();

        assert_eq!(totals.ars().total_cost().minor_units(), 1_000_000);
        assert_eq!(totals.ars().user_share().numerator_minor(), 250_000);
        assert_eq!(totals.ars().user_share().denominator(), 1);
    }

    #[test]
    fn suma_partes_fraccionarias_sin_redondeo() {
        let service_a = Service::new("A", "100,00", 3, None).unwrap();
        let service_b = Service::new("B", "100,00", 3, None).unwrap();
        let totals = calculate_service_totals(&[service_a, service_b]).unwrap();

        assert_eq!(totals.ars().total_cost().minor_units(), 20_000);
        assert_eq!(totals.ars().user_share().numerator_minor(), 20_000);
        assert_eq!(totals.ars().user_share().denominator(), 3);
    }

    #[test]
    fn totales_son_deterministas() {
        let service_a = Service::new("A", "10000,00", 3, None).unwrap();
        let service_b = Service::new("B", "6000,00", 2, None).unwrap();
        let first = calculate_service_totals(&[service_a.clone(), service_b.clone()]).unwrap();
        let second = calculate_service_totals(&[service_a, service_b]).unwrap();

        assert_eq!(
            first.ars().total_cost().minor_units(),
            second.ars().total_cost().minor_units()
        );
        assert_eq!(
            first.ars().user_share().numerator_minor(),
            second.ars().user_share().numerator_minor()
        );
        assert_eq!(
            first.ars().user_share().denominator(),
            second.ars().user_share().denominator()
        );
        assert_eq!(
            first.usd().total_cost().minor_units(),
            second.usd().total_cost().minor_units()
        );
        assert_eq!(
            first.usd().user_share().numerator_minor(),
            second.usd().user_share().numerator_minor()
        );
        assert_eq!(
            first.usd().user_share().denominator(),
            second.usd().user_share().denominator()
        );
    }

    #[test]
    fn propaga_desbordamiento_aritmetico() {
        let maximum = Service::new("Max", "92233720368547758.07", 1, None).unwrap();
        let one_cent = Service::new("Uno", "0,01", 1, None).unwrap();
        let error = calculate_service_totals(&[maximum, one_cent]).unwrap_err();

        assert!(matches!(
            error,
            DomainError::ArithmeticOverflow {
                field: "amount",
                ..
            }
        ));
    }
}
