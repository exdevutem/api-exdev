use serde::{Deserialize, Serialize};

use crate::v1::models::project::ProjectState;

#[derive(Serialize, Deserialize)]
pub struct CreateProjectSchema {
    pub name: String,

    pub description: Option<String>,

    pub involved: Option<Vec<uuid::Uuid>>,
}

#[derive(Serialize, Deserialize)]
pub struct UpdateProjectSchema {
    pub name: Option<String>,

    pub description: Option<String>,

    pub involved: Option<Vec<uuid::Uuid>>,

    pub state: Option<ProjectState>,
}
