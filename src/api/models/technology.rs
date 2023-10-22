use sqlx::FromRow;
use time::OffsetDateTime;

use crate::shared::dto::technology::TechnologyDto;

use super::common::{Delete, Get, GetAll};
use super::STModel;

#[derive(FromRow)]
pub struct Technology {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    pub purpose: String,
    pub created_at: OffsetDateTime,
}

impl STModel for Technology {
    const TABLE_NAME: &'static str = "technologies";
}

impl Get for Technology {}
impl GetAll for Technology {}
impl Delete for Technology {}

impl From<Technology> for TechnologyDto {
    fn from(val: Technology) -> Self {
        TechnologyDto {
            id: val.id,
            name: val.name,
            description: val.description,
            purpose: val.purpose,
            created_at: val.created_at,
        }
    }
}

impl From<Box<Technology>> for TechnologyDto {
    fn from(val: Box<Technology>) -> Self {
        (*val).into()
    }
}
