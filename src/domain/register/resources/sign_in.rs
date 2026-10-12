use std::sync::Arc;

use argon2::{
    password_hash::{rand_core::OsRng, PasswordHasher, SaltString},
    Argon2,
};

use crate::domain::{
    error::DomainError,
    register::{
        model::{UserCreateModel, UserModel},
        repository::UserRepository,
    },
};

fn hash_password(password: &str) -> Result<String, DomainError> {
    let salt = SaltString::generate(&mut OsRng);
    Argon2::default()
        .hash_password(password.as_bytes(), &salt)
        .map(|hash| hash.to_string())
        .map_err(|err| DomainError::InternalServerError(err.to_string()))
}

pub async fn execute(
    user_repository: Arc<dyn UserRepository>,
    mut user_create_model: UserCreateModel,
) -> Result<UserModel, DomainError> {
    if let Some(password) = user_create_model.password.take() {
        let hash = tokio::task::spawn_blocking(move || hash_password(&password))
            .await
            .map_err(|err| DomainError::InternalServerError(err.to_string()))??;
        user_create_model.password = Some(hash);
    }

    user_repository.insert(&user_create_model).await
}

#[cfg(test)]
mod tests {
    use super::*;

    use async_trait::async_trait;
    use mockall::mock;

    mock! {
        pub FakeUserRepository { }

        #[async_trait]
        impl UserRepository for FakeUserRepository {
            async fn insert(&self, user_create_model: &UserCreateModel) -> Result<UserModel, DomainError>;
        }
    }

    #[tokio::test]
    async fn it_should_return_user_created_with_hashed_password() {
        let mut user_repository = MockFakeUserRepository::new();

        user_repository
            .expect_insert()
            .withf(|model| {
                model
                    .password
                    .as_deref()
                    .is_some_and(|password| password.starts_with("$argon2"))
            })
            .return_once(|_| Ok(UserModel::mock_default()));

        let result = execute(Arc::new(user_repository), UserCreateModel::mock_default()).await;

        match result {
            Ok(_) => {}
            Err(err) => unreachable!("{err}"),
        }
    }

    #[tokio::test]
    async fn it_should_return_error_when_user_already_exists() {
        let mut user_repository = MockFakeUserRepository::new();

        user_repository
            .expect_insert()
            .return_once(|_| Err(DomainError::BadRequest("user already exists".to_string())));

        let result = execute(Arc::new(user_repository), UserCreateModel::mock_default()).await;

        assert!(matches!(result, Err(DomainError::BadRequest(_))));
    }
}
