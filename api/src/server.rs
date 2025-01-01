use axum::Router;

pub async fn start_server(port: u16, api: Router) -> Result<(), std::io::Error> {
    tracing::info!("Starting server on port {}", port);

    let address = "127.0.0.1:".to_owned() + &port.to_string();
    let listener = tokio::net::TcpListener::bind(address).await?;

    axum::serve(listener, api).await
}
