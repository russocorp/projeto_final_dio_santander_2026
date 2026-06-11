use axum::Router;
use sqlx::PgPool;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::{net::TcpListener, sync::Mutex};
use tracing::info;
use tracing_subscriber::{
    Layer, fmt::format::FmtSpan, layer::SubscriberExt, util::SubscriberInitExt,
};

use crate::{models::moedas::Moeda, rotas};

#[derive(Clone)]
pub struct AppState {
    pub moedas: Arc<Mutex<HashMap<i32, Moeda>>>,
    pub db: PgPool,
}

impl AppState {
    async fn new() -> color_eyre::Result<Self> {
        let database_url = std::env::var("DATABASE_URL")?;
        let db = PgPool::connect(&database_url).await?;
        Ok(Self {
            moedas: Default::default(),
            db,
        })
    }
}

pub struct App;
impl App {
    pub async fn start() -> color_eyre::Result<()> {
        let layer = tracing_subscriber::fmt::layer()
            .with_span_events(FmtSpan::NEW)
            .boxed();

        tracing_subscriber::registry().with(layer).init();

        info!("Iniciando serviço...");

        let state = AppState::new().await?;

        let listener = TcpListener::bind("127.0.0.1:8080").await?;
        info!("Serviço iniciado com sucesso na porta 8080!");
        let router = Router::new()
            //Rotas para verificar se tá rodando a API
            .merge(rotas::index::router())
            .nest("/api", rotas::api::router())
            .with_state(state);
        axum::serve(listener, router).await?;

        Ok(())
    }
}
