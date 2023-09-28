use sqlx::FromRow;
use time::OffsetDateTime;

use crate::dto::technology::TechnologyDto;

#[derive(FromRow)]
pub struct Technology {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    pub purpose: String,
    pub created_at: OffsetDateTime,
}

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
