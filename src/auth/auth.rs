use crate::{
    erros::AppError,
    models::usuario::{Usuario, UsuarioCreate, UsuarioLogin},
    repositorio::Repositorio,
};

impl UsuarioLogin {
    pub async fn login(&self, repositorio: &Repositorio) -> Result<Usuario, AppError> {
        let usuario = match repositorio
            .get_usuario_user_name(&self.user_name)
            .await
            .map_err(AppError::DatabaseError)?
        {
            Some(usuario) => usuario,
            None => return Err(AppError::UsuarioInexistente),
        };

        match password_auth::verify_password(self.password.clone(), &usuario.hashed_password) {
            Ok(()) => Ok(usuario),
            _ => return Err(AppError::InvalidCredentials),
        }
    }
}

impl UsuarioCreate {
    pub async fn create_usuario(&self, repositorio: &Repositorio) -> Result<Usuario, AppError> {
        let mut usuario_create = self.clone();

        let hashed_password = password_auth::generate_hash(usuario_create.password.clone());

        if usuario_create.user_name == "admin" {
            usuario_create.is_admin = true;
        }

        let novo_usuario = match repositorio
            .create_usuario(hashed_password, &usuario_create)
            .await
        {
            Ok(novo_usuario) => novo_usuario,
            Err(sqlx::Error::Database(db_err)) if db_err.is_unique_violation() => {
                return Err(AppError::UsuarioDuplicado);
            }
            Err(err) => return Err(AppError::DatabaseError(err)),
        };
        Ok(novo_usuario)
    }
}
