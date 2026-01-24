use std::sync::{Arc, Mutex};

use axum::{Router, middleware, routing::get};

use rusqlite::{Connection, Result as RusqliteResult};
use std::net::SocketAddr;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use store::{
    config::Config,
    router::hello_world,
    server_state::{ServerElements, ServerState},
};

use tower::ServiceBuilder;
use tower_http::trace::TraceLayer;


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = Config::new();

    tracing_subscriber::registry()
    .with(tracing_subscriber::EnvFilter::new(config.get_rust_log()))
    .with(tracing_subscriber::fmt::layer())
    .init();

    let app_state:ServerState = Arc::new(Mutex::new(ServerElements::new()));
    let routes_all = Router::new()
        .route("/hello", get(hello_world::hello_handler))
        .with_state(app_state);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await?;
    tracing::debug!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, routes_all.into_make_service_with_connect_info::<SocketAddr>()).await?;
    println!("Axum server created!");
    Ok(())
}
