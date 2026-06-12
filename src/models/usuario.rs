use serde::Deserialize;

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
