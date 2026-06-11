use axum::response::{IntoResponse, Json};
use serde::Serialize;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("Autorização não encontrada no header")]
    MissingAuthorization,
    #[error("Credenciais inválidas")]
    InvalidCredentials,
    #[error("Não encontrado")]
    NotFound,
    #[error("Não autorizado")]
    Unauthorized,
    #[error("Erro interno do servidor")]
    InternalServerError,
}
#[derive(Serialize)]
pub struct ErrorResponse {
    error: String,
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        let error_message = self.to_string();
        let response = ErrorResponse {
            error: error_message,
        };
        let status = match self {
            Self::MissingAuthorization | Self::InvalidCredentials => {
                axum::http::StatusCode::UNAUTHORIZED
            }
            Self::NotFound => axum::http::StatusCode::NOT_FOUND,
            Self::Unauthorized => axum::http::StatusCode::FORBIDDEN,
            Self::InternalServerError => axum::http::StatusCode::INTERNAL_SERVER_ERROR,
        };

        (status, Json(response).into_response()).into_response()
    }
}
