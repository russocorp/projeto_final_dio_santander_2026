use askama::Template;
use axum::{
    Json, Router,
    response::{Html, IntoResponse, Redirect, Response},
    routing::{get, post},
};
use bigdecimal::BigDecimal;
use serde::Deserialize;
use std::str::FromStr;
use time::Date;
use time::macros::format_description;

use crate::{
    app::AppState,
    erros::AppError,
    models::{
        moeda::{Moeda, Transacao},
        transacao::TransacaoCreate,
        usuario::UsuarioLogado,
    },
    repositorio::Repositorio,
};

use crate::filters;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/", get(index))
        .route("/transacao", post(post_index))
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

#[derive(Deserialize)]
struct TransacaoPayload {
    pub id: i32,
    #[serde(deserialize_with = "deserializar_data")]
    pub data: Date,
    pub quantidade: BigDecimal,
}

async fn post_index(
    usuario_logado: Option<UsuarioLogado>,
    repositorio: Repositorio,
    Json(payload): Json<TransacaoPayload>,
) -> Result<Response, AppError> {
    //Ok(Redirect::to("/login").into_response())
    match usuario_logado {
        Some(usuario) => {
            // Implementação simples para criar um usuário do tipo administrador.
            let transacao_create = TransacaoCreate {
                id: payload.id,
                data: payload.data,
                quantidade: payload.quantidade,
            };
            let _transacao = repositorio
                .create_transacao(&usuario, &transacao_create)
                .await;

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

fn deserializar_data<'de, D>(deserializer: D) -> Result<Date, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    // Remove a parte do tempo se vier no formato ISO completo
    let s = s.split('T').next().unwrap_or(&s);
    let formato = format_description!("[year]-[month]-[day]");
    Date::parse(s, &formato).map_err(serde::de::Error::custom)
}
