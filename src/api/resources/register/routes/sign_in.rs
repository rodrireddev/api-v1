use actix_web::{
    post,
    web::{self, Data},
    HttpResponse,
};

use validator::Validate;

use crate::{
    api::{
        lib::AppState,
        resources::register::dto::{self, ResponseUser},
        utils::response::ApiResponse,
    },
    domain::{error::DomainError, register},
};

#[utoipa::path(
    post,
    operation_id = "sign_in",
    path = "/register",
    tag = "register",
    request_body = RequestCreateUser,
    responses(
         (status = 201, description = "user created",  body = ApiResponseUser),
         (status = 400, description = "Invalid payload or user already exists",  body = ErrorResponse),
    ),
 )]
#[post("/register")]
async fn handler(
    state: Data<AppState>,
    body: web::Json<dto::RequestCreateUser>,
) -> Result<HttpResponse, DomainError> {
    body.validate()?;

    let user =
        register::resources::sign_in::execute(state.user_repository.clone(), body.0.into())
            .await?;

    let response = ApiResponse::<ResponseUser>::new(vec![user.into()], None, None, None);

    Ok(HttpResponse::Created().json(response))
}

#[cfg(test)]
mod tests {
    use actix_web::{http::StatusCode, test};

    use crate::api::{
        resources::register::{dto, routes::init_routes},
        tests::utils::get_app,
    };

    #[actix_web::test]
    async fn it_should_return_user_created() {
        let (_, app) = get_app(init_routes).await;

        let req = test::TestRequest::post()
            .uri("/register")
            .set_json(dto::RequestCreateUser::mock_default())
            .to_request();

        let res = test::call_service(&app, req).await;

        assert_eq!(res.status().as_u16(), StatusCode::CREATED);
    }

    #[actix_web::test]
    async fn it_should_return_bad_request_when_email_is_invalid() {
        let (_, app) = get_app(init_routes).await;

        let mut body = dto::RequestCreateUser::mock_default();
        body.email = "not-an-email".to_string();
        let req = test::TestRequest::post()
            .uri("/register")
            .set_json(body)
            .to_request();

        let res = test::call_service(&app, req).await;

        assert_eq!(res.status().as_u16(), StatusCode::BAD_REQUEST);
    }
}
