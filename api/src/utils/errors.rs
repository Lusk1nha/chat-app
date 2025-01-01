use axum::{
    body::Body,
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};

use serde::Serialize;

#[derive(Serialize)]
pub struct ErrorResponse {
    pub message: String,
    #[serde(serialize_with = "status_code_to_u16")]
    pub status_code: StatusCode,
}

fn status_code_to_u16<S>(status_code: &StatusCode, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    serializer.serialize_u16(status_code.as_u16())
}

impl ErrorResponse {
    fn new(message: String, status_code: StatusCode) -> Self {
        Self {
            message,
            status_code,
        }
    }
}

impl IntoResponse for ErrorResponse {
    fn into_response(self) -> Response<Body> {
        // Retorna as mensagens de erro com o status code dinâmico
        (self.status_code, Json(self)).into_response()
    }
}
