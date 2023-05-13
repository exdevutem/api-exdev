// TODO: Eliminar despues de implementar
#![allow(dead_code)]

mod club_member;
mod project;

use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use dotenv::dotenv;
use sqlx::SqlitePool;

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let pool = match SqlitePool::connect(&database_url).await {
        Ok(value) => value,
        _ => std::process::exit(1),
    };

    match sqlx::query!("SELECT * FROM memos").fetch_one(&pool).await {
        Ok(value) => println!("{:?}", value.text),
        _ => std::process::exit(1),
    };

    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(echo)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
