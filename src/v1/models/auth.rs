//! Modelo de aplicación que consume la API
//!
//! Este modelo identifica a alguna aplicación en específico. En términos concretos, puede
//! referirse a la página web del club, a la app mi utem si es que esta bebiera de la API, etc.
//!
//! Finalmente, corresponde a cualquier software que consuma los endpoints de esta API.

use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

/// Modelo de una fila de una aplicación.
///
/// Todos estos campos corresponden directamente con las columnas de la tabla de apps en la base de
/// datos.
#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct AppModel {
    /// Identificador único de la APP.
    pub uuid: String,
    /// Nombre de la App.
    pub name: String,
    /// Una descripción breve de la aplicación.
    pub description: Option<String>,
    /// El token de la App, manejado por el PrefixedApiKeyController de la estructura del estado de
    /// la aplicación.
    pub api_token: String,
    /// Timestamp de creación.
    pub created_at: NaiveDateTime,
    /// Timestamp con la última actualización de esta App.
    pub updated_at: NaiveDateTime,
    /// Timestamp opcional que indica cuando fue eliminada esta App.
    pub deleted_at: Option<NaiveDateTime>,
}
