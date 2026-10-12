use chrono::{DateTime, Utc};
use serde::Deserialize;
use serde::Serialize;
use utoipa::ToSchema;
use uuid::Uuid;
use validator::Validate;

use crate::domain::register::model::{UserCreateModel, UserModel};

#[cfg_attr(test, derive(Serialize))]
#[derive(Debug, Deserialize, Validate, ToSchema, Clone)]
pub struct RequestCreateUser {
    #[validate(length(min = 1, max = 64))]
    pub account_name: String,
    #[validate(length(min = 1, max = 64))]
    pub name: String,
    #[validate(email, length(max = 64))]
    pub email: String,
    #[validate(length(min = 8, max = 512))]
    pub password: Option<String>,
}
impl From<RequestCreateUser> for UserCreateModel {
    fn from(value: RequestCreateUser) -> Self {
        UserCreateModel::new(value.account_name, value.name, value.email, value.password)
    }
}
#[cfg(test)]
impl RequestCreateUser {
    pub fn mock_default() -> Self {
        Self {
            account_name: "Kael99".to_string(),
            name: "Kael".to_string(),
            email: "kael@gmail.com".to_string(),
            password: Some("12345678".to_string()),
        }
    }
}

#[cfg_attr(test, derive(Deserialize))]
#[derive(Debug, Serialize, ToSchema)]
pub struct ResponseUser {
    pub id: Uuid,
    pub account_name: String,
    pub name: String,
    pub email: String,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
impl From<UserModel> for ResponseUser {
    fn from(value: UserModel) -> Self {
        Self {
            id: value.id,
            account_name: value.account_name,
            name: value.name,
            email: value.email,
            is_active: value.is_active,
            created_at: value.created_at,
            updated_at: value.updated_at,
        }
    }
}
