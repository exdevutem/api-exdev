//! Modelo para un proyecto
//!
//! Este modelo corresponde a la abstracción de la tabla homónima, y que se usa para trabajar con
//! el estado de los proyectos del club ExDev.

use serde::{Deserialize, Serialize};

use super::club_member::ClubMemberModel;

/// Estructura de un proyecto.
///
/// Contiene un vector de integrantes que es mapeado en una relación n:m.
#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Project {
    /// Identificador único del proyecto
    uuid: String,

    /// Integrantes relacionados a este proyecto.
    involved: Vec<ClubMemberModel>,

    /// Nombre del proyecto.
    name: String,

    /// Descripción del proyecto.
    description: String,

    /// Estado actual del proyecto.
    state: ProjectState,
}

/// Los distintos estados en los que se puede encontrar un proyecto.
#[derive(Debug, Deserialize, Serialize)]
enum ProjectState {
    /// El proyecto aún no inicia.
    NotStarted,

    /// El proyecto está progresando normalmente.
    InProgress,

    /// El proyecto está detenido, por alguna razón.
    Idle,

    /// El proyecto está en busca de nuevas ideas.
    LookingForIdeas,

    /// El proyecto ya fue finalizado!
    Finished,

    /// El proyecto fue cancelado. QEPD en paz.
    Cancelled,
}
