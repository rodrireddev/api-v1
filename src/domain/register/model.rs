use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct UserCreateModel {
    pub id: Uuid,
    pub account_name: String,
    pub name: String,
    pub email: String;
    pub password: Option<String>,
}

impl UserCreateModel {
    pub fn new(account_name: String, name: String,email: String, password: Option<String>) -> Self {
        Self {
            id: Uuid::new_v4(),
            account_name,
            name,
            email,
            password,
        }
    }
}