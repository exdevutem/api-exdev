pub mod club_members;

pub fn add_member_routes() -> actix_web::Scope {
    actix_web::web::scope("/members")
        .service(club_members::get_club_members)
        .service(club_members::get_single_member)
        .service(club_members::add_club_member)
        .service(club_members::update_club_member)
        .service(club_members::delete_member)
}
