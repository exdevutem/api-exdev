//! Modelo para un proyecto
//!
//! Este modelo corresponde a la abstracción de la tabla homónima, y que se usa para trabajar con
//! el estado de los proyectos del club ExDev.

use std::str::FromStr;

use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::{sqlite::SqliteQueryResult, QueryBuilder, Row, Sqlite};
use uuid::Uuid;

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
        id: uuid::Uuid,
        data: UpdateProjectSchema,
        pool: &sqlx::SqlitePool,
    ) -> Result<ProjectModel, sqlx::Error> {
        // Valores anteriores.
        let previous = ProjectModel::find_by_id(id, pool).await?;

        // Actualizo el proyecto.
        sqlx::query(
            r#" UPDATE projects
                    SET name = ?, description = ?, state = ? WHERE uuid = ?"#,
        )
        .bind(data.name.unwrap_or(previous.name))
        .bind(
            data.description
                .unwrap_or(previous.description.unwrap_or_default()),
        )
        .bind(data.state.unwrap_or(previous.state))
        .bind(id.to_string())
        .execute(pool)
        .await?;

        let schema = data.involved.unwrap_or_default();
        // Si quieren agregar involucrados:
        if let Some(mut add) = schema.add {
            // Obtengo a los involucrados en el proyecto.
            let previous_ids = sqlx::query(
                "SELECT club_member_uuid FROM project_involvement WHERE project_uuid = ?",
            )
            .bind(id.to_string())
            .fetch_all(pool)
            .await?
            .iter()
            .map(|row| -> String { row.get("club_member_uuid") })
            .collect::<String>();

            // Quito los duplicados que quieren agregar.
            add.sort_unstable();
            add.dedup();

            // Quito los involucrados que ya están en el proyecto.
            let add = add
                .iter()
                .filter(|add_id| !previous_ids.contains(&add_id.to_string()))
                .collect::<Vec<&Uuid>>();

            // Agrego a los resultantes.
            let mut qb: QueryBuilder<Sqlite> = QueryBuilder::new(
                "INSERT INTO project_involvement(project_uuid, club_member_uuid) ",
            );

            qb.push_values(add, |mut b, value| {
                b.push_bind(id.to_string()).push_bind(value.to_string());
            });

            qb.build().execute(pool).await?;
        }

        // Si quieren quitar involucrados
        if let Some(mut remove) = schema.remove {
            if !remove.is_empty() {
                // Quito los duplicados.
                remove.sort_unstable();
                remove.dedup();

                let mut qb: QueryBuilder<Sqlite> = QueryBuilder::new(
                    "DELETE FROM project_involvement WHERE club_member_uuid IN (",
                );

                let mut separated = qb.separated(", ");

                for id in remove.iter() {
                    separated.push_bind(id.to_string());
                }

                separated.push_unseparated(") ");

                qb.build().execute(pool).await?;
            }
        }

        // Entrego el proyecto resultante.
        ProjectModel::find_by_id(id, pool).await
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
