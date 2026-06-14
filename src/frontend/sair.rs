use axum::{
    Router,
    response::{IntoResponse, Redirect, Response},
    routing::get,
};
use axum_extra::extract::{CookieJar, cookie::Cookie};

use crate::{app::AppState, erros::AppError};

pub fn router() -> Router<AppState> {
    Router::new().route("/logout", get(index))
}

async fn index(jar: CookieJar) -> Result<Response, AppError> {
    let jar = jar.remove(Cookie::from("token"));

    Ok((jar, Redirect::to("/login")).into_response())
}
