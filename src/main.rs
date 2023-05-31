// TODO: Eliminar despues de implementar
#![allow(dead_code)]

mod models;
mod schemas;

use actix_web::{get, post, put, web, App, HttpResponse, HttpServer, Responder};
use dotenv::dotenv;
use schemas::club_member::UpdateMemberSchema;
use serde_json::json;
use sqlx::SqlitePool;

use crate::{
    models::club_member::ClubMemberModel,
    schemas::club_member::{ClubMemberResponse, CreateMemberSchema},
};

#[get("/")]
async fn get_club_members(data: web::Data<AppState>) -> impl Responder {
    let members = sqlx::query_as!(ClubMemberModel, "SELECT * FROM club_members")
        .fetch_all(&data.pool)
        .await
        .unwrap();

    let members = members
        .into_iter()
        .map(|model| -> ClubMemberResponse { ClubMemberResponse::new(&model) })
        .collect::<Vec<ClubMemberResponse>>();

    HttpResponse::Ok().json(json!({"status": 200, "members": members}))
}

#[post("/")]
async fn add_club_member(
    body: web::Json<CreateMemberSchema>,
    data: web::Data<AppState>,
) -> impl Responder {
    let member_id = uuid::Uuid::new_v4().to_string();

    let query_result = sqlx::query(
        r#"
    INSERT INTO club_members (uuid, name, birthday, email, github)
    VALUES (?, ?, ?, ?, ?)"#,
    )
    .bind(member_id.clone())
    .bind(body.name.to_string())
    .bind(body.birthday.to_owned())
    .bind(body.email.to_owned())
    .bind(body.github.to_owned())
    .execute(&data.pool)
    .await
    .map_err(|err: sqlx::Error| err.to_string());

    if let Err(err) = query_result {
        return HttpResponse::InternalServerError().json(
            // WARN: Esto pasa el mensaje de error directo. Deberia haber un filtro a futuro que lo
            // saque si no estamos en ambiente de desarrollo.
            json!({
                "status": 500,
                "message": "Ha ocurrido un error interno",
                "debug" : err
            }),
        );
    }

    HttpResponse::Ok().json(json!({
        "status": 200,
        "message": "Se ha agregado correctamente al miembro!"
    }))
}

#[put("/{id}")]
async fn update_club_member(
    path: web::Path<uuid::Uuid>,
    body: web::Json<UpdateMemberSchema>,
    data: web::Data<AppState>,
) -> impl Responder {
    let member_id = path.into_inner().to_string();

    let query_result = sqlx::query_as!(
        ClubMemberModel,
        r#"SELECT * FROM club_members WHERE uuid = ?"#,
        member_id
    )
    .fetch_one(&data.pool)
    .await;

    let member = match query_result {
        Ok(member) => member,
        Err(sqlx::Error::RowNotFound) => {
            return HttpResponse::NotFound().json(json!({
                "status": 404,
                "message": "No se encontro el miembro referido."
            }))
        }
        Err(e) => {
            // WARN: Esto pasa el mensaje de error directo. Deberia haber un filtro a futuro que lo
            // saque si no estamos en ambiente de desarrollo.
            return HttpResponse::InternalServerError().json(json!({
                "status": 500,
                "message": "Ocurrio algo inesperado...",
                "debug": e.to_string()
            }));
        }
    };

    // No se si hay una forma mas bonita de hacer esto...
    // Segun estuve leyendo, solo se puede hacer 'bonito' si no se usa la version en macro de
    // `sqlx::query`, una lastima!
    let name = body.name.to_owned().unwrap_or_else(|| member.name);
    // TODO: Verificar que sea una fecha.
    let birthday = body.birthday.to_owned().or(member.birthday);
    let email = body.email.to_owned().or(member.email);
    let github = body.github.to_owned().or(member.github);
    let state = body.state.to_owned().unwrap_or_else(|| member.state);

    let update_member = sqlx::query!(
        r#"
    UPDATE club_members
    SET name = ?, birthday = ?, email = ?, github = ?, state = ? WHERE uuid = ?"#,
        name,
        birthday,
        email,
        github,
        state,
        member_id
    )
    .execute(&data.pool)
    .await;

    match update_member {
        Ok(_) => HttpResponse::Ok().json(json!({
            "status": 200,
            "message": "Se ha actualizado la informacion del miembro."
        })),
        Err(e) => {
            // WARN: Esto pasa el mensaje de error directo. Deberia haber un filtro a futuro que lo
            // saque si no estamos en ambiente de desarrollo.
            return HttpResponse::InternalServerError().json(json!({
                "status": 500,
                "message": "Ocurrio algo inesperado...",
                "debug": e.to_string()
            }));
        }
    }
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
            .service(add_club_member)
            .service(update_club_member)
    })
    .bind((host, port))?
    .run()
    .await
}
