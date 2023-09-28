use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TechnologyDto {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    pub purpose: String,
    pub created_at: OffsetDateTime,
}
