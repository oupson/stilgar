use axum::{extract::State, response::IntoResponse, routing::get, Router, Json};

use crate::state::AppState;

use super::model::Sensor;

pub(crate) fn router() -> Router<AppState> {
    Router::new().route("/list", get(list_sensors))
}

async fn list_sensors(State(app_state): State<AppState>) -> impl IntoResponse {
    Json(sqlx::query!("SELECT sensorMacAdresse, sensorName FROM SENSOR")
        .map(|row| Sensor::new(row.sensormacadresse, row.sensorname))
        .fetch_all(app_state.pool())
        .await
        .unwrap())
}
