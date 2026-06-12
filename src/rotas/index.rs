use crate::app::AppState;
use axum::{Router, routing::get};
use time::OffsetDateTime;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/", get(hello_world))
        .route("/ping", get(ping))
}

#[tracing::instrument(skip_all)]
async fn ping() -> String {
    let agora = OffsetDateTime::now_local().unwrap_or_else(|_| OffsetDateTime::now_utc());
    let formatado = agora
        .format(&time::macros::format_description!(
            "[year]-[month]-[day] [hour]:[minute]:[second]"
        ))
        .unwrap();
    format!("PONG - {}", formatado)
}

#[tracing::instrument(skip_all)]
async fn hello_world() -> &'static str {
    "Hello, World!!!"
}
