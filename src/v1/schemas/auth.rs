//! Esquemas de los inputs de los handlers de Auth.
//!
//! Estos esquemas se refieren a la creación y actualización de nuevas aplicaciones dentro de la
//! API.

use serde::{Deserialize, Serialize};

/// Estructura de creación de una nueva aplicación.
#[derive(Serialize, Deserialize, Debug)]
pub struct CreateAppSchema {
    /// Nombre de la aplicación
    pub name: String,

    /// Descripción opcional de la aplicación.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

/// Estructura de actualización de una aplicación.
///
/// Notar dos cosas: no se cambia la UUID de la aplicación, y todos los valores son opcionales.
#[derive(Serialize, Deserialize, Debug)]
pub struct UpdateAppSchema {
    /// Nuevo nombre de la aplicación.
    pub name: Option<String>,

    /// Nueva descripción de la aplicación.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}
