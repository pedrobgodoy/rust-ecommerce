use actix_web::{
    error::{self},
    http::{header::ContentType, StatusCode},
    HttpResponse,
};
use derive_more::{Display, Error};
use serde::Serialize;

pub struct HttpOptions {
    pub host: String,
    pub port: u16,
}

#[derive(Debug, Display, Error)]
pub enum HttpError {
    #[display(fmt = "internal error")]
    InternalError,

    #[display(fmt = "not found")]
    NotFound,
}

#[derive(Serialize)]
struct HttpErrorResponseBody {
    message: String,
}

impl HttpErrorResponseBody {
    fn new(message: String) -> Self {
        Self { message }
    }
}

impl error::ResponseError for HttpError {
    fn error_response(&self) -> HttpResponse {
        let data = HttpErrorResponseBody::new(self.to_string());

        HttpResponse::build(self.status_code())
            .insert_header(ContentType::json())
            .body(serde_json::to_string(&data).unwrap())
    }

    fn status_code(&self) -> StatusCode {
        match *self {
            HttpError::InternalError => StatusCode::INTERNAL_SERVER_ERROR,
            HttpError::NotFound => StatusCode::NOT_FOUND,
        }
    }
}
