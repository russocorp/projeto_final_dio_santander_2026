use crate::app::App;

mod app;
mod auth;
mod erros;
mod models;
mod rotas;

#[tokio::main]
async fn main() -> color_eyre::Result<()> {
    App::start().await
}
