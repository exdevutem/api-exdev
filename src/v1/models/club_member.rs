//! Modelo para un miembro del club.
//!
//! Este modelo corresponde a la abstracción de la tabla homónima, y que se usa para trabajar con
//! el estado de los miembros del club ExDev.

use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::sqlite::SqliteQueryResult;

use crate::v1::schemas::club_member::{CreateMemberSchema, UpdateMemberSchema};

/// Modelo de un miembro del club.
///
/// Consideramos tan solo un par de valores ahora mismo, pero creemos que hay más que quizás se
/// tengan que incluir a futuro, conforme se genere la discusión y se esclarezcan los mismos.
#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct ClubMemberModel {
    /// ID única de cada miembro.
    pub uuid: String,
    /// El nombre del integrante!
    pub name: String,
    /// Su cumpleaños! Opcional, por si alguno no quiere compartirlo para esta API.
    pub birthday: Option<String>,
    /// El estado actual del integrante, detallado más a fondo en su Enum.
    pub state: String,
    /// El email del integrante. También opcional por si no quiere compartirlo.
    pub email: Option<String>,
    /// El Github de este integrante!
    pub github: Option<String>,
    /// Fecha de creación de esta fila de la BDD.
    pub created_at: NaiveDateTime,
    /// Fecha de la Última modificación de este fila.
    pub updated_at: NaiveDateTime,
}

/// Estado de un miembro del club.
///
/// Se reconocen actualmente 4 estados distintos de un miembro del Exdev.
#[derive(Debug, Serialize, Deserialize)]
pub enum MemberState {
    // Integrante Activo del club.
    Active,
    // Integrante inactivo, pero aún parte del club.
    Unactive,
    // Ex miembro del club, que se ha graduado de este.
    Graduated,
    // Ex miembro del club, por diversas razones.
    NoLongerAMember,
}

// FIX: Esta implementación está mal, pues puede fallar. From se supone que es para
// implementaciones perfectas, pero al ser este un parseo, no siempre se dará que es perfecto. El
// caso obvio es el que es tratado en "_" del match. Deberíamos convertir esto en una
// implementación de "TryFrom".
impl From<String> for MemberState {
    /// Convierte un String a un Estado de un miembro.
    fn from(input: String) -> MemberState {
        use MemberState as ms;

        match input.as_str() {
            "Active" => ms::Active,
            "Unactive" => ms::Unactive,
            "Graduated" => ms::Graduated,
            "NoLongerAMember" => ms::NoLongerAMember,
            _ => ms::Unactive,
        }
    }
}

impl From<MemberState> for String {
    /// Convierte un estado de miembro a un String.
    fn from(val: MemberState) -> Self {
        use MemberState as ms;

        match val {
            ms::Active => String::from("Active"),
            ms::Unactive => String::from("Unactive"),
            ms::Graduated => String::from("Graduated"),
            ms::NoLongerAMember => String::from("NoLongerAMember"),
        }
    }
}

impl ClubMemberModel {
    /// Obtiene todos los integrantes del club.
    ///
    /// Esta función no filtra bajo ningún criterio (o no aún anyways), sino que entrega todos los
    /// integrantes del club sin más.
    pub async fn get_all(pool: &sqlx::SqlitePool) -> Result<Vec<ClubMemberModel>, sqlx::Error> {
        sqlx::query_as!(ClubMemberModel, "SELECT * FROM club_members")
            .fetch_all(pool)
            .await
    }

    /// Obtiene un único integrante del club según su UUID.
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
    }

    /// Crea (o agregar, supongo) un nuevo integrante del club
    pub async fn create(
        member_id: &str,
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

    /// Actualiza los datos de un integrante del club.
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
        .bind(new_data.name.to_owned().unwrap_or(member.name))
        // TODO: Verificar que sea una fecha.
        .bind(new_data.birthday.to_owned().or(member.birthday))
        .bind(new_data.email.to_owned().or(member.email))
        .bind(new_data.github.to_owned().or(member.github))
        .bind(new_data.state.to_owned().unwrap_or(member.state))
        .bind(member.uuid)
        .execute(pool)
        .await
    }

    /// Elimina a un integrante del club.
    ///
    /// Es importante notar que esta función solo los elimina de la base de datos, pero estas
    /// personas seguirán existiendo en la vida real! seguiremos trabajando para que también los
    /// elimine ahí, pero hasta no saber cómo hacer esto, seguiremos como estamos ahora mismo.
    pub async fn delete(
        member: &ClubMemberModel,
        pool: &sqlx::SqlitePool,
    ) -> Result<SqliteQueryResult, sqlx::Error> {
        sqlx::query!(r#"DELETE FROM club_members WHERE uuid = ?"#, member.uuid)
            .execute(pool)
            .await
    }
}
