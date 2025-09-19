use super::super::Service;
use axum::{extract::State, routing::get, Router};
use chrono::Local;
use log::debug;
use std::sync::Arc;

pub fn routes() -> Router<Arc<Service>> {
    Router::new().route("/health", get(health))
}

async fn health(State(service): State<Arc<Service>>) -> String {
    let now = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();

    // this check is redundant but it shows how to use state on a handler
    if service.debug {
        debug!("Health check at {now}");
    }

    now
}
