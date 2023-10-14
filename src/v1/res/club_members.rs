use actix_web::{delete, get, post, put, web};

use crate::{
    v1::{models::club_member::ClubMemberModel, responders::errors::DBError},
    v1::{
        responders::basic_response::BasicResponse,
        schemas::club_member::{ClubMemberResponse, CreateMemberSchema, UpdateMemberSchema},
    },
    AppState,
};

#[get("")]
async fn get_club_members(
    data: web::Data<AppState>,
) -> Result<BasicResponse<Vec<ClubMemberResponse>>, DBError> {
    // NOTE: Esto devuelve [] en caso de error.
    let members = ClubMemberModel::get_all(&data.pool).await?;

    let members = ClubMemberResponse::from_vector(&members);

    Ok(BasicResponse::new("Lista de miembros", Some(members)))
}

#[get("/{id}")]
async fn get_single_member(
    path: web::Path<uuid::Uuid>,
    data: web::Data<AppState>,
) -> Result<BasicResponse<ClubMemberResponse>, DBError> {
    let member_id = path.into_inner().to_string();

    let member: ClubMemberResponse = ClubMemberModel::get_one(&member_id, &data.pool)
        .await?
        .into();

    Ok(BasicResponse::new(
        "Se ha encontrado el siguiente miembro del club",
        Some(member),
    ))
}

#[post("/create")]
async fn add_club_member(
    body: web::Json<CreateMemberSchema>,
    data: web::Data<AppState>,
) -> Result<BasicResponse<()>, DBError> {
    let member_id = uuid::Uuid::new_v4().to_string();

    ClubMemberModel::create(&member_id, body.into_inner(), &data.pool).await?;

    Ok(BasicResponse::new(
        "Se ha agregado exitosamente un nuevo miembro",
        None,
    ))
}

#[put("/update/{id}")]
async fn update_club_member(
    path: web::Path<uuid::Uuid>,
    body: web::Json<UpdateMemberSchema>,
    data: web::Data<AppState>,
) -> Result<BasicResponse<()>, DBError> {
    let member_id = path.into_inner().to_string();

    let target_member = ClubMemberModel::get_one(&member_id, &data.pool).await?;

    ClubMemberModel::update(target_member, body.into_inner(), &data.pool).await?;

    Ok(BasicResponse::new(
        "Se ha actualizado la informacion del miembro.",
        None,
    ))
}

#[delete("/delete/{id}")]
pub async fn delete_member(
    path: web::Path<uuid::Uuid>,
    data: web::Data<AppState>,
) -> Result<BasicResponse<()>, DBError> {
    let member_id = path.into_inner().to_string();

    let member = ClubMemberModel::get_one(&member_id, &data.pool).await?;

    ClubMemberModel::delete(&member, &data.pool).await?;

    Ok(BasicResponse::new(
        "Se ha eliminado el registro con exito.",
        None,
    ))
}
