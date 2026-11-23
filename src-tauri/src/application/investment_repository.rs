use std::fmt;

use chrono::{DateTime, Utc};
use rusqlite::Connection;
use uuid::Uuid;

use crate::domain::error::DomainError;
use crate::domain::investment::{Investment, Month};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct InvestmentId(Uuid);

impl InvestmentId {
    pub(crate) fn new() -> Self {
        Self(Uuid::new_v4())
    }

    pub(crate) fn parse(value: &str) -> Option<Self> {
        Uuid::parse_str(value).ok().map(Self)
    }
}

impl fmt::Display for InvestmentId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

#[derive(Debug, Clone)]
pub struct InvestmentRecord {
    pub(crate) id: InvestmentId,
    pub(crate) investment: Investment,
    pub(crate) created_at: DateTime<Utc>,
    pub(crate) updated_at: DateTime<Utc>,
}

impl InvestmentRecord {
    pub fn id(&self) -> InvestmentId {
        self.id
    }

    pub fn investment(&self) -> &Investment {
        &self.investment
    }

    pub fn created_at(&self) -> DateTime<Utc> {
        self.created_at
    }

    pub fn updated_at(&self) -> DateTime<Utc> {
        self.updated_at
    }
}

pub trait InvestmentRepository {
    fn insert(
        &self,
        connection: &Connection,
        investment: &Investment,
    ) -> Result<InvestmentRecord, DomainError>;

    fn get(
        &self,
        connection: &Connection,
        id: InvestmentId,
    ) -> Result<InvestmentRecord, DomainError>;

    fn update(
        &self,
        connection: &Connection,
        id: InvestmentId,
        investment: &Investment,
    ) -> Result<InvestmentRecord, DomainError>;

    fn list_by_month(
        &self,
        connection: &Connection,
        month: Month,
    ) -> Result<Vec<InvestmentRecord>, DomainError>;
}
