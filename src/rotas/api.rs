use crate::app::AppState;
use crate::auth::admin::Admin;
use crate::erros::AppError;
use crate::models::moedas::{Moeda, MoedaCreate, MoedaUpdate};
use axum::{Json, Router, extract::State, routing::get};

pub fn router() -> Router<AppState> {
    Router::new().route(
        "/moedas",
        get(get_moedas).post(post_moedas).patch(patch_moedas),
    )
}

#[tracing::instrument(skip_all)]
async fn get_moedas(state: State<AppState>) -> Json<Vec<Moeda>> {
    let _moedas = state.moedas.lock().await;

    Json(_moedas.values().cloned().collect())
}

#[tracing::instrument(skip_all)]
async fn post_moedas(
    _admin: Admin,
    state: State<AppState>,
    Json(request): Json<MoedaCreate>,
) -> Json<Moeda> {
    let mut _moedas = state.moedas.lock().await;

    let _id = _moedas.values().map(|m| m.id).max().unwrap_or(0) + 1;

    let nova_moeda = Moeda {
        id: _id,
        nome: request.nome,
        simbolo: request.simbolo,
        valor: request.valor,
    };

    _moedas.insert(_id, nova_moeda.clone());

    Json(nova_moeda)
}

#[tracing::instrument(skip_all)]
async fn patch_moedas(
    _admin: Admin,
    state: State<AppState>,
    Json(request): Json<MoedaUpdate>,
) -> Result<Json<Moeda>, AppError> {
    let mut _moedas = state.moedas.lock().await;

    let Some(moeda_alterar) = _moedas.get_mut(&request.id) else {
        return Err(AppError::NotFound);
    };

    moeda_alterar.nome = request.nome.clone();
    moeda_alterar.simbolo = request.simbolo.clone();
    moeda_alterar.valor = request.valor;
    Ok(Json(moeda_alterar.clone()))
}
