//! Modelo para un proyecto
//!
//! Este modelo corresponde a la abstracción de la tabla homónima, y que se usa para trabajar con
//! el estado de los proyectos del club ExDev.

use std::str::FromStr;

use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::{sqlite::SqliteQueryResult, QueryBuilder, Sqlite};

use crate::v1::schemas::{
    club_member::ClubMemberResponse,
    project::{CreateProjectSchema, UpdateProjectSchema},
};

use super::club_member::ClubMemberModel;

/// Estructura de un proyecto.
///
/// Contiene un vector de integrantes que es mapeado en una relación n:m.
#[derive(Serialize, Deserialize, sqlx::FromRow)]
pub struct ProjectModel {
    /// Identificador único del proyecto
    uuid: String,

    /// Nombre del proyecto.
    name: String,

    /// Descripción del proyecto.
    description: Option<String>,

    /// Estado actual del proyecto.
    state: ProjectState,

    /// Integrantes relacionados a este proyecto.
    #[sqlx(skip)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    involved: Vec<ClubMemberResponse>,

    /// Fecha de creación
    pub created_at: NaiveDateTime,

    /// Fecha de la Última modificación de este fila.
    pub updated_at: NaiveDateTime,
}

/// Los distintos estados en los que se puede encontrar un proyecto.
#[derive(Debug, Deserialize, Serialize, sqlx::Type)]
pub enum ProjectState {
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

impl TryFrom<String> for ProjectState {
    type Error = anyhow::Error;

    /// Converts a String into a ProjectState
    fn try_from(value: String) -> Result<Self, Self::Error> {
        value.parse()
    }
}

impl FromStr for ProjectState {
    type Err = anyhow::Error;

    /// Convers a &str into a ProjectState
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "NotStarted" => Ok(Self::NotStarted),
            "InProgress" => Ok(Self::InProgress),
            "Idle" => Ok(Self::Idle),
            "LookingForIdeas" => Ok(Self::LookingForIdeas),
            "Finished" => Ok(Self::Finished),
            "Cancelled" => Ok(Self::Cancelled),
            _ => Err(anyhow::anyhow!("No se ha podido parsear el String.")),
        }
    }
}

impl ProjectModel {
    pub async fn find_by_id(
        id: uuid::Uuid,
        pool: &sqlx::SqlitePool,
    ) -> Result<ProjectModel, sqlx::Error> {
        let id = id.to_string().to_owned();

        // Busco el proyecto.
        let mut project: ProjectModel =
            sqlx::query_as(r#"SELECT * FROM projects WHERE projects.uuid = $1"#)
                .bind(id.clone())
                .fetch_one(pool)
                .await?;

        let involved: Vec<ClubMemberModel> = sqlx::query_as(
            r#"SELECT club_members.* FROM club_members
            JOIN project_involvement ON project_involvement.club_member_uuid = club_members.uuid
            WHERE project_involvement.project_uuid = $1"#,
        )
        .bind(id)
        .fetch_all(pool)
        .await?;

        // Busco a los involucrados.
        project.involved = ClubMemberResponse::from_vector(&involved);

        Ok(project)
    }

    pub async fn get_all(pool: &sqlx::SqlitePool) -> Result<Vec<ProjectModel>, sqlx::Error> {
        sqlx::query_as(r#"SELECT * FROM projects"#)
            .fetch_all(pool)
            .await
    }

    pub async fn create(
        data: CreateProjectSchema,
        pool: &sqlx::SqlitePool,
    ) -> Result<ProjectModel, sqlx::Error> {
        let id = uuid::Uuid::new_v4();
        sqlx::query(r#"INSERT INTO projects(uuid, name, description) VALUES (?, ?, ?)"#)
            .bind(id.to_string())
            .bind(data.name)
            .bind(data.description)
            .execute(pool)
            .await?;

        // Agrega a todos los involucrados relacionados al proyecto.
        if let Some(member_ids) = data.involved {
            let mut qb: QueryBuilder<Sqlite> = QueryBuilder::new(
                "INSERT INTO project_involvement(project_uuid, club_member_uuid) ",
            );

            qb.push_values(member_ids, |mut b, value| {
                b.push_bind(id.to_string()).push_bind(value.to_string());
            });

            qb.build().execute(pool).await?;
        }

        ProjectModel::find_by_id(id, pool).await
    }

    pub async fn update(
        _data: &UpdateProjectSchema,
        _pool: &sqlx::SqlitePool,
    ) -> Result<(), sqlx::Error> {
        unimplemented!();
    }

    pub async fn delete(
        id: uuid::Uuid,
        pool: &sqlx::SqlitePool,
    ) -> Result<SqliteQueryResult, sqlx::Error> {
        sqlx::query("DELETE FROM projects WHERE $1")
            .bind(id.to_string())
            .execute(pool)
            .await
    }
}
