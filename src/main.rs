// TODO: Eliminar despues de implementar
#![allow(dead_code)]

mod models;

use actix_web::{web, App, HttpServer};
use dotenv::dotenv;
use sqlx::SqlitePool;

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

    HttpServer::new(move || App::new().app_data(web::Data::new(AppState { pool: pool.clone() })))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
