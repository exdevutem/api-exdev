use actix_web::web;

pub mod models;
pub mod res;
pub mod schemas;

pub fn routes() -> actix_web::Scope {
    web::scope("/v1").service(
        web::scope("/members")
            .service(res::club_members::get_club_members)
            .service(res::club_members::get_single_member)
            .service(res::club_members::add_club_member)
            .service(res::club_members::update_club_member),
    )
}
