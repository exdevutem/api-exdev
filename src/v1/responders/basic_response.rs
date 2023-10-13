use actix_web::{http::StatusCode, HttpRequest, HttpResponse, Responder};
use serde::Serialize;

#[derive(Serialize)]
pub struct BasicResponse<T: Serialize> {
    status: u16,

    message: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    data: Option<T>,
}

impl<T: Serialize> BasicResponse<T> {
    pub fn new(message: impl Into<String>, data: Option<T>) -> BasicResponse<T> {
        BasicResponse {
            status: StatusCode::OK.into(),
            message: message.into(),
            data,
        }
    }
}

impl<T: Serialize> Responder for BasicResponse<T> {
    type Body = actix_web::body::BoxBody;

    fn respond_to(self, _req: &HttpRequest) -> HttpResponse<Self::Body> {
        HttpResponse::Ok().json(self)
    }
}
