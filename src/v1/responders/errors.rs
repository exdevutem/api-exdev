use actix_web::{
    http::{header::ContentType, StatusCode},
    HttpResponse,
};

#[derive(Debug)]
pub struct DBError(sqlx::Error);

impl std::fmt::Display for DBError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<sqlx::Error> for DBError {
    fn from(value: sqlx::Error) -> Self {
        Self(value)
    }
}

impl actix_web::error::ResponseError for DBError {
    fn error_response(&self) -> actix_web::HttpResponse<actix_web::body::BoxBody> {
        HttpResponse::build(self.status_code())
            .insert_header(ContentType::json())
            .body(match self.0 {
                sqlx::Error::RowNotFound => "No se ha encontrado el recurso buscado",
                _ => "Ocurrió un error en la bdd.",
            })
    }

    fn status_code(&self) -> actix_web::http::StatusCode {
        match self.0 {
            sqlx::Error::RowNotFound => StatusCode::NOT_FOUND,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}
