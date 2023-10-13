use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::sqlite::SqliteQueryResult;

use crate::v1::schemas::club_member::{CreateMemberSchema, UpdateMemberSchema};

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

impl ClubMemberModel {
    pub async fn get_all(pool: &sqlx::SqlitePool) -> Result<Vec<ClubMemberModel>, sqlx::Error> {
        sqlx::query_as!(ClubMemberModel, "SELECT * FROM club_members")
            .fetch_all(pool)
            .await
            .map_err(|e| e.into())
    }

    pub async fn get_one(
        member_id: impl Into<String>,
        pool: &sqlx::SqlitePool,
    ) -> Result<ClubMemberModel, sqlx::Error> {
        let member_id: String = member_id.into();
        sqlx::query_as!(
            ClubMemberModel,
            "SELECT * FROM club_members WHERE uuid = ?",
            member_id
        )
        .fetch_one(pool)
        .await
        .map_err(|e| e.into())
    }

    pub async fn create(
        member_id: &String,
        value: CreateMemberSchema,
        pool: &sqlx::SqlitePool,
    ) -> Result<SqliteQueryResult, sqlx::Error> {
        sqlx::query(
            r#"
    INSERT INTO club_members (uuid, name, birthday, email, github)
    VALUES (?, ?, ?, ?, ?)"#,
        )
        .bind(member_id.clone())
        .bind(value.name)
        .bind(value.birthday)
        .bind(value.email)
        .bind(value.github)
        .execute(pool)
        .await
    }

    pub async fn update(
        member: ClubMemberModel,
        new_data: UpdateMemberSchema,
        pool: &sqlx::SqlitePool,
    ) -> Result<SqliteQueryResult, sqlx::Error> {
        sqlx::query(
            r#"
    UPDATE club_members
    SET name = ?, birthday = ?, email = ?, github = ?, state = ? WHERE uuid = ?"#,
        )
        .bind(new_data.name.to_owned().unwrap_or_else(|| member.name))
        // TODO: Verificar que sea una fecha.
        .bind(new_data.birthday.to_owned().or(member.birthday))
        .bind(new_data.email.to_owned().or(member.email))
        .bind(new_data.github.to_owned().or(member.github))
        .bind(new_data.state.to_owned().unwrap_or_else(|| member.state))
        .bind(member.uuid)
        .execute(pool)
        .await
    }

    pub async fn delete(
        member: &ClubMemberModel,
        pool: &sqlx::SqlitePool,
    ) -> Result<SqliteQueryResult, sqlx::Error> {
        sqlx::query!(r#"DELETE FROM club_members WHERE uuid = ?"#, member.uuid)
            .execute(pool)
            .await
    }
}
