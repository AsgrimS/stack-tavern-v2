pub mod common;
pub mod technology;

pub trait STModel {
    ///  The name of the table in the stack tavern database.
    const TABLE_NAME: &'static str;
}
