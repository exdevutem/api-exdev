use serde::{Deserialize, Serialize};

use crate::models::club_member::ClubMemberModel;

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateMemberSchema {
    pub name: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub birthday: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub github: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ClubMemberResponse {
    uuid: String,
    name: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    birthday: Option<String>,
    state: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    email: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    github: Option<String>,
}

impl ClubMemberResponse {
    pub fn new(cmm: &ClubMemberModel) -> ClubMemberResponse {
        ClubMemberResponse {
            uuid: cmm.uuid.to_owned(),
            name: cmm.name.to_owned(),
            birthday: cmm.birthday.to_owned(),
            email: cmm.email.to_owned(),
            github: cmm.github.to_owned(),
            state: cmm.state.to_owned(),
        }
    }
}
