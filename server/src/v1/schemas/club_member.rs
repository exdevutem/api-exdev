//! Esquemas relacionados a los integrantes del club.
//!
//! Existen solo dos tipos de esquemas; para crear un nuevo integrante y para actualizar a este
//! integrante. Además, existe un esquema para enviar los datos de un usuario desde algún handler,
//! de forma de no enviar nada que sea 'peligroso', o qué sé yo.

use serde::{Deserialize, Serialize};

use crate::v1::models::club_member::ClubMemberModel;

/// Estructura para la creación de un nuevo integrante.
///
/// El único campo requerido es el del nombre, pues todos los demás tienen valores por defecto y/u
/// opcionales.
#[derive(Serialize, Deserialize, Debug)]
pub struct CreateMemberSchema {
    /// Nombre del integrante a agregar.
    pub name: String,

    /// (Opcional) Cumpleaños del integrante.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub birthday: Option<String>,

    /// (Opcional) Email del integrante.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,

    /// (Opcional) Github del integrante.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub github: Option<String>,
}

/// Estructura para actualizar los datos de un nuevo integrante.
///
/// Todos sus valores son opcionales, y en caso de no estar presentes se opta por el valor anterior
/// que estuviera presente en la fila de ese integrante.
#[derive(Serialize, Deserialize, Debug)]
pub struct UpdateMemberSchema {
    // WARN: ¿Qué pasa si quiero eliminar algún valor que haya sido guardado en la bdd?.
    // Por ejemplo, algún integrante quiere quitar su cumpleaños de la bdd.
    /// Nombre del integrante.
    pub name: Option<String>,
    /// Cumpleaños del integrante.
    pub birthday: Option<String>,
    /// Email del integrante.
    pub email: Option<String>,
    /// Github del integrante.
    pub github: Option<String>,
    /// Estado del integrante.
    pub state: Option<String>,
}

/// Estructura para enviar a un integrante como respuesta.
///
/// La mayor diferencia con los esquemas anteriores es la presencia de la UUID y el estado del
/// integrante como valores requeridos para instanciarlos.
#[derive(Serialize, Deserialize, Debug)]
pub struct ClubMemberResponse {
    /// ID único del integrante.
    uuid: String,

    /// Nombre del integrante.
    name: String,

    /// Fecha de cumpleaños del integrante.
    #[serde(skip_serializing_if = "Option::is_none")]
    birthday: Option<String>,

    /// Estado del integrante.
    state: String,

    /// Email del integrante.
    #[serde(skip_serializing_if = "Option::is_none")]
    email: Option<String>,

    /// Github del integrante.
    #[serde(skip_serializing_if = "Option::is_none")]
    github: Option<String>,
}

impl ClubMemberResponse {
    /// Crea una nueva estructura de respuesta a partir de un modelo de la bdd.
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

    /// Convierte un slice (o vector) de modelos en un vector de respuestas
    ///
    /// # Ejemplo:
    /// ```
    /// // Vector de Modelos
    /// let modelos = ClubMemberModel::get_all(&data.pool);
    ///
    /// // Vector de respuestas.
    /// let respuestas = ClubMemberResponse::from_vector(modelos);
    /// ```
    pub fn from_vector(member_models: &[ClubMemberModel]) -> Vec<ClubMemberResponse> {
        member_models
            .iter()
            .map(|model| -> ClubMemberResponse { ClubMemberResponse::new(model) })
            .collect::<Vec<ClubMemberResponse>>()
    }
}

impl From<ClubMemberModel> for ClubMemberResponse {
    /// Convierte un modelo a una respuesta.
    ///
    /// Esta implementación fue creada para poder hacer las conversiones de forma implícita, y solo
    /// llama al método `new` de la respuesta, nada más.
    fn from(member_model: ClubMemberModel) -> ClubMemberResponse {
        ClubMemberResponse::new(&member_model)
    }
}
