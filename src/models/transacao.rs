use bigdecimal::BigDecimal;
use serde::{Deserialize, Serialize};
use sqlx::types::Json;
use time::serde::iso8601;
use time::{Date, OffsetDateTime};

#[derive(Deserialize, Serialize, Clone, sqlx::FromRow)]
pub struct TransacaoCreate {
    pub id: i32,
    pub data: Date,
    pub valor_compra: BigDecimal,
    pub quantidade: BigDecimal,
}

#[derive(Deserialize, Serialize, Clone, sqlx::FromRow)]
pub struct Transacao {
    pub id: i32,
    pub id_usuarios: i32,
    pub id_moedas: i32,
    pub data_transacao: Date,
    pub quantidade: BigDecimal,
    pub valor_compra: BigDecimal,
}

// Cria um módulo inline para o Serde usar o formato YYYY-MM-DD com o tipo Date
time::serde::format_description!(data_formatada, Date, "[year]-[month]-[day]");

#[derive(Deserialize, Serialize, Clone, sqlx::FromRow, Debug)]
pub struct TransacaoUsuarioDetalhes {
    #[serde(with = "data_formatada")]
    pub data: Date,
    pub valor: BigDecimal,
    pub quantidade: BigDecimal,
    pub diferenca: BigDecimal,
}

#[derive(Deserialize, Serialize, Clone, sqlx::FromRow, Debug)]
pub struct TransacaoUsuario {
    pub id: i32,
    pub nome: String,
    pub simbolo: String,
    pub valor: BigDecimal,
    pub diferenca: BigDecimal,
    pub quantidade: BigDecimal,
    pub transacoes: Json<Vec<TransacaoUsuarioDetalhes>>,
}
