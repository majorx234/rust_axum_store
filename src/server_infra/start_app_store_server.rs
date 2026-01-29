use std::net::SocketAddr;
use store::server_infra::{app_init::app_init, app_state::ServerElements};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    let server_state = ServerElements::new();
    let router = app_init(server_state);
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await?;
    tracing::debug!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, router.into_make_service_with_connect_info::<SocketAddr>()).await?;
    Ok(())
}
