use crate::domain::error::DomainError;
use crate::domain::money::{Currency, ExactMoney};
use crate::domain::service::Service;
use crate::domain::service_totals::calculate_service_totals;

pub fn calculate_monthly_expenses(
    services: &[Service],
    currency: Currency,
) -> Result<ExactMoney, DomainError> {
    let totals = calculate_service_totals(services)?;

    Ok(match currency {
        Currency::Ars => totals.ars().user_share().clone(),
        Currency::Usd => totals.usd().user_share().clone(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_exact(
        amount: &ExactMoney,
        numerator_minor: i128,
        denominator: u128,
        currency: Currency,
    ) {
        assert_eq!(amount.numerator_minor(), numerator_minor);
        assert_eq!(amount.denominator(), denominator);
        assert_eq!(amount.currency(), currency);
    }

    #[test]
    fn suma_partes_personales() {
        let service_a = Service::new("A", "10000,00", 4, None).unwrap();
        let service_b = Service::new("B", "6000,00", 2, None).unwrap();

        let expenses = calculate_monthly_expenses(&[service_a, service_b], Currency::Ars).unwrap();

        assert_exact(&expenses, 550_000, 1, Currency::Ars);
    }

    #[test]
    fn sin_servicios_activos_devuelve_cero_exacto() {
        let frozen =
            Service::new_frozen_with_currency("Luz", "10000,00", Currency::Ars, 4, None).unwrap();

        let expenses =
            calculate_monthly_expenses(std::slice::from_ref(&frozen), Currency::Ars).unwrap();

        assert_exact(&expenses, 0, 1, Currency::Ars);
    }

    #[test]
    fn excluye_servicios_congelados() {
        let active = Service::new("Activo", "10000,00", 4, None).unwrap();
        let frozen =
            Service::new_frozen_with_currency("Congelado", "5000,00", Currency::Ars, 1, None)
                .unwrap();

        let expenses = calculate_monthly_expenses(&[active, frozen], Currency::Ars).unwrap();

        assert_exact(&expenses, 250_000, 1, Currency::Ars);
    }

    #[test]
    fn conserva_fracciones_exactas() {
        let service_a = Service::new("A", "100,00", 3, None).unwrap();
        let service_b = Service::new("B", "100,00", 3, None).unwrap();

        let expenses = calculate_monthly_expenses(&[service_a, service_b], Currency::Ars).unwrap();

        assert_exact(&expenses, 20_000, 3, Currency::Ars);
    }

    #[test]
    fn separa_ars_y_usd_sin_conversion() {
        let ars = Service::new("Luz", "10000,00", 4, None).unwrap();
        let usd = Service::new_with_currency("Netflix", "20,00", Currency::Usd, 2, None).unwrap();
        let services = [ars, usd];

        let ars_expenses = calculate_monthly_expenses(&services, Currency::Ars).unwrap();
        let usd_expenses = calculate_monthly_expenses(&services, Currency::Usd).unwrap();

        assert_exact(&ars_expenses, 250_000, 1, Currency::Ars);
        assert_exact(&usd_expenses, 1_000, 1, Currency::Usd);
    }

    #[test]
    fn sin_servicios_en_la_moneda_solicitada_devuelve_cero_en_esa_moneda() {
        let ars = Service::new("Luz", "10000,00", 4, None).unwrap();

        let usd_expenses =
            calculate_monthly_expenses(std::slice::from_ref(&ars), Currency::Usd).unwrap();

        assert_exact(&usd_expenses, 0, 1, Currency::Usd);
    }

    #[test]
    fn propaga_desbordamiento_aritmetico() {
        let maximum = Service::new("Max", "92233720368547758.07", 1, None).unwrap();
        let one_cent = Service::new("Uno", "0,01", 1, None).unwrap();

        let error = calculate_monthly_expenses(&[maximum, one_cent], Currency::Ars).unwrap_err();

        assert!(matches!(
            error,
            DomainError::ArithmeticOverflow {
                field: "amount",
                ..
            }
        ));
    }
}
