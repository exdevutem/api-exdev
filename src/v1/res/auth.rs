//! Handlers relacionados a la creación de Applicaciones clientes de la API.
//!
//! Solo existen dos rutas en este momento: una que crea estas aplicaciones y otra que regenera la
//! llave en caso de necesitarse.

use actix_web::{post, put, web, HttpResponse, Responder};
use serde_json::json;

use crate::{
    v1::{
        models::auth::AppModel,
        schemas::auth::{CreateAppSchema, UpdateAppSchema},
    },
    AppState,
};

/// Crea una nueva aplicación
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
    .bind(&app_id)
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

    HttpResponse::Created().json(json!({
        "status": 201,
        "api_key": pak.to_string(),
        "app_id": app_id,
    }))
}

/// Regenera la llave de la aplicación.
#[put("/regenerate/{id}")]
async fn regenerate(
    body: web::Json<UpdateAppSchema>,
    data: web::Data<AppState>,
    path: web::Path<uuid::Uuid>,
) -> impl Responder {
    let app_id = path.into_inner().to_string();

    let query_result = sqlx::query_as!(AppModel, "SELECT * FROM apps WHERE uuid = ?", app_id)
        .fetch_one(&data.pool)
        .await;

    // Abortamos si no existe la app.
    let app = match query_result {
        Ok(app) => app,
        Err(e) => {
            return HttpResponse::NotFound().json(json!({
                "status": 404,
                "message": "No se encontro la app buscada",
                "debug": e.to_string()
            }));
        }
    };

    let name = body.name.to_owned().unwrap_or(app.name);
    let desc = body.description.to_owned().or(app.description);

    let query_result = sqlx::query!(
        r#"UPDATE apps SET name = ?, description = ? WHERE uuid = ?"#,
        name,
        desc,
        app_id
    )
    .execute(&data.pool)
    .await;

    match query_result {
        Ok(_) => HttpResponse::Ok().json(json!({
            "status": 200,
            "message": "App actualizada correctamente"
        })),
        Err(e) => HttpResponse::InternalServerError().json(json!({
        "status": 500,
         "message": "No se pudo actualizar el registro",
         "debug": e.to_string()
        })),
    }
}
