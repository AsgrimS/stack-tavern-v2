use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UserInfoDto {
    pub username: String,
    pub user_id: String,
}
