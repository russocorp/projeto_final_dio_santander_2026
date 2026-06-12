use crate::app::App;

mod app;
mod auth;
mod erros;
mod frontend;
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

Executar os migrations no banco:
cargo sqlx migrate run

Iniciar com watch:
cargo watch -x check -x run
*/
