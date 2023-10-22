use crate::api::models::technology::Technology;

pub async fn delete_technology(id: &i32) -> Result<u64, sqlx::Error> {
    use crate::api::models::common::Delete;

    Technology::delete(id).await
}
