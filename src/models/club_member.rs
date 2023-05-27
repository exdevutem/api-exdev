use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum MemberState {
    Active,
    Unactive,
    Graduated,
    NoLongerAMember,
}

impl From<String> for MemberState {
    fn from(input: String) -> MemberState {
        match input.as_str() {
            "Active" => MemberState::Active,
            "Unactive" => MemberState::Unactive,
            "Graduated" => MemberState::Graduated,
            "NoLongerAMember" => MemberState::NoLongerAMember,
            _ => MemberState::Unactive,
        }
    }
}

impl Into<String> for MemberState {
    fn into(self: MemberState) -> String {
        match self {
            MemberState::Active => String::from("Active"),
            MemberState::Unactive => String::from("Unactive"),
            MemberState::Graduated => String::from("Graduated"),
            MemberState::NoLongerAMember => String::from("NoLongerAMember"),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct ClubMemberModel {
    pub uuid: String,
    pub name: String,
    pub birthday: Option<String>,
    pub state: String,
    pub email: Option<String>,
    pub github: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}
