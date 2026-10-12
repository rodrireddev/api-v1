use std::sync::Arc;

use async_trait::async_trait;
use deadpool_postgres::Pool;

use tokio_postgres::{error::SqlState, Row};

use crate::domain::{
    error::DomainError,
    register::{
        model::{UserCreateModel, UserModel},
        repository::UserRepository,
    },
};

const QUERY_INSERT_USER: &str = "
    insert into users
        (id, account_name, name, email, password_hash)
    values
        ($1,$2,$3,$4,$5)
    returning
        id as user_id,
        account_name as user_account_name,
        name as user_name,
        email as user_email,
        is_active as user_is_active,
        created_at as user_created_at,
        updated_at as user_updated_at;";

pub struct PgUserRepository {
    pool: Arc<Pool>,
}
impl PgUserRepository {
    pub fn new(pool: Arc<Pool>) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl UserRepository for PgUserRepository {
    async fn insert(&self, user_create_model: &UserCreateModel) -> Result<UserModel, DomainError> {
        let client = self.pool.get().await?;
        let stmt = client.prepare(QUERY_INSERT_USER).await?;
        let result = client
            .query_one(
                &stmt,
                &[
                    &user_create_model.id,
                    &user_create_model.account_name,
                    &user_create_model.name,
                    &user_create_model.email,
                    &user_create_model.password,
                ],
            )
            .await
            .map_err(|err| {
                if err.code() == Some(&SqlState::UNIQUE_VIOLATION) {
                    DomainError::BadRequest("account name or email already exists".to_string())
                } else {
                    err.into()
                }
            })?;

        Ok((&result).into())
    }
}

impl From<&Row> for UserModel {
    fn from(row: &Row) -> Self {
        Self {
            id: row.get("user_id"),
            account_name: row.get("user_account_name"),
            name: row.get("user_name"),
            email: row.get("user_email"),
            is_active: row.get("user_is_active"),
            created_at: row.get("user_created_at"),
            updated_at: row.get("user_updated_at"),
        }
    }
}
