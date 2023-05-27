// TODO: Eliminar despues de implementar
#![allow(dead_code)]

mod models;

use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use dotenv::dotenv;
use models::club_member::ClubMember;
use serde_json::json;
use sqlx::SqlitePool;

use crate::models::club_member::ClubMemberModel;

#[get("/")]
async fn get_club_members(data: web::Data<AppState>) -> impl Responder {
    let members = sqlx::query_as!(ClubMemberModel, "SELECT * FROM club_members")
        .fetch_all(&data.pool)
        .await
        .unwrap();

    let members = members
        .into_iter()
        .map(|model| -> ClubMember { ClubMember::new(&model) })
        .collect::<Vec<ClubMember>>();

    HttpResponse::Ok().json(json!({"status": 200, "members": members}))
}

struct AppState {
    pool: SqlitePool,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let database_url =
        std::env::var("DATABASE_URL").expect("No se ha seteado la URL de la base de datos.");
    let host = std::env::var("HOST").unwrap_or(String::from("127.0.0.1"));
    let port = std::env::var("PORT")
        .unwrap_or(String::from("8080"))
        .parse::<u16>()
        .expect("No se pudo parsear el puerto");

    let pool = SqlitePool::connect(&database_url)
        .await
        .expect("Could not connect to Database");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(AppState { pool: pool.clone() }))
            .service(get_club_members)
    })
    .bind((host, port))?
    .run()
    .await
}
