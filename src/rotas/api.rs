use crate::app::AppState;
use crate::auth::admin::Admin;
use crate::erros::AppError;
use crate::models::moeda::{Moeda, MoedaCreate, MoedaUpdate};
use crate::repositorio::Repositorio;
use axum::extract::Path;
use axum::{Json, Router, routing::get};

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/moedas", get(get_moedas).post(post_moedas))
        .route("/moedas/{id}", get(get_moeda).patch(patch_moedas))
}

#[tracing::instrument(skip_all)]
async fn get_moeda(repositorio: Repositorio, Path(id): Path<i32>) -> Result<Json<Moeda>, AppError> {
    match repositorio.get_moeda(id).await? {
        Some(moeda) => Ok(Json(moeda)),
        None => Err(AppError::NotFound),
    }
}

#[tracing::instrument(skip_all)]
async fn get_moedas(repositorio: Repositorio) -> Result<Json<Vec<Moeda>>, AppError> {
    let _moedas = repositorio.get_moedas().await?;

    Ok(Json(_moedas))
}

#[tracing::instrument(skip_all)]
async fn post_moedas(
    _admin: Admin,
    repositorio: Repositorio,
    Json(request): Json<MoedaCreate>,
) -> Result<Json<Moeda>, AppError> {
    let nova_moeda = repositorio.create_moeda(request).await?;

    Ok(Json(nova_moeda))
}

#[tracing::instrument(skip_all)]
async fn patch_moedas(
    _admin: Admin,
    repositorio: Repositorio,
    Path(id): Path<i32>,
    Json(request): Json<MoedaUpdate>,
) -> Result<Json<Moeda>, AppError> {
    match repositorio.update_moeda(id, request).await? {
        Some(moeda_atualizada) => Ok(Json(moeda_atualizada)),
        None => Err(AppError::NotFound),
    }
}
