use axum::{Router, routing::get};
use time::OffsetDateTime;
use tokio::net::TcpListener;
use tracing::info;
use tracing_subscriber::{
    Layer, fmt::format::FmtSpan, layer::SubscriberExt, util::SubscriberInitExt,
};

pub struct App;
impl App {
    pub async fn start() -> color_eyre::Result<()> {
        let layer = tracing_subscriber::fmt::layer()
            .with_span_events(FmtSpan::NEW)
            .boxed();

        tracing_subscriber::registry().with(layer).init();

        info!("Iniciando serviço...");

        let listener = TcpListener::bind("127.0.0.1:8080").await?;
        let router = Router::new()
            .route("/", get(hello_world))
            .route("/ping", get(ping));
        axum::serve(listener, router).await?;

        info!("Serviço iniciado com sucesso!");
        Ok(())
    }
}

#[tracing::instrument]
async fn hello_world() -> &'static str {
    "Hello, World!!!"
}

#[tracing::instrument]
async fn ping() -> String {
    let agora = OffsetDateTime::now_local().unwrap_or_else(|_| OffsetDateTime::now_utc());
    let formatado = agora
        .format(&time::format_description::well_known::Iso8601::DEFAULT)
        .unwrap();
    // Ou formato personalizado:
    // let formatado = agora.format(&time::macros::format_description!("[year]-[month]-[day] [hour]:[minute]")).unwrap();
    format!("PONG! {}", formatado)
}
