use askama::Template;
use axum::{
    Form, Router,
    http::StatusCode,
    response::{Html, IntoResponse, Redirect},
    routing::{get, post},
};
use axum_extra::extract::{CookieJar, cookie::Cookie};
use jwt_simple::token;
use serde::Deserialize;

use crate::{
    app::AppState,
    erros::AppError,
    models::usuario::{UsuarioLogado, UsuarioLogin},
    repositorio::Repositorio,
};

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/login", get(login))
        .route("/login", post(post_login))
}
#[derive(Template)]
#[template(path = "login.html")]
struct LoginPage {
    error_message: Option<String>,
}

#[derive(Deserialize)]
struct LoginPayload {
    user_name: String,
    password: String,
}

// Rota GET: Exibe a página de login limpa (sem erros)
async fn login(usuario_logado: Option<UsuarioLogado>) -> Result<impl IntoResponse, AppError> {
    if usuario_logado.is_some() {
        return Ok(Redirect::to("/").into_response());
    }

    let template = LoginPage {
        error_message: None,
    };

    match template.render() {
        Ok(html) => return Ok(Html(html).into_response()),
        Err(_) => Err(AppError::InternalServerError),
    }
}

// Rota POST: Processa as credenciais e retorna Result<(), AuthError>
async fn post_login(
    repositorio: Repositorio,
    jar: CookieJar,
    Form(payload): Form<LoginPayload>,
) -> Result<impl IntoResponse, AppError> {
    let usuario_login = UsuarioLogin {
        user_name: payload.user_name,
        password: payload.password,
    };

    // Executa o login e intercepta o erro diretamente
    match usuario_login.login(&repositorio).await {
        Ok(usuario) => {
            // faz algo com o usuário

            let token = match usuario.generate_auth_token(&repositorio).await {
                Ok(token) => token,
                Err(_) => return Err(AppError::InternalServerError),
            };

            let cookie = Cookie::build(("token", token)).http_only(true);

            return Ok((jar.add(cookie), Redirect::to("/")).into_response());
        }
        Err(err) => {
            let mensagem = err.to_string();

            // Renderiza o template de login injetando a mensagem de erro específica
            let template = LoginPage {
                error_message: Some(mensagem),
            };

            let status = match err {
                AppError::UsuarioInexistente | AppError::SenhaIncorreta => StatusCode::UNAUTHORIZED,
                _ => StatusCode::INTERNAL_SERVER_ERROR,
            };
            match template.render() {
                Ok(html) => return Ok((status, Html(html)).into_response()),
                Err(_) => Err(AppError::InternalServerError),
            }
        }
    }
}
