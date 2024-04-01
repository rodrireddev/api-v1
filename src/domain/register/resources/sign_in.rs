use actix_web::{
    post,
    web::{self, Data},
    HttpResponse,
};

use validator::Validate;

use crate::{
    api::{
        lib::AppState,
        resources::categories::dto::{self, ResponseCategory},
        utils::response::ApiResponse,
    },
    domain::{categories, error::DomainError},
};

#[utoipa::path(
    post,
    operation_id = "create_categories",
    path = "/categories",
    tag = "categories",
    request_body = RequestCreateCategory,
    responses(
         (status = 201, description = "category created",  body = ApiResponseCategory),
         (status = 400, description = "Invalid payload",  body = ErrorResponse),
    ),
 )]
#[post("/categories")]
async fn handler(
    state: Data<AppState>,
    body: web::Json<dto::RequestCreateCategory>,
) -> Result<HttpResponse, DomainError> {
    body.validate()?;

    let category =
        categories::resources::create::execute(state.category_repository.clone(), body.0.into())
            .await?;

    let response = ApiResponse::<ResponseCategory>::new(vec![category.into()], None, None, None);

    Ok(HttpResponse::Created().json(response))
}