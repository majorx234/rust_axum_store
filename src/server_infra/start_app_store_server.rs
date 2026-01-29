use std::net::SocketAddr;
use store::{config::Config, server_infra::{app_init::app_init, app_state::ServerElements}};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = Config::new();
    let server_state = ServerElements::new();
    let router = app_init(server_state);
    let listener = tokio::net::TcpListener::bind(config.get_host_socket_addr()).await?;
    tracing::debug!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, router.into_make_service_with_connect_info::<SocketAddr>()).await?;
    Ok(())
}
