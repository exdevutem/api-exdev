use actix_web::{post, put, web, HttpResponse, Responder};
use serde_json::json;

use crate::{v1::schemas::auth::CreateAppSchema, AppState};

#[post("/register")]
async fn register(body: web::Json<CreateAppSchema>, data: web::Data<AppState>) -> impl Responder {
    let app_id = uuid::Uuid::new_v4().to_string();

    let name = body.name.to_owned();
    let description = body.description.to_owned();
    let (pak, hash) = data.pak_controller.clone().generate_key_and_hash();

    let query_result = sqlx::query(
        r#"
        INSERT INTO apps(uuid, name, description, api_token)
        VALUES (?, ?, ?, ?);"#,
    )
    .bind(app_id)
    .bind(name)
    .bind(description)
    .bind(hash)
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
        "status": 201,
        "api_key": pak.to_string(),
    }))
}

#[put("/regenerate")]
async fn regenerate() -> impl Responder {
    HttpResponse::Ok()
}
