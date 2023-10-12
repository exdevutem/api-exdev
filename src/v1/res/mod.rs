pub mod auth;
pub mod club_members;

pub fn add_member_routes() -> actix_web::Scope {
    use club_members as cm;
    actix_web::web::scope("/members")
        .service(cm::get_club_members)
        .service(cm::get_single_member)
        .service(cm::add_club_member)
        .service(cm::update_club_member)
        .service(cm::delete_member)
}

pub fn add_auth_routes() -> actix_web::Scope {
    use auth as a;
    actix_web::web::scope("/auth")
        .service(a::register)
        .service(a::regenerate)
}
