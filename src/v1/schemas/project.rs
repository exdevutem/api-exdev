use serde::{Deserialize, Serialize};

use crate::v1::models::project::ProjectState;

/// Esquema de creación de proyectos
///
/// Solo el nombre del proyecto es necesario. Los involucrados pueden ser entregados via un arreglo
/// de UUID. Si no se puede parsear la UUID, la API arrojará un error (no panic).
#[derive(Serialize, Deserialize)]
pub struct CreateProjectSchema {
    /// Nombre del proyecto.
    pub name: String,

    /// Descripción del proyecto. La sugerencia es que sea un documento tipo Markdown, cosa de que
    /// pueda ser publicable y parseable fácilmente.
    pub description: Option<String>,

    /// Lista de UUID de los involucrados en un proyecto.
    pub involved: Option<Vec<uuid::Uuid>>,
}

/// Esquema de actualización del proyecto
///
/// Todos los valores son opcionales. Esta es la forma de actualizar el estado de un proyecto; esto
/// se hace via una String que sea textual un valor del Enum ProjectState.
#[derive(Serialize, Deserialize)]
pub struct UpdateProjectSchema {
    /// Nuevo nombre del proyecto.
    pub name: Option<String>,

    /// Nueva descripción del proyecto.
    pub description: Option<String>,

    /// Cómo alterar la lista de involucrados en el proyecto.
    pub involved: Option<UpdateInvolvedSchema>,

    /// Nuevo estado del proyecto.
    pub state: Option<ProjectState>,
}

/// Esquema de actualización de involucrados.
///
/// Permite agregar y eliminar involucrados vía un objeto durante la llamada de actualización del
/// proyecto.
#[derive(Serialize, Deserialize, Default)]
pub struct UpdateInvolvedSchema {
    /// Lista de UUID a agregar.
    pub add: Option<Vec<uuid::Uuid>>,

    /// Lista de UUID a eliminar.
    pub remove: Option<Vec<uuid::Uuid>>,
}
