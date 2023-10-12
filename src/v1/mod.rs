use actix_web::web;

pub mod models;
pub mod res;
pub mod schemas;

pub fn routes() -> actix_web::Scope {
    web::scope("/v1")
        .service(res::add_member_routes())
        .service(res::add_auth_routes())
}
