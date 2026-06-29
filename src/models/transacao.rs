use bigdecimal::BigDecimal;
use serde::{Deserialize, Serialize};
use time::Date;

#[derive(Deserialize, Serialize, Clone, sqlx::FromRow)]
pub struct TransacaoCreate {
    pub id: i32,
    pub data: Date,
    pub quantidade: BigDecimal,
}

#[derive(Deserialize, Serialize, Clone, sqlx::FromRow)]
pub struct Transacao {
    pub id: i32,
    pub id_usuarios: i32,
    pub id_moedas: i32,
    pub data_transacao: Date,
    pub quantidade: BigDecimal,
}
