mod v1;

use super::Service;
use axum::Router;
use std::sync::Arc;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;
use v1::ApiDocV1;

pub fn routes() -> Router<Arc<Service>> {
    Router::new()
        .nest("/api/v1", v1::routes())
        .merge(SwaggerUi::new("/api/v1/docs").url("/api-docs/v1/openapi.json", ApiDocV1::openapi()))
}
