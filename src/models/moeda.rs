use bigdecimal::BigDecimal;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Clone, sqlx::FromRow)]
pub struct Moeda {
    pub id: i32,
    pub nome: String,
    pub simbolo: String,
    pub valor: BigDecimal,
}

#[derive(Deserialize, Clone, sqlx::FromRow)]
pub struct MoedaUpdate {
    pub nome: Option<String>,
    pub simbolo: Option<String>,
    pub valor: Option<BigDecimal>,
}

#[derive(Deserialize, sqlx::FromRow)]
pub struct MoedaCreate {
    pub nome: String,
    pub simbolo: String,
    pub valor: BigDecimal,
}

#[derive(Serialize, Clone, sqlx::FromRow)]
pub struct Transacao {
    pub data: String,
    pub valor: BigDecimal,
}
