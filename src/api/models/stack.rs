use sqlx::FromRow;
use time::OffsetDateTime;

use crate::shared::dto::stack::StackDto;

use super::common::{Delete, Get, GetAll};
use super::STModel;

#[derive(FromRow)]
pub struct Stack {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    pub created_at: OffsetDateTime,
    pub author_id: i32,
}

impl STModel for Stack {
    const TABLE_NAME: &'static str = "stacks";
}

impl Get for Stack {}
impl GetAll for Stack {}
impl Delete for Stack {}

impl From<Stack> for StackDto {
    fn from(val: Stack) -> Self {
        StackDto {
            id: val.id,
            name: val.name,
            description: val.description,
            created_at: val.created_at,
        }
    }
}

impl From<Box<Stack>> for StackDto {
    fn from(val: Box<Stack>) -> Self {
        (*val).into()
    }
}
