use std::sync::Arc;

use async_trait::async_trait;
use deadpool_postgres::Pool;

use tokio_postgres::{types::ToSql, Row};
use uuid::Uuid;

use crate::domain::{
    categories::{
        model::{CategoryCreateModel, CategoryModel, CategoryUpdateModel},
        repository::CategoryRepository,
    },
    error::DomainError,
};

const QUERY_INSERT_USER: &str = "
    insert into category
        (id, name, description)
    values
        ($1,$2,$3)
    returning
        id as category_id,
        name as category_name,
        description as category_description,
        is_active as category_is_active,
        created_at as category_created_at,
        updated_at as category_updated_at;";