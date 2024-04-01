use actix_web::web;

pub mod sign_in;

pub fn init_routes(config: &mut web::ServiceConfig) {
    config.service(sign_in::handler);
}
