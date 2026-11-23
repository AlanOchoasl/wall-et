use rusqlite::Connection;

use crate::application::service_repository::{ServiceId, ServiceRecord, ServiceRepository};
use crate::domain::error::DomainError;
use crate::domain::money::Currency;
use crate::domain::service::Service;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CreateServiceInput {
    pub name: String,
    pub total_cost: String,
    pub currency: Currency,
    pub shared_people_count: u32,
    pub notes: Option<String>,
}

impl Default for CreateServiceInput {
    fn default() -> Self {
        Self {
            name: String::new(),
            total_cost: String::new(),
            currency: Currency::Ars,
            shared_people_count: 1,
            notes: None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UpdateServiceInput {
    pub name: String,
    pub total_cost: String,
    pub shared_people_count: u32,
    pub notes: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DeleteConfirmation {
    Confirmed,
    Cancelled,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DeleteServiceOutcome {
    Deleted,
    Cancelled,
}

pub fn create_service<R>(
    connection: &Connection,
    repository: &R,
    input: CreateServiceInput,
) -> Result<ServiceRecord, DomainError>
where
    R: ServiceRepository + ?Sized,
{
    let service = Service::new_with_currency(
        input.name,
        input.total_cost.as_str(),
        input.currency,
        input.shared_people_count,
        input.notes,
    )?;

    repository.insert(connection, &service)
}

pub fn update_service<R>(
    connection: &Connection,
    repository: &R,
    id: ServiceId,
    input: UpdateServiceInput,
) -> Result<ServiceRecord, DomainError>
where
    R: ServiceRepository + ?Sized,
{
    let current = repository
        .get(connection, id)?
        .ok_or(DomainError::NotFound)?;
    let replacement = current.service().edit(
        input.name,
        input.total_cost.as_str(),
        input.shared_people_count,
        input.notes,
    )?;

    repository.update(connection, id, &replacement)
}

pub fn delete_service<R>(
    connection: &Connection,
    repository: &R,
    id: ServiceId,
    confirmation: DeleteConfirmation,
) -> Result<DeleteServiceOutcome, DomainError>
where
    R: ServiceRepository + ?Sized,
{
    match confirmation {
        DeleteConfirmation::Cancelled => Ok(DeleteServiceOutcome::Cancelled),
        DeleteConfirmation::Confirmed => {
            repository.delete(connection, id)?;
            Ok(DeleteServiceOutcome::Deleted)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::infrastructure::database;
    use crate::infrastructure::service_repository::SqliteServiceRepository;

    const REPOSITORY: SqliteServiceRepository = SqliteServiceRepository;

    fn ars_input(name: &str, total_cost: &str, shared_people_count: u32) -> CreateServiceInput {
        CreateServiceInput {
            name: name.to_owned(),
            total_cost: total_cost.to_owned(),
            currency: Currency::Ars,
            shared_people_count,
            notes: None,
        }
    }

    fn update_input(name: &str, total_cost: &str, shared_people_count: u32) -> UpdateServiceInput {
        UpdateServiceInput {
            name: name.to_owned(),
            total_cost: total_cost.to_owned(),
            shared_people_count,
            notes: None,
        }
    }

    #[test]
    fn alta_valida_devuelve_registro_activo_y_parte_calculada() {
        let connection = database::open_in_memory().unwrap();

        let record = create_service(
            &connection,
            &REPOSITORY,
            ars_input("Spotify", "10000,00", 4),
        )
        .unwrap();
        let share = record.service().my_share().unwrap();

        assert_eq!(record.service().name(), "Spotify");
        assert_eq!(
            record.service().status(),
            crate::domain::service::ServiceStatus::Active
        );
        assert_eq!(share.numerator_minor(), 250_000);
        assert_eq!(share.denominator(), 1);
        assert_eq!(share.currency(), Currency::Ars);
    }

    #[test]
    fn alta_usa_ars_por_defecto() {
        let connection = database::open_in_memory().unwrap();
        let input = CreateServiceInput {
            name: "Luz".to_owned(),
            total_cost: "100,00".to_owned(),
            shared_people_count: 1,
            ..Default::default()
        };

        let record = create_service(&connection, &REPOSITORY, input).unwrap();

        assert_eq!(record.service().currency(), Currency::Ars);
    }

    #[test]
    fn alta_en_usd_no_convierte() {
        let connection = database::open_in_memory().unwrap();
        let input = CreateServiceInput {
            name: "Netflix".to_owned(),
            total_cost: "15.99".to_owned(),
            currency: Currency::Usd,
            shared_people_count: 2,
            notes: None,
        };

        let record = create_service(&connection, &REPOSITORY, input).unwrap();

        assert_eq!(record.service().currency(), Currency::Usd);
        assert_eq!(record.service().total_cost().minor_units(), 1_599);
    }

    #[test]
    fn alta_invalida_no_persiste() {
        let connection = database::open_in_memory().unwrap();
        let input = ars_input("   ", "100,00", 1);

        let error = create_service(&connection, &REPOSITORY, input).unwrap_err();

        assert!(matches!(
            error,
            DomainError::Validation { field: "name", .. }
        ));
        assert!(REPOSITORY.list_active(&connection).unwrap().is_empty());
    }

    #[test]
    fn edicion_recalcula_la_parte() {
        let connection = database::open_in_memory().unwrap();
        let created = create_service(
            &connection,
            &REPOSITORY,
            ars_input("Spotify", "10000,00", 4),
        )
        .unwrap();

        let updated = update_service(
            &connection,
            &REPOSITORY,
            created.id(),
            update_input("Spotify", "12000,00", 4),
        )
        .unwrap();
        let share = updated.service().my_share().unwrap();

        assert_eq!(updated.service().total_cost().minor_units(), 1_200_000);
        assert_eq!(share.numerator_minor(), 300_000);
        assert_eq!(share.denominator(), 1);
    }

    #[test]
    fn edicion_invalida_no_modifica_el_servicio() {
        let connection = database::open_in_memory().unwrap();
        let created = create_service(
            &connection,
            &REPOSITORY,
            ars_input("Spotify", "10000,00", 4),
        )
        .unwrap();

        let error = update_service(
            &connection,
            &REPOSITORY,
            created.id(),
            update_input("", "12000,00", 4),
        )
        .unwrap_err();

        assert!(matches!(
            error,
            DomainError::Validation { field: "name", .. }
        ));
        let loaded = REPOSITORY
            .get(&connection, created.id())
            .unwrap()
            .expect("el servicio original debe permanecer");
        assert_eq!(loaded.service().name(), "Spotify");
        assert_eq!(loaded.service().total_cost().minor_units(), 1_000_000);
        assert_eq!(loaded.service().shared_people_count().get(), 4);
    }

    #[test]
    fn eliminacion_confirmada_quita_el_servicio() {
        let connection = database::open_in_memory().unwrap();
        let created =
            create_service(&connection, &REPOSITORY, ars_input("Luz", "100,00", 1)).unwrap();

        let outcome = delete_service(
            &connection,
            &REPOSITORY,
            created.id(),
            DeleteConfirmation::Confirmed,
        )
        .unwrap();

        assert_eq!(outcome, DeleteServiceOutcome::Deleted);
        assert!(REPOSITORY.get(&connection, created.id()).unwrap().is_none());
        assert!(REPOSITORY.list_active(&connection).unwrap().is_empty());
    }

    #[test]
    fn eliminacion_cancelada_no_modifica_el_servicio() {
        let connection = database::open_in_memory().unwrap();
        let created =
            create_service(&connection, &REPOSITORY, ars_input("Luz", "100,00", 1)).unwrap();

        let outcome = delete_service(
            &connection,
            &REPOSITORY,
            created.id(),
            DeleteConfirmation::Cancelled,
        )
        .unwrap();

        assert_eq!(outcome, DeleteServiceOutcome::Cancelled);
        let loaded = REPOSITORY
            .get(&connection, created.id())
            .unwrap()
            .expect("el servicio no debe eliminarse");
        assert_eq!(loaded.service().name(), "Luz");
        assert_eq!(REPOSITORY.list_active(&connection).unwrap().len(), 1);
    }
}
