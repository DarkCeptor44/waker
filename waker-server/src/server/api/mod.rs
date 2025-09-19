mod v1;

use crate::server::Service;
use axum::Router;
use std::sync::Arc;

pub fn routes() -> Router<Arc<Service>> {
    Router::new().nest("/api/v1", v1::routes())
}
