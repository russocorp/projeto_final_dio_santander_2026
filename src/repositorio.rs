use axum::extract::FromRequestParts;
use sqlx::PgPool;
use std::convert::Infallible;

use crate::{
    app::AppState,
    models::{
        moeda::{Moeda, MoedaUpdate},
        transacao::{Transacao, TransacaoCreate},
        usuario::{Usuario, UsuarioCreate, UsuarioLogado},
    },
};

pub struct Repositorio {
    db: PgPool,
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

    pub async fn create_moeda(&self, moeda: MoedaUpdate) -> sqlx::Result<Moeda> {
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

    pub async fn create_usuario(
        &self,
        _hashed_password: String,
        usuario: &UsuarioCreate,
    ) -> sqlx::Result<Usuario> {
        let novo_usuario = sqlx::query_as!(
            Usuario,
            "INSERT INTO usuarios (nome, user_name, hashed_password, is_admin, inclusao_usuario)
            VALUES ($1, $2, $3, $4, 'SISTEMA')
            RETURNING id, nome, user_name, hashed_password, is_admin
            ",
            usuario.nome,
            usuario.user_name,
            _hashed_password,
            usuario.is_admin
        )
        .fetch_one(&self.db)
        .await?;

        Ok(novo_usuario)
    }

    pub async fn get_usuario_user_name(&self, user_name: &String) -> sqlx::Result<Option<Usuario>> {
        let usuario = sqlx::query_as!(
            Usuario,
            "SELECT id, nome, user_name, hashed_password, is_admin
            FROM usuarios
            WHERE user_name = $1
            ",
            user_name
        )
        .fetch_optional(&self.db)
        .await?;

        Ok(usuario)
    }

    pub async fn create_transacao(
        &self,
        usuario: &UsuarioLogado,
        transacao: &TransacaoCreate,
    ) -> sqlx::Result<Transacao> {
        let nova_transacao = sqlx::query_as!(
            Transacao,
            "INSERT INTO transacoes (id_usuarios, id_moedas, data_transacao, quantidade, inclusao_usuario)
            VALUES ($1, $2, $3, $4, $5)
            RETURNING id, id_usuarios, id_moedas, data_transacao, quantidade
            ",
            usuario.id,
            transacao.id,
            transacao.data,
            transacao.quantidade,
            usuario.user_name
        )
        .fetch_one(&self.db)
        .await?;

        Ok(nova_transacao)
    }
}
