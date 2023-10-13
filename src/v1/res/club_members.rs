use actix_web::{delete, get, post, put, web, HttpResponse, Responder};
use serde_json::json;

use crate::{
    v1::schemas::club_member::{ClubMemberResponse, CreateMemberSchema, UpdateMemberSchema},
    v1::{models::club_member::ClubMemberModel, responders::errors::DBError},
    AppState,
};

#[get("")]
async fn get_club_members(data: web::Data<AppState>) -> impl Responder {
    // NOTE: Esto devuelve [] en caso de error.
    let members = ClubMemberModel::get_all(&data.pool)
        .await
        .unwrap_or_default();
    let members = ClubMemberResponse::from_vector(&members);

    HttpResponse::Ok().json(json!({"status": 200, "members": members }))
}

#[get("/{id}")]
async fn get_single_member(
    path: web::Path<uuid::Uuid>,
    data: web::Data<AppState>,
) -> Result<HttpResponse, DBError> {
    let member_id = path.into_inner().to_string();

    let member: ClubMemberResponse = ClubMemberModel::get_one(&member_id, &data.pool)
        .await?
        .into();

    Ok(HttpResponse::Ok().json(json!({"status": 200, "member": member})))
}

#[post("/create")]
async fn add_club_member(
    body: web::Json<CreateMemberSchema>,
    data: web::Data<AppState>,
) -> Result<HttpResponse, DBError> {
    let member_id = uuid::Uuid::new_v4().to_string();

    ClubMemberModel::create(&member_id, body.into_inner(), &data.pool).await?;

    Ok(HttpResponse::Ok().json(json!({
        "status": 200,
        "message": "Se ha agregado correctamente al miembro!"
    })))
}

#[put("/{id}")]
async fn update_club_member(
    path: web::Path<uuid::Uuid>,
    body: web::Json<UpdateMemberSchema>,
    data: web::Data<AppState>,
) -> Result<HttpResponse, DBError> {
    let member_id = path.into_inner().to_string();

    let target_member = ClubMemberModel::get_one(&member_id, &data.pool).await?;

    ClubMemberModel::update(target_member, body.into_inner(), &data.pool).await?;

    Ok(HttpResponse::Ok().json(json!({
        "status": 200,
        "message": "Se ha actualizado la informacion del miembro."
    })))
}

#[delete("/{id}")]
pub async fn delete_member(
    path: web::Path<uuid::Uuid>,
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

    let deleted_result = sqlx::query!(r#"DELETE FROM club_members WHERE uuid = ?"#, member.uuid)
        .execute(&data.pool)
        .await;

    match deleted_result {
        Ok(_) => HttpResponse::Ok().json(json!({
            "status": "200",
            "message": "Se ha eliminado el registro con exito."
        })),
        Err(e) => {
            return HttpResponse::InternalServerError().json(json!({
                "status": 500,
                "message": "Ocurrio algo inesperado...",
                "debug": e.to_string()
            }));
        }
    }
}
