use async_trait::async_trait;

use crate::domain::error::DomainError;

use super::model::{UserCreateModel, UserModel};

#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn insert(&self, user_create_model: &UserCreateModel) -> Result<UserModel, DomainError>;
}
