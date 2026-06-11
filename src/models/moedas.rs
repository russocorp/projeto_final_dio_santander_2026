use serde::{Deserialize, Serialize};

#[derive(Serialize, Clone)]
pub struct Moeda {
    pub id: i32,
    pub nome: String,
    pub simbolo: String,
    pub valor: f64,
}

#[derive(Deserialize, Clone)]
pub struct MoedaUpdate {
    pub id: i32,
    pub nome: String,
    pub simbolo: String,
    pub valor: f64,
}

#[derive(Deserialize)]
pub struct MoedaCreate {
    pub nome: String,
    pub simbolo: String,
    pub valor: f64,
}
