use serde::Deserialize;
use utoipa::ToSchema;
use validator::Validate;

use crate::domain::register::model::UserCreateModel;

#[cfg_attr(test, derive(serde::Serialize))]
#[derive(Debug, Deserialize, Validate, ToSchema, Clone)]
pub struct RequestCreateUser {
    #[validate(length(max = 64))]
    pub account_name: String,
    #[validate(length(max = 64))]
    pub name: String,
    #[validate(length(max = 64))]
    pub email: String,
    #[validate(length(max = 512))]
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
            email: "Kael@gmail.com".to_string(),
            password: Some("12345678".to_string()),
        }
    }
}

