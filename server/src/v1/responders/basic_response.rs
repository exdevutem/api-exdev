//! Respuesta simple de la API.
//!
//! La respuesta típica de la API es un estado de 200, un mensaje relacionado a la operación, y
//! opcionalmente los datos que se hayan solicitado.
use actix_web::{http::StatusCode, HttpRequest, HttpResponse, Responder};
use serde::Serialize;

/// La respuesta típica de la API en forma de estructura.
#[derive(Serialize)]
pub struct BasicResponse<T: Serialize> {
    /// Estado HTTP. Necesariamente es un u16 debido a que actic_web::http::StatusCode no
    /// implementa Serialize o Deserialize de serde.
    status: u16,

    /// Mensaje de la API respecto de la operación hecha.
    message: String,

    /// Los datos solicitados por el cliente, en caso de que existan.
    #[serde(skip_serializing_if = "Option::is_none")]
    data: Option<T>,
}

impl<T: Serialize> BasicResponse<T> {
    /// Crea una nueva respuesta típica
    pub fn new(message: impl Into<String>, data: Option<T>) -> BasicResponse<T> {
        BasicResponse {
            // FIX: Quizás no siempre deba ser un 200. Por ejemplo, los handlers de las operaciones
            // de creación quizás quieran devolver un estado 201 (Created).
            status: StatusCode::OK.into(),
            message: message.into(),
            data,
        }
    }
}

impl<T: Serialize> Responder for BasicResponse<T> {
    type Body = actix_web::body::BoxBody;

    /// Implementación para actix
    ///
    /// Esto evita tener que explicitar la respuesta después de cada Handler.
    fn respond_to(self, _req: &HttpRequest) -> HttpResponse<Self::Body> {
        HttpResponse::Ok().json(self)
    }
}
