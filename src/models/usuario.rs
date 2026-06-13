use serde::{Deserialize, Serialize};

pub struct UsuarioLogin {
    pub user_name: String,
    pub password: String,
}

#[derive(Clone)]
pub struct Usuario {
    pub id: i32,
    pub nome: String,
    pub user_name: String,
    pub hashed_password: String,
    pub is_admin: bool,
}

#[derive(Deserialize, Clone)]
pub struct UsuarioCreate {
    pub nome: String,
    pub user_name: String,
    pub password: String,
    pub is_admin: bool,
}

#[derive(Deserialize, Serialize)]
pub struct UsuarioLogado {
    pub id: i32,
    pub user_name: String,
    pub is_admin: bool,
}
