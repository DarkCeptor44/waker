#![allow(clippy::needless_for_each)]

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
use utoipa::{IntoParams, OpenApi, ToSchema};
use waker::{create_magic_packet, wake_device, WakeOptions};

#[derive(OpenApi)]
#[openapi(
    info(title = "Waker API v1", version = "0.1.0",),
    paths(health, greet, wakeup),
    components(schemas(GreetError, WakeupRequest, WakeupResponse))
)]
pub struct ApiDocV1;

#[derive(Deserialize, ToSchema)]
struct WakeupRequest {
    /// MAC address of the machine to wake
    mac: String,
}

#[derive(Serialize, ToSchema, IntoParams)]
struct WakeupResponse {
    /// Message
    message: String,
}

#[derive(Serialize, ToSchema)]
struct GreetError {
    /// Error message
    message: String,
}

pub fn routes() -> Router<Arc<Service>> {
    Router::new()
        .route("/health", get(health))
        .route("/greet/{name}", get(greet))
        .route("/wakeup", post(wakeup))
}

/// Health check
#[utoipa::path(
    get,
    path = "/api/v1/health",
    responses(
        (status = 200, description = "Health check successful", body = String)
    )
)]
async fn health(State(service): State<Arc<Service>>) -> String {
    let now = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();

    // this check is redundant but it shows how to use state on a handler
    if service.debug {
        debug!("Health check at {now}");
    }

    now
}

// TODO remove this example
/// Greet a person
#[utoipa::path(
    get,
    path = "/api/v1/greet/{name}",
    params(
        ("name" = String, Path, description = "Name to greet", example = "John"),
    ),
    responses(
        (status = 200, description = "Greeted successfully", body = String, example = "Hello, John!"),
        (status = 400, description = "Name is empty", body = GreetError, example = json!({"message": "name cannot be empty"})),
    )
)]
async fn greet(Path(name): Path<String>) -> impl IntoResponse {
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

/// Wake up a machine with a magic packet
#[utoipa::path(
    post,
    path = "/api/v1/wakeup",
    request_body(content = WakeupRequest, description = "WakeupRequest with MAC address", examples(
        ("Failure" = (value = json!({"mac": "invalidmac"}))),
        ("Success" = (value = json!({"mac": "01:23:45:67:89:AB"}))),
    )),
    responses(
        (status = 200, description = "Magic packet sent", body = WakeupResponse, example = json!({"message": "Magic packet sent to 01:23:45:67:89:AB"})),
        (status = 400, description = "WakeupRequest is invalid", body = WakeupResponse, example = json!({"message": "Invalid MAC address"})),
        (status = 500, description = "Failed to send magic packet", body = WakeupResponse, example = json!({"message": "Failed to send magic packet"})),
    )
)]
async fn wakeup(Json(payload): Json<WakeupRequest>) -> impl IntoResponse {
    debug!("Received wakeup request for `{}`", payload.mac);

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
