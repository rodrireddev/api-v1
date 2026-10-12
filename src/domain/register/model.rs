use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct UserCreateModel {
    pub id: Uuid,
    pub account_name: String,
    pub name: String,
    pub email: String,
    pub password: Option<String>,
}
impl UserCreateModel {
    pub fn new(account_name: String, name: String, email: String, password: Option<String>) -> Self {
        Self {
            id: Uuid::new_v4(),
            account_name,
            name,
            email,
            password,
        }
    }
}
#[cfg(test)]
impl UserCreateModel {
    pub fn mock_default() -> Self {
        Self {
            id: Uuid::new_v4(),
            account_name: "Kael99".to_string(),
            name: "Kael".to_string(),
            email: "kael@gmail.com".to_string(),
            password: Some("12345678".to_string()),
        }
    }
}

#[derive(Debug, Clone)]
pub struct UserModel {
    pub id: Uuid,
    pub account_name: String,
    pub name: String,
    pub email: String,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
#[cfg(test)]
impl UserModel {
    pub fn mock_default() -> Self {
        Self {
            id: Uuid::new_v4(),
            account_name: "Kael99".to_string(),
            name: "Kael".to_string(),
            email: "kael@gmail.com".to_string(),
            is_active: true,
            created_at: DateTime::default(),
            updated_at: DateTime::default(),
        }
    }
}
