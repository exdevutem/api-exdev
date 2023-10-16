use actix_web::web::{Data, Json, Path};
use actix_web::{delete, get, post, put};

use crate::v1::schemas::project::{CreateProjectSchema, UpdateProjectSchema};
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
    path: Path<uuid::Uuid>,
    data: Data<AppState>,
) -> Result<BasicResponse<ProjectModel>, DBError> {
    let project_id = path.into_inner();

    Ok(BasicResponse::new(
        "Se ha encontrado el siguiente proyecto del club",
        Some(ProjectModel::find_by_id(project_id, &data.pool).await?),
    ))
}

#[get("")]
async fn get_projects(data: Data<AppState>) -> Result<BasicResponse<Vec<ProjectModel>>, DBError> {
    Ok(BasicResponse::new(
        "Se han conseguido los siguientes proyectos",
        Some(ProjectModel::get_all(&data.pool).await?),
    ))
}

#[post("/create")]
async fn create_project(
    body: Json<CreateProjectSchema>,
    data: Data<AppState>,
) -> Result<BasicResponse<ProjectModel>, DBError> {
    let project = ProjectModel::create(body.into_inner(), &data.pool).await?;

    Ok(BasicResponse::new(
        "Se ha creado un nuevo proyecto",
        Some(project),
    ))
}

#[put("/update/{id}")]
async fn update_project(
    path: Path<uuid::Uuid>,
    body: Json<UpdateProjectSchema>,
    data: Data<AppState>,
) -> Result<BasicResponse<ProjectModel>, DBError> {
    let project = ProjectModel::update(path.into_inner(), body.into_inner(), &data.pool).await?;

    Ok(BasicResponse::new(
        "Se ha actualizado el proyecto",
        Some(project),
    ))
}

#[delete("/delete/{id}")]
async fn delete_project(
    path: Path<uuid::Uuid>,
    data: Data<AppState>,
) -> Result<BasicResponse<()>, DBError> {
    ProjectModel::delete(path.into_inner(), &data.pool).await?;

    Ok(BasicResponse::new("Se ha eliminado el proyecto {id}", None))
}
