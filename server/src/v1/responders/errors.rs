//! Respuestas de error de la API.
//!
//! Este módulo habilita el uso del operador `?` en la mayoría de handlers de la API. Es posible
//! que a futuro se necesite crear otros tipos de respuestas de error conforme otras operaciones
//! que sean implementadas puedan fallar.

use actix_web::{
    http::{header::ContentType, StatusCode},
    HttpResponse,
};
use serde_json::json;

/// Error de la base de datos.
///
/// Se toman todos tal cual son recibidos.
#[derive(Debug)]
pub struct DBError(sqlx::Error);

impl std::fmt::Display for DBError {
    /// ¿Cómo imprimo el error en pantalla?
    ///
    /// Simplemente uso la implementación hecha por sqlx::Error.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<sqlx::Error> for DBError {
    /// Convertir desde sqlx::Error a DBError.
    ///
    /// Esto habilita las conversiones implícitas, y hace que te saltes llamados a la función
    /// `map_err`.
    fn from(value: sqlx::Error) -> Self {
        Self(value)
    }
}

impl actix_web::error::ResponseError for DBError {
    /// Cuerpo de la respuesta del error.
    ///
    /// En caso de que el error sea tipo RowNotFound, el mensaje es un poco distinto. Fuera de eso,
    /// siempre se entrega el mismo error.
    fn error_response(&self) -> actix_web::HttpResponse<actix_web::body::BoxBody> {
        HttpResponse::build(self.status_code())
            .insert_header(ContentType::json())
            .json(match self.0 {
                sqlx::Error::RowNotFound => {
                    json!({"status": 404, "message": "No se ha encontrado el recurso buscado"})
                }
                _ => json!({"status": 500, "message": "Ocurrió un error en la bdd.", "debug": self.0.to_string()}),
            })
    }

    /// Código de error.
    ///
    /// Se va por defecto a una respuesta 500, pero existe la excepción de buscar datos que no
    /// existen, por lo que se entrega un error 404.
    fn status_code(&self) -> actix_web::http::StatusCode {
        match self.0 {
            sqlx::Error::RowNotFound => StatusCode::NOT_FOUND,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}
