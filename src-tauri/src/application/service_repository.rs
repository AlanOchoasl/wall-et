use std::fmt;

use chrono::{DateTime, Utc};
use rusqlite::Connection;
use uuid::Uuid;

use crate::domain::error::DomainError;
use crate::domain::service::{Service, ServiceStatus};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ServiceId(Uuid);

impl ServiceId {
    pub(crate) fn new() -> Self {
        Self(Uuid::new_v4())
    }

    pub(crate) fn parse(value: &str) -> Option<Self> {
        Uuid::parse_str(value).ok().map(Self)
    }
}

impl fmt::Display for ServiceId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

#[derive(Debug, Clone)]
pub struct ServiceRecord {
    pub(crate) id: ServiceId,
    pub(crate) service: Service,
    pub(crate) created_at: DateTime<Utc>,
    pub(crate) updated_at: DateTime<Utc>,
}

impl ServiceRecord {
    pub fn id(&self) -> ServiceId {
        self.id
    }

    pub fn service(&self) -> &Service {
        &self.service
    }

    pub fn created_at(&self) -> DateTime<Utc> {
        self.created_at
    }

    pub fn updated_at(&self) -> DateTime<Utc> {
        self.updated_at
    }
}

pub trait ServiceRepository {
    fn insert(
        &self,
        connection: &Connection,
        service: &Service,
    ) -> Result<ServiceRecord, DomainError>;

    fn get(
        &self,
        connection: &Connection,
        id: ServiceId,
    ) -> Result<Option<ServiceRecord>, DomainError>;

    fn update(
        &self,
        connection: &Connection,
        id: ServiceId,
        service: &Service,
    ) -> Result<ServiceRecord, DomainError>;

    fn delete(&self, connection: &Connection, id: ServiceId) -> Result<(), DomainError>;

    fn set_status(
        &self,
        connection: &Connection,
        id: ServiceId,
        status: ServiceStatus,
    ) -> Result<ServiceRecord, DomainError>;

    fn list_active(&self, connection: &Connection) -> Result<Vec<ServiceRecord>, DomainError>;

    fn list_frozen(&self, connection: &Connection) -> Result<Vec<ServiceRecord>, DomainError>;
}
