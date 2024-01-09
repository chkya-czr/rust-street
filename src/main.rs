use std::net::SocketAddr;

use settings::SETTINGS;

use tracing::info;
mod app;
mod routes;
mod services;
mod settings;

#[tokio::main]
async fn main() {
    let app = app::create_app().await;

    let port = SETTINGS.server.port;
    let address: SocketAddr = SocketAddr::from(([127, 0, 0, 1], port));

    info!("Server listening on {}", &address);
    axum::Server::bind(&address)
        .serve(app.into_make_service())
        .await
        .expect("Failed to start server");
}
