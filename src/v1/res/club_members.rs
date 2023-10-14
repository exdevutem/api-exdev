//! Handlers para peticiones relacionadas a integrantes del club.
//!
//! Estas funciones siguen un patrón CRUD de API típico con las 5 funciones que esperarías
//! encontrar en cualquier API REST basada en recursos:
//! - Mostrar todo
//! - Mostrar uno
//! - Crear
//! - Actualizar
//! - Eliminar.
//!
//! Actualmente estas funciones son llevadas de forma literal, pero quizás un poco de discusión
//! deba llevarse a cabo a futuro sobre estas.

use actix_web::{delete, get, post, put, web};

use crate::{
    v1::{models::club_member::ClubMemberModel, responders::errors::DBError},
    v1::{
        responders::basic_response::BasicResponse,
        schemas::club_member::{ClubMemberResponse, CreateMemberSchema, UpdateMemberSchema},
    },
    AppState,
};

/// Obtiene una lista de todos los miembros
///
/// Actualmente esta función no hace ningún tipo de filtrado, y solo falla según falle la base de
/// datos por alguna razón.
///
/// Los pasos que sigue son los siguientes:
/// 1. Obtiene un vector con los modelos de todos los integrantes.
/// 2. Convierte ese vector de modelos a un vector de Respuestas (ClubMemberResponse)
/// 3. Genera una respuesta básica con estos datos.
#[get("")]
async fn get_club_members(
    data: web::Data<AppState>,
) -> Result<BasicResponse<Vec<ClubMemberResponse>>, DBError> {
    let members = ClubMemberModel::get_all(&data.pool).await?;

    let members = ClubMemberResponse::from_vector(&members);

    Ok(BasicResponse::new("Lista de miembros", Some(members)))
}

/// Obtiene un único miembro según su UUID
///
/// En caso de no encontrar el miembro, genera un DBError con código 404. Lo que esperarías, vaya.
/// Si lo encuentra, envía una respuesta básica con los datos de ese miembro.
///
/// Lo único entretenido que está pasando es que la transformación hacia un ClubMemberResponse se
/// hace hinteando el tipo de la variable y utilizando la función into().
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

/// Agrega un nuevo integrante al club.
///
/// El cuerpo de esta petición requiere un JSON con un campo "name" requerido. Puedes ver más
/// detalles al respecto revisando CreateMemberSchema para entender qué valores son requeridos,
/// posibles y demás.
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

/// Actualiza los datos de un integrante del club.
///
/// Los pasos son:
/// 1. Obtener los datos del integrante a partir de su UUID.
/// 2. Intentar actualizar estos datos utilizando el cuerpo de la petición.
/// 3. Devuelve una respuesta básica.
#[put("/update/{id}")]
async fn update_club_member(
    path: web::Path<uuid::Uuid>,
    body: web::Json<UpdateMemberSchema>,
    data: web::Data<AppState>,
) -> Result<BasicResponse<()>, DBError> {
    let member_id = path.into_inner().to_string();

    let target_member = ClubMemberModel::get_one(&member_id, &data.pool).await?;

    ClubMemberModel::update(target_member, body.into_inner(), &data.pool).await?;

    // NOTE: Debería esto devolver los datos nuevos del integrante?
    Ok(BasicResponse::new(
        "Se ha actualizado la informacion del miembro.",
        None,
    ))
}

/// Elimina a un integrante de la bdd.
///
/// Actualmente esta eliminación es total. Quita la fila de la Base de datos. Quizás deberíamos
/// considerar hacer un 'soft delete' de alguna forma en vez de eliminarlo directamente.
///
/// En cualquier caso, los pasos son los de siempre:
/// 1. Obtener el integrante a eliminar.
/// 2. Intentar eliminarlo.
/// 3. Devolver una respuesta básica.
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
