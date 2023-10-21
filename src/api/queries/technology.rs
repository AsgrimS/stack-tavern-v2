use crate::api::models::technology::Technology;
use crate::shared::dto::technology::TechnologyDto;

pub async fn get_technologies() -> Result<Vec<TechnologyDto>, sqlx::Error> {
    use crate::api::models::common::GetAll;

    let technologies = Technology::get_all().await?;
    Ok(technologies.into_iter().map(|t| t.into()).collect())
}

pub async fn get_technology(id: &i32) -> Result<TechnologyDto, sqlx::Error> {
    use crate::api::models::common::Get;

    let technology = Technology::get(id).await?;
    Ok(technology.into())
}
