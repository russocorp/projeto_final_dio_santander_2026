use askama::Template;
use axum::{
    Router,
    response::{Html, IntoResponse, Redirect, Response},
    routing::get,
};

use crate::{app::AppState, erros::AppError, models::usuario::UsuarioLogado};

pub fn router() -> Router<AppState> {
    Router::new().route("/moedas", get(index))
}
#[derive(Template)]
#[template(path = "moedas.html")]
struct MoedasPage {
    usuario: String,
    is_admin: bool,
}

async fn index(usuario_logado: Option<UsuarioLogado>) -> Result<Response, AppError> {
    match usuario_logado {
        Some(usuario) => {
            if !usuario.is_admin {
                return Ok(Redirect::to("/").into_response());
            }
            Ok(Html(
                MoedasPage {
                    usuario: usuario.user_name,
                    is_admin: usuario.is_admin,
                }
                .render()?,
            )
            .into_response())
        }
        None => Ok(Redirect::to("/login").into_response()),
    }
}
