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

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let pool = SqlitePool::connect(&database_url)
        .await
        .expect("Could not connect to Database");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(AppState { pool: pool.clone() }))
            .service(get_club_members)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
