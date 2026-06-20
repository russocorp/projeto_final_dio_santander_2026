use askama::Template;
use axum::{
    Router,
    response::{Html, IntoResponse, Redirect, Response},
    routing::get,
};
use bigdecimal::BigDecimal;
use std::str::FromStr;

use crate::{
    app::AppState,
    erros::AppError,
    models::{
        moeda::{Moeda, Transacao},
        usuario::UsuarioLogado,
    },
    repositorio::{self, Repositorio},
};

pub fn router() -> Router<AppState> {
    Router::new().route("/", get(index))
}
#[derive(Template)]
#[template(path = "index.html")]
struct IndexPage {
    usuario: String,
    is_admin: bool,
    moedas: Vec<Moeda>,
    transacoes: Vec<Transacao>,
}

// Rota GET: Exibe a página inicial
async fn index(
    usuario_logado: Option<UsuarioLogado>,
    repositorio: Repositorio,
) -> Result<Response, AppError> {
    match usuario_logado {
        Some(usuario) => {
            let dados = repositorio.get_moedas().await?;
            let mut transacoes: Vec<Transacao> = Vec::new();
            transacoes.push(Transacao {
                data: "10/10/2025".to_string(),
                valor: BigDecimal::from_str("0.8").unwrap(),
            });

            Ok(Html(
                IndexPage {
                    usuario: usuario.user_name,
                    is_admin: usuario.is_admin,
                    moedas: dados,
                    transacoes,
                }
                .render()?,
            )
            .into_response())
        }
        None => Ok(Redirect::to("/login").into_response()),
    }
}
