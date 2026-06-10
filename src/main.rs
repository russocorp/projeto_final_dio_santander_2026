use crate::app::App;
mod app;

#[tokio::main]
async fn main() -> color_eyre::Result<()> {
    App::start().await
}
