use askama::Template;
use axum::{
    Router,
    response::{Html, IntoResponse, Redirect, Response},
    routing::get,
};

use crate::{app::AppState, erros::AppError, models::usuario::UsuarioLogado};

pub fn router() -> Router<AppState> {
    Router::new().route("/", get(index))
}
#[derive(Template)]
#[template(path = "index.html")]
struct IndexPage {
    usuario: String,
}

// Rota GET: Exibe a página inicial
async fn index(usuario_logado: Option<UsuarioLogado>) -> Result<Response, AppError> {
    match usuario_logado {
        Some(usuario) => Ok(Html(
            IndexPage {
                usuario: usuario.user_name,
            }
            .render()?,
        )
        .into_response()),
        None => Ok(Redirect::to("/login").into_response()),
    }
}
