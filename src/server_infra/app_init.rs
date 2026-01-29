use axum::{Router, routing::get};
use tower_http::trace::{DefaultMakeSpan, TraceLayer};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use crate::{config::Config, router::{hello_store::hello_store}};
use super::app_state::ServerElements;

pub fn app_init(server_state: ServerElements) -> Router {
    let config = Config::new();

    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(config.get_rust_log()))
        .with(tracing_subscriber::fmt::layer())
        .init();
    let routes_store = Router::new()
        .route("/store", get(hello_store))
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(DefaultMakeSpan::default().include_headers(true)),)
        .with_state(server_state);
    routes_store
}
