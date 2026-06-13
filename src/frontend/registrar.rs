use askama::Template;
use axum::{
    Form, Router,
    http::StatusCode,
    response::{Html, IntoResponse, Redirect},
    routing::{get, post},
};
use axum_extra::extract::{CookieJar, cookie::Cookie};
use serde::Deserialize;

use crate::{
    app::AppState,
    erros::AppError,
    models::usuario::{UsuarioCreate, UsuarioLogado},
    repositorio::Repositorio,
};

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/registrar", get(registrar))
        .route("/registrar", post(post_registrar))
}
#[derive(Template)]
#[template(path = "registrar.html")]
struct RegistrarPage {
    error_message: Option<String>,
    old_nome: Option<String>,
    old_user_name: Option<String>,
}

#[derive(Deserialize)]
struct RegistrarPayload {
    nome: String,
    user_name: String,
    password: String,
}

async fn registrar(usuario_logado: Option<UsuarioLogado>) -> Result<impl IntoResponse, AppError> {
    if usuario_logado.is_some() {
        return Ok(Redirect::to("/").into_response());
    }

    let template = RegistrarPage {
        error_message: None,
        old_nome: None,
        old_user_name: None,
    };

    match template.render() {
        Ok(html) => return Ok(Html(html).into_response()),
        Err(_) => Err(AppError::InternalServerError),
    }
}

// Rota POST: Processa as credenciais e retorna Result<(), AuthError>
async fn post_registrar(
    repositorio: Repositorio,
    jar: CookieJar,
    Form(payload): Form<RegistrarPayload>,
) -> impl IntoResponse {
    // Implementação simples para criar um usuário do tipo administrador.
    let usuario_criar = UsuarioCreate {
        nome: payload.nome.clone(),
        user_name: payload.user_name.clone(),
        password: payload.password.clone(),
        is_admin: payload.nome.clone() == "admin",
    };

    // Executa o login e intercepta o erro diretamente
    if let Err(err) = usuario_criar.create_usuario(&repositorio).await {
        // Define a mensagem com base no AppError retornado pelo banco/regra de negócio
        let mensagem = err.to_string();

        // Renderiza o template de login injetando a mensagem de erro específica
        let template = RegistrarPage {
            error_message: Some(mensagem),
            old_nome: Some(usuario_criar.nome),
            old_user_name: Some(usuario_criar.user_name),
        };

        let status = match err {
            AppError::UsuarioDuplicado => StatusCode::BAD_REQUEST,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        };

        return match template.render() {
            Ok(html) => (status, Html(html)).into_response(),
            Err(_) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Erro crítico de renderização",
            )
                .into_response(),
        };
    }

    let cookie = Cookie::build(("flash", "Usuário Cadastrado com Sucesso!"))
        .path("/")
        .http_only(true);

    (jar.add(cookie), Redirect::to("/login")).into_response()
}
