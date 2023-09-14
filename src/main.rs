// TODO: Eliminar despues de implementar
#![allow(dead_code)]

mod v1;

use actix_web::{web, App, HttpServer};
use dotenv::dotenv;
use prefixed_api_key::PrefixedApiKeyController;
use sqlx::SqlitePool;

pub struct AppState {
    pool: SqlitePool,
    pak_controller: PrefixedApiKeyController<rand::rngs::OsRng, sha2::Sha256>,
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

    // Create the API Key controller for the app.
    let pak_controller = PrefixedApiKeyController::configure()
        .prefix("ExDevUtem".to_owned())
        .seam_defaults()
        .finalize()
        .expect("Could not create Prefixed Api Key Controller");

    let server = HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(AppState {
                pool: pool.clone(),
                pak_controller: pak_controller.clone(),
            }))
            .service(v1::routes())
    })
    .bind((host.clone(), port))?
    .run();

    println!("Escuchando en {host}:{port}");

    server.await
}
