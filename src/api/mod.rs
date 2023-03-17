use axum::Router;

use crate::state::AppState;

pub(crate) mod model;

mod sensor;

pub(crate) fn router() -> Router<AppState> {
    Router::new()
    .nest("/sensor", sensor::router())
}