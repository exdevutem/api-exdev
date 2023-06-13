use serde::{Deserialize, Serialize};

use super::club_member::ClubMemberModel;

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
    involved: Vec<ClubMemberModel>,
    name: String,
    description: String,
    state: ProjectState,
}
