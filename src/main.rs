use crate::app::App;

mod app;
mod auth;
mod erros;
mod models;
mod repositorio;
mod rotas;

#[tokio::main]
async fn main() -> color_eyre::Result<()> {
    App::start().await
}

/*
Gerar novos migrations:
cargo sqlx migrate add --timestamp -r popular_moedas

Iniciar com watch:
cargo watch -x check -x run
*/
