use axum::response::{IntoResponse, Json, Response};
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
    #[error("Usuário não encontrado")]
    UsuarioInexistente,
    #[error("Não autorizado")]
    Unauthorized,
    #[error("Senha incorreta")]
    SenhaIncorreta,
    #[error("Erro interno do servidor")]
    InternalServerError,
    #[error(transparent)]
    DatabaseError(#[from] sqlx::Error),
    #[error("Nome de usuário já existe")]
    UsuarioDuplicado,
    #[error(transparent)]
    Template(#[from] askama::Error),
    #[error(transparent)]
    ConfiguracaoInvalida(#[from] std::env::VarError),
    #[error(transparent)]
    Jwt(#[from] jwt_simple::Error),
}
#[derive(Serialize)]
pub struct ErrorResponse {
    error: String,
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let error_message = self.to_string();
        let response = ErrorResponse {
            error: error_message,
        };
        let status = match self {
            Self::UsuarioDuplicado => axum::http::StatusCode::BAD_REQUEST,
            Self::MissingAuthorization | Self::InvalidCredentials | Self::SenhaIncorreta => {
                axum::http::StatusCode::UNAUTHORIZED
            }
            Self::NotFound | Self::UsuarioInexistente => axum::http::StatusCode::NOT_FOUND,
            Self::Unauthorized => axum::http::StatusCode::FORBIDDEN,
            Self::InternalServerError
            | Self::DatabaseError(_)
            | Self::Template(_)
            | Self::ConfiguracaoInvalida(_)
            | Self::Jwt(_) => axum::http::StatusCode::INTERNAL_SERVER_ERROR,
        };

        (status, Json(response).into_response()).into_response()
    }
}
