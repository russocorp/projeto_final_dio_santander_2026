use std::convert::Infallible;

use axum::extract::FromRequestParts;
use axum_extra::extract::{CookieJar, cookie::Cookie};
use jwt_simple::{
    algorithms::{HS256Key, MACLike},
    claims::Claims,
    reexports::coarsetime::Duration,
};

use crate::{
    app::{App, AppState},
    erros::AppError,
    models::usuario::{Usuario, UsuarioCreate, UsuarioLogado, UsuarioLogin},
    repositorio::Repositorio,
};

impl UsuarioLogin {
    pub async fn login(&self, repositorio: &Repositorio) -> Result<UsuarioLogado, AppError> {
        let usuario = match repositorio
            .get_usuario_user_name(&self.user_name)
            .await
            .map_err(AppError::DatabaseError)?
        {
            Some(usuario) => usuario,
            None => return Err(AppError::UsuarioInexistente),
        };

        match password_auth::verify_password(self.password.clone(), &usuario.hashed_password) {
            Ok(()) => {
                let usuario_logado = UsuarioLogado {
                    id: usuario.id,
                    user_name: usuario.user_name,
                    is_admin: usuario.is_admin,
                };
                Ok(usuario_logado)
            }
            _ => return Err(AppError::InvalidCredentials),
        }
    }
}

impl UsuarioCreate {
    pub async fn create_usuario(&self, repositorio: &Repositorio) -> Result<Usuario, AppError> {
        let mut usuario_create = self.clone();

        let hashed_password = password_auth::generate_hash(usuario_create.password.clone());

        if usuario_create.user_name == "admin" {
            usuario_create.is_admin = true;
        }

        let novo_usuario = match repositorio
            .create_usuario(hashed_password, &usuario_create)
            .await
        {
            Ok(novo_usuario) => novo_usuario,
            Err(sqlx::Error::Database(db_err)) if db_err.is_unique_violation() => {
                return Err(AppError::UsuarioDuplicado);
            }
            Err(err) => return Err(AppError::DatabaseError(err)),
        };
        Ok(novo_usuario)
    }
}

impl UsuarioLogado {
    pub async fn generate_auth_token(&self, repositorio: &Repositorio) -> Result<String, AppError> {
        let secret = std::env::var("JWT_SECRET_KEY")?;
        let key = HS256Key::from_bytes(secret.as_bytes());

        let usuario = match repositorio
            .get_usuario_user_name(&self.user_name)
            .await
            .map_err(AppError::DatabaseError)?
        {
            Some(usuario) => usuario,
            None => return Err(AppError::UsuarioInexistente),
        };
        let usuario_logado = UsuarioLogado {
            id: usuario.id,
            user_name: usuario.user_name.clone(),
            is_admin: usuario.is_admin,
        };

        let claims = Claims::with_custom_claims(usuario_logado, Duration::from_mins(20));
        let token = key.authenticate(claims)?;
        Ok(token)
    }

    pub async fn get_auth_token(token: &str) -> Result<Self, AppError> {
        let secret = std::env::var("JWT_SECRET_KEY")?;
        let key = HS256Key::from_bytes(secret.as_bytes());
        let claims: Self = key.verify_token(token, None)?.custom;
        Ok(Self {
            id: claims.id,
            user_name: claims.user_name,
            is_admin: claims.is_admin,
        })
    }
}

impl FromRequestParts<AppState> for UsuarioLogado {
    type Rejection = AppError;

    async fn from_request_parts(
        parts: &mut axum::http::request::Parts,
        _state: &AppState,
    ) -> Result<Self, Self::Rejection> {
        let jar = CookieJar::from_headers(&parts.headers);
        let token = match jar.get("token") {
            Some(token) => token.value(),
            None => return Err(AppError::InvalidCredentials),
        };
        match UsuarioLogado::get_auth_token(token).await {
            Ok(usuario) => Ok(usuario),
            Err(err) => Err(err),
        }
    }
}

impl FromRequestParts<AppState> for Option<UsuarioLogado> {
    type Rejection = Infallible;

    async fn from_request_parts(
        parts: &mut axum::http::request::Parts,
        _state: &AppState,
    ) -> Result<Self, Self::Rejection> {
        let jar = CookieJar::from_headers(&parts.headers);
        let token = match jar.get("token") {
            Some(token) => token.value(),
            None => return Ok(None),
        };
        match UsuarioLogado::get_auth_token(token).await {
            Ok(usuario) => Ok(Some(usuario)),
            Err(_) => Ok(None),
        }
    }
}
