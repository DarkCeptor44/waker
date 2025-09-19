use super::super::Service;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use chrono::Local;
use log::debug;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use waker::{create_magic_packet, wake_device, WakeOptions};

pub fn routes() -> Router<Arc<Service>> {
    Router::new()
        .route("/health", get(health))
        .route("/greet/{name}", get(greet))
        .route("/wakeup", post(wakeup))
}

async fn health(State(service): State<Arc<Service>>) -> String {
    let now = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();

    // this check is redundant but it shows how to use state on a handler
    if service.debug {
        debug!("Health check at {now}");
    }

    now
}

// TODO remove this example
async fn greet(Path(name): Path<String>) -> impl IntoResponse {
    #[derive(Serialize)]
    struct GreetError {
        message: String,
    }

    let name = name.trim();
    if name.is_empty() {
        let status = StatusCode::BAD_REQUEST;
        let body = Json(GreetError {
            message: "name cannot be empty".to_string(),
        });

        (status, body).into_response()
    } else {
        format!("Hello, {name}!").into_response()
    }
}

#[derive(Deserialize)]
struct WakeupRequest {
    mac: String,
}

async fn wakeup(Json(payload): Json<WakeupRequest>) -> impl IntoResponse {
    #[derive(Serialize)]
    struct WakeupResponse {
        message: String,
    }

    debug!("Received wakeup request for {}", payload.mac);

    let packet = match create_magic_packet(&payload.mac) {
        Ok(p) => p,
        Err(e) => {
            let status = StatusCode::BAD_REQUEST;
            let body = Json(WakeupResponse {
                message: e.to_string(),
            });
            return (status, body).into_response();
        }
    };

    match wake_device(WakeOptions::new(&packet)) {
        Ok(()) => {
            let status = StatusCode::OK;
            let body = Json(WakeupResponse {
                message: format!("Magic packet sent to {}", payload.mac),
            });
            (status, body).into_response()
        }
        Err(e) => {
            let status = StatusCode::INTERNAL_SERVER_ERROR;
            let body = Json(WakeupResponse {
                message: e.to_string(),
            });
            (status, body).into_response()
        }
    }
}
