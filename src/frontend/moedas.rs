use askama::Template;
use axum::{
    Json, Router,
    extract::Path,
    response::{Html, IntoResponse, Redirect, Response},
    routing::{get, patch, post},
};
use bigdecimal::BigDecimal;
use serde::Deserialize;

use crate::{
    app::AppState,
    erros::AppError,
    models::{
        moeda::{Moeda, MoedaUpdate},
        usuario::UsuarioLogado,
    },
    repositorio::Repositorio,
};

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/moedas", get(get_moedas))
        .route("/moedas", post(post_moedas))
        .route("/moedas/{id}", patch(patch_moedas))
}
#[derive(Template)]
#[template(path = "moedas.html")]
struct MoedasPage {
    usuario: String,
    is_admin: bool,
    moedas: Vec<Moeda>,
}

async fn get_moedas(
    usuario_logado: Option<UsuarioLogado>,
    repositorio: Repositorio,
) -> Result<Response, AppError> {
    match usuario_logado {
        Some(usuario) => {
            if !usuario.is_admin {
                return Ok(Redirect::to("/").into_response());
            }
            let dados = repositorio.get_moedas().await?;
            Ok(Html(
                MoedasPage {
                    usuario: usuario.user_name,
                    is_admin: usuario.is_admin,
                    moedas: dados,
                }
                .render()?,
            )
            .into_response())
        }
        None => Ok(Redirect::to("/login").into_response()),
    }
}

#[derive(Deserialize)]
struct MoedaPayload {
    pub nome: String,
    pub simbolo: String,
    pub valor: BigDecimal,
}

async fn post_moedas(
    usuario_logado: Option<UsuarioLogado>,
    repositorio: Repositorio,
    Json(payload): Json<MoedaPayload>, //Form(payload): Form<MoedaPayload>,
) -> Result<Response, AppError> {
    match usuario_logado {
        Some(usuario) => {
            if !usuario.is_admin {
                return Ok(Redirect::to("/").into_response());
            }
            // Implementação simples para criar um usuário do tipo administrador.
            let moeda_update = MoedaUpdate {
                nome: Some(payload.nome.clone()),
                simbolo: Some(payload.simbolo.clone()),
                valor: Some(payload.valor),
            };
            //let _alterado = repositorio.update_moeda(id, moeda_update).await;
            let _moeda = repositorio.create_moeda(moeda_update).await;

            let dados = repositorio.get_moedas().await?;
            Ok(Html(
                MoedasPage {
                    usuario: usuario.user_name,
                    is_admin: usuario.is_admin,
                    moedas: dados,
                }
                .render()?,
            )
            .into_response())
        }
        None => Ok(Redirect::to("/login").into_response()),
    }
}

async fn patch_moedas(
    usuario_logado: Option<UsuarioLogado>,
    repositorio: Repositorio,
    Path(id): Path<i32>,
    Json(payload): Json<MoedaPayload>, //Form(payload): Form<MoedaPayload>,
) -> Result<Response, AppError> {
    match usuario_logado {
        Some(usuario) => {
            if !usuario.is_admin {
                return Ok(Redirect::to("/").into_response());
            }
            // Implementação simples para criar um usuário do tipo administrador.
            let moeda_update = MoedaUpdate {
                nome: Some(payload.nome.clone()),
                simbolo: Some(payload.simbolo.clone()),
                valor: Some(payload.valor),
            };
            //let _alterado = repositorio.update_moeda(id, moeda_update).await;
            let _moeda = match repositorio
                .update_moeda(id, moeda_update)
                .await
                .map_err(AppError::DatabaseError)?
            {
                Some(moeda) => moeda,
                None => return Err(AppError::InternalServerError),
            };

            let dados = repositorio.get_moedas().await?;
            Ok(Html(
                MoedasPage {
                    usuario: usuario.user_name,
                    is_admin: usuario.is_admin,
                    moedas: dados,
                }
                .render()?,
            )
            .into_response())
        }
        None => Ok(Redirect::to("/login").into_response()),
    }
}
