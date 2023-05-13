use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
enum MemberState {
    Active,
    Unactive,
    Graduated,
    NoLongerAMember,
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct ClubMember {
    uuid: String,
    name: String,
    birthday: String,
    state: MemberState,
    email: String,
    github: String,
}
