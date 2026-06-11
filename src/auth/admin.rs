use axum::extract::FromRequestParts;

use crate::{app::AppState, erros::AppError};

pub struct Admin;

impl FromRequestParts<AppState> for Admin {
    type Rejection = AppError;

    async fn from_request_parts(
        parts: &mut axum::http::request::Parts,
        _state: &AppState,
    ) -> Result<Self, Self::Rejection> {
        let Some(auth) = parts.headers.get(axum::http::header::AUTHORIZATION) else {
            return Err(AppError::MissingAuthorization);
        };
        if auth != "senha_teste" {
            return Err(AppError::InvalidCredentials);
        }

        Ok(Admin)
    }
}
