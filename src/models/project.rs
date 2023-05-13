use crate::models::club_member::ClubMember;

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
enum ProjectState {
    NotStarted,
    InProgress,
    Idle,
    LookingForIdeas,
    Finished,
    Cancelled,
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Project {
    uuid: String,
    involved: Vec<ClubMember>,
    name: String,
    description: String,
    state: ProjectState,
}
