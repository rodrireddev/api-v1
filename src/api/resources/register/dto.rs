use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};
use uuid::Uuid;
use validator::Validate;

use crate::{
    api::utils::validator::validate_page_size_max,
    domain::categories::model::{CategoryCreateModel, CategoryModel, CategoryUpdateModel},
};

#[cfg_attr(test, derive(Serialize))]
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
            password: "12345678".to_string(),
        }
    }
}

