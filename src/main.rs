//! API centralizada y multipropósito del ExDev.
//!
//! Esta API fue diseñada con el propósito de ser una única fuente central de
//! todos los procesos que ocurren en el club.
//!
//! Hay algunas decisiones de 'filosofía' que se tomaron para llegar a este diseño;
//! partiendo desde el lenguaje elegido. Se eligió Rust como lenguaje de desarrollo de esta
//! API debido a su robustez a lo largo del tiempo. En un principio, si esta API es bien
//! diseñada, no será necesario tocarla en mucho tiempo.
//!
//! El mayor problema que enfrenta esto es que la rotación de personas del club es muy alta
//! por lo que la documentación **tiene** que estar a la par. Esta es otra buena razón para
//! ocupar Rust. Si estás leyendo esto para aprender a usar la API, es porque tomamos una buena
//! decisión!
//!
//! Créditos:
//! - Rafael Morales V. - Inicié el proyecto en 2023!

use actix_web::{web, App, HttpServer};
use dotenv::dotenv;
use prefixed_api_key::PrefixedApiKeyController;
use sqlx::SqlitePool;

pub mod v1;

/// Estructura del estado de la aplicación.
///
/// Existen solo dos valores para esta estructura en este momento; uno es una conexión con la base
/// de datos, y el otro es un controlador para las llaves de la API.
///
/// Estos valores son compartidos por toda la API via Actix.
pub struct AppState {
    /// Contiene la conexión con la Base de datos.
    pool: SqlitePool,
    /// Maneja el uso de las llaves de autorización de la aplicación.
    pak_controller: PrefixedApiKeyController<rand::rngs::OsRng, sha2::Sha256>,
}

/// Función principal de la API
///
/// Esta función hace un par de verificaciones iniciales antes de iniciar.
/// 1. Verifica los valores de ambiente, los cuales puedes revisar en el archivo
/// `.env.example`
/// 2. Intenta conectarse con una base de datos SQLite3 siguiendo los valores del archivo de
///    ambiente.
/// 3. Inicia un controlador de PrefixedApiKey, que maneja las llaves de la API.
/// 4. Configura una instancia del servidor según las rutas definidas en cada módulo de versión. A
///    fecha de esta documentación, solo existe el módulo `v1`.
#[actix_web::main]
pub async fn main() -> std::io::Result<()> {
    // Se traen los valores del archivo `.env` para trabajarlos como valores de configuración de
    // la API.
    dotenv().ok();

    // Se traen las configuraciones de la API desde los valores de las variables de ambiente.
    let database_url =
        std::env::var("DATABASE_URL").expect("No se ha seteado la URL de la base de datos.");
    let host = std::env::var("HOST").unwrap_or(String::from("127.0.0.1"));
    let port = std::env::var("PORT")
        .unwrap_or(String::from("8080"))
        .parse::<u16>()
        .expect("No se pudo parsear el puerto");

    // Se instancia una conexión a la base de datos a partir de la configuración del archivo
    // ambiente.
    let pool = SqlitePool::connect(&database_url)
        .await
        .expect("Could not connect to Database");

    // Se crea un controlador de PrefixedApiKey para la API.
    let pak_controller = PrefixedApiKeyController::configure()
        .prefix("ExDevUtem".to_owned())
        .seam_defaults()
        .finalize()
        .expect("Could not create Prefixed Api Key Controller");

    // Se instancia el servidor de Actix.
    let server = HttpServer::new(move || {
        App::new()
            // Este servidor se instancia con los datos sacados anteriormente dentro de una
            // estructura AppState.
            .app_data(web::Data::new(AppState {
                pool: pool.clone(),
                pak_controller: pak_controller.clone(),
            }))
            // Se agregan todas las rutas del módulo `v1`.
            .service(v1::routes())
    })
    .bind((host.clone(), port))?
    .run();

    println!("Escuchando en {host}:{port}");

    server.await
}
