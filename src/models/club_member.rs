use chrono::{NaiveDateTime, TimeZone};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
enum MemberState {
    Active,
    Unactive,
    Graduated,
    NoLongerAMember,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ClubMember {
    uuid: String,
    name: String,
    birthday: Option<String>,
    state: MemberState,
    email: Option<String>,
    github: Option<String>,
    created_at: chrono::DateTime<chrono::Local>,
    updated_at: chrono::DateTime<chrono::Local>,
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

impl ClubMember {
    pub fn new(cmm: &ClubMemberModel) -> ClubMember {
        ClubMember {
            uuid: cmm.uuid.to_owned(),
            name: cmm.name.to_owned(),
            birthday: cmm.birthday.to_owned(),
            email: cmm.email.to_owned(),
            github: cmm.github.to_owned(),
            created_at: chrono::Local.from_local_datetime(&cmm.created_at).unwrap(),
            updated_at: chrono::Local.from_local_datetime(&cmm.updated_at).unwrap(),
            state: match cmm.state.as_str() {
                "Active" => MemberState::Active,
                "Unactive" => MemberState::Unactive,
                "Graduated" => MemberState::Graduated,
                "NoLongerAMember" => MemberState::NoLongerAMember,
                _ => MemberState::Unactive,
            },
        }
    }
}
