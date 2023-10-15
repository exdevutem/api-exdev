//! Módulo de respuestas de la API.
//!
//! Aquí encontrarás el cuerpo y carne de la API, junto con el mapeo de ruta a función dentro de
//! esta.

pub mod auth;
pub mod club_members;
pub mod projects;

/// Agrega las rutas relacionada a la autorización de Apps.
///
/// Las rutas son agregadas bajo el campo de `/auth`, por lo que todas las funciones puedes ser
/// encontradas bajo `/v1/auth`.
pub fn add_auth_routes() -> actix_web::Scope {
    use auth as a;

    actix_web::web::scope("/auth")
        .service(a::register)
        .service(a::regenerate)
}

/// Agrega las rutas relacionada a los integrantes del club.
///
/// Las rutas son agregadas bajo el campo de `/members`, por lo que todas las funciones puedes ser
/// encontradas bajo `/v1/members`.
pub fn add_member_routes() -> actix_web::Scope {
    use club_members as cm;

    actix_web::web::scope("/members")
        .service(cm::get_club_members)
        .service(cm::get_single_member)
        .service(cm::add_club_member)
        .service(cm::update_club_member)
        .service(cm::delete_member)
}

/// Agrega las rutas relacionadas a los proyectos del club.
///
/// Las rutas son agregadas bajo el campo de `/projects`, por lo que todas las funciones pueden ser
/// encontradas bajo `/v1/projects`.
pub fn add_project_routes() -> actix_web::Scope {
    use projects as p;

    actix_web::web::scope("/projects").service(p::get_single_member)
}
