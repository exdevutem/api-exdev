use actix_web::{delete, get, post, put, web};

use crate::{
    v1::{
        models::project::ProjectModel,
        responders::{basic_response::BasicResponse, errors::DBError},
    },
    AppState,
};

/// Obtiene un único proyecto según su UUID
///
/// En caso de no encontrar el proyecto, genera un DBError con código 404. Lo que esperarías, vaya.
/// Si lo encuentra, envía una respuesta básica con los datos de ese proyecto.
#[get("/{id}")]
async fn get_single_member(
    path: web::Path<uuid::Uuid>,
    data: web::Data<AppState>,
) -> Result<BasicResponse<ProjectModel>, DBError> {
    let project_id = path.into_inner();

    Ok(BasicResponse::new(
        "Se ha encontrado el siguiente proyecto del club",
        Some(ProjectModel::find_by_id(project_id, &data.pool).await?),
    ))
}

#[get("")]
async fn get_projects(
    data: web::Data<AppState>,
) -> Result<BasicResponse<Vec<ProjectModel>>, DBError> {
    Ok(BasicResponse::new(
        "Se han conseguido los siguientes proyectos",
        Some(ProjectModel::get_all(&data.pool).await?),
    ))
}

#[post("/create")]
async fn create_project(data: web::Data<AppState>) -> Result<BasicResponse<()>, DBError> {
    unimplemented!();
}

#[put("/{id}")]
async fn update_project(
    path: web::Path<uuid::Uuid>,
    data: web::Data<AppState>,
) -> Result<BasicResponse<()>, DBError> {
    unimplemented!();
}

#[delete("/{id}")]
async fn delete_project(
    path: web::Path<uuid::Uuid>,
    data: web::Data<AppState>,
) -> Result<BasicResponse<()>, DBError> {
    unimplemented!();
}
