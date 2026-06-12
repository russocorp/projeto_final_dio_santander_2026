use axum::Router;
use sqlx::PgPool;
use tokio::net::TcpListener;
use tower_http::services::{ServeDir, ServeFile};
use tracing::info;
use tracing_subscriber::{
    Layer, fmt::format::FmtSpan, layer::SubscriberExt, util::SubscriberInitExt,
};

use crate::{frontend, rotas};

#[derive(Clone)]
pub struct AppState {
    pub db: PgPool,
}

impl AppState {
    async fn new() -> color_eyre::Result<Self> {
        let database_url = std::env::var("DATABASE_URL")?;
        let db = PgPool::connect(&database_url).await?;
        Ok(Self { db })
    }
}

pub struct App;
impl App {
    pub async fn start() -> color_eyre::Result<()> {
        let layer = tracing_subscriber::fmt::layer()
            .with_span_events(FmtSpan::NEW)
            .boxed();

        tracing_subscriber::registry().with(layer).init();
        dotenvy::dotenv()?;

        info!("Iniciando serviço...");

        let state = AppState::new().await?;

        let listener = TcpListener::bind("127.0.0.1:8080").await?;
        info!("Serviço iniciado com sucesso na porta 8080!");
        let router = Router::new()
            //Rotas para verificar se tá rodando a API
            .merge(rotas::index::router())
            .nest("/api", rotas::api::router())
            .merge(frontend::login::router())
            .merge(frontend::registrar::router())
            .route_service(
                "/favicon.png",
                ServeFile::new("assets/image/favicon-32x32.png"),
            )
            .nest_service("/assets", ServeDir::new("assets"))
            .with_state(state);
        axum::serve(listener, router).await?;

        Ok(())
    }
}
