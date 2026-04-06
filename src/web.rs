use crate::dns::SharedState;
use crate::models::DnsLog;
use axum::{extract::State, routing::get, Json, Router};
use tower_http::{cors::CorsLayer, services::ServeDir};

async fn get_reports(State(state): State<SharedState>) -> Json<Vec<DnsLog>> {
    let logs = state.read().await;
    let mut reversed: Vec<DnsLog> = logs.clone();
    reversed.reverse();
    Json(reversed)
}

pub fn create_router(shared_state: SharedState) -> Router {
    Router::new()
        .nest_service("/", ServeDir::new("public"))
        .route("/api/reports", get(get_reports))
        .layer(CorsLayer::permissive())
        .with_state(shared_state)
}