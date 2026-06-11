use axum::extract::FromRequestParts;

use crate::app::AppState;

pub struct Usuario {
    pub id: i32,
    pub username: String,
    pub admin: bool,
}

impl FromRequestParts<AppState> for Usuario {
    type Rejection = ();

    async fn from_request_parts(
        _parts: &mut axum::http::request::Parts,
        _state: &AppState,
    ) -> Result<Self, Self::Rejection> {
        // Aqui você pode implementar a lógica para extrair o usuário da requisição,
        // por exemplo, verificando um token de autenticação ou uma sessão.
        // Para simplificar, vamos retornar um usuário fictício.

        Ok(Usuario {
            id: 1,
            username: "usuario_exemplo".to_string(),
            admin: false,
        })
    }
}
