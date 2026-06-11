use std::convert::Infallible;

use axum::extract::FromRequestParts;
use sqlx::PgPool;

use crate::{
    app::AppState,
    models::moedas::{Moeda, MoedaCreate, MoedaUpdate},
};

pub struct Repositorio {
    db: PgPool,
}

impl Repositorio {
    pub async fn get_moeda(&self, id: i32) -> sqlx::Result<Option<Moeda>> {
        let moeda = sqlx::query_as!(
            Moeda,
            "SELECT id, nome, simbolo, valor
            FROM moedas
            WHERE id = $1
            ",
            id
        )
        .fetch_optional(&self.db)
        .await?;

        Ok(moeda)
    }

    pub async fn get_moedas(&self) -> sqlx::Result<Vec<Moeda>> {
        let moedas = sqlx::query_as!(
            Moeda,
            "SELECT id, nome, simbolo, valor
            FROM moedas
            "
        )
        .fetch_all(&self.db)
        .await?;

        Ok(moedas)
    }

    pub async fn create_moeda(&self, moeda: MoedaCreate) -> sqlx::Result<Moeda> {
        let nova_moeda = sqlx::query_as!(
            Moeda,
            "INSERT INTO moedas (nome, simbolo, valor, inclusao_usuario)
            VALUES ($1, $2, $3, 'SISTEMA')
            RETURNING id, nome, simbolo, valor
            ",
            moeda.nome,
            moeda.simbolo,
            moeda.valor
        )
        .fetch_one(&self.db)
        .await?;

        Ok(nova_moeda)
    }

    pub async fn update_moeda(&self, id: i32, moeda: MoedaUpdate) -> sqlx::Result<Option<Moeda>> {
        let moeda_atualizada = sqlx::query_as!(
            Moeda,
            "UPDATE moedas
            SET nome = coalesce($1, nome), simbolo = coalesce($2, simbolo), valor = coalesce($3, valor), alteracao_usuario = 'SISTEMA', alteracao_data = NOW()
            WHERE id = $4
            RETURNING id, nome, simbolo, valor
            ",
            moeda.nome,
            moeda.simbolo,
            moeda.valor,
            id
        )
        .fetch_optional(&self.db)
        .await?;

        Ok(moeda_atualizada)
    }
}

impl FromRequestParts<AppState> for Repositorio {
    type Rejection = Infallible;

    async fn from_request_parts(
        _parts: &mut axum::http::request::Parts,
        state: &AppState,
    ) -> Result<Self, Self::Rejection> {
        Ok(Self {
            db: state.db.clone(),
        })
    }
}
