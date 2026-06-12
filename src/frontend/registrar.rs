use askama::Template;
use axum::{
    Form, Router,
    http::StatusCode,
    response::{Html, IntoResponse},
    routing::{get, post},
};
use serde::Deserialize;

use crate::{
    app::AppState, erros::AppError, models::usuario::UsuarioLogin, repositorio::Repositorio,
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
}

#[derive(Deserialize)]
struct RegistrarPayload {
    nome: String,
    user_name: String,
    password: String,
}

// Rota GET: Exibe a página de login limpa (sem erros)
async fn registrar() -> Result<Html<String>, AppError> {
    Ok(Html(
        RegistrarPage {
            error_message: None,
        }
        .render()
        .unwrap_or_else(|_| "Erro ao renderizar página".into()),
    ))
}

// Rota POST: Processa as credenciais e retorna Result<(), AuthError>
async fn post_registrar(
    repositorio: Repositorio,
    Form(payload): Form<RegistrarPayload>,
) -> impl IntoResponse {
    // Alterado para 'impl IntoResponse' para aceitar retornos de tipos diferentes

    let usuario_login = UsuarioLogin {
        user_name: payload.user_name,
        password: payload.password,
    };

    // Executa o login e intercepta o erro diretamente
    if let Err(err) = usuario_login.login(&repositorio).await {
        // Define a mensagem com base no AppError retornado pelo banco/regra de negócio
        let mensagem = err.to_string();

        // Renderiza o template de login injetando a mensagem de erro específica
        let template = RegistrarPage {
            error_message: Some(mensagem),
        };

        // Retorna a página com o erro (Status 401 para credenciais, ou 500 para falhas internas)
        let status = match err {
            AppError::UsuarioInexistente | AppError::SenhaIncorreta => StatusCode::UNAUTHORIZED,
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

    // Sucesso: Retorna página ou redireciona
    (
        StatusCode::OK,
        Html("<h1>Login efetuado com sucesso!</h1>".to_string()),
    )
        .into_response()
}
