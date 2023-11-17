use configs::Configurations;
use std::net::SocketAddr;

mod app;
mod configs;
mod database;
pub mod models;
pub mod schema;
mod shutdown;

#[tokio::main]
async fn main() {
    // initialize tracing
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    let config = Configurations::new().expect("Error loading the configurations.");

    let app_state = database::get_connection_pool(&config);
    let app = app::create_app(app_state);

    let address: SocketAddr = format!("{}:{}", config.server.host, config.server.port)
        .parse()
        .expect("Unable to parse socket address");
    let rx = shutdown::register();

    tracing::info!("Starting server on {:?}", address);
    axum::Server::bind(&address)
        .serve(app.into_make_service())
        .with_graceful_shutdown(async {
            rx.await.ok(); // This will block until a shutdown signal is received
            tracing::info!("Handling graceful shutdown");
            tracing::info!("Close resources, drain and shutdown event handler... etc");
        })
        .await
        .expect("Failed to start server");
}
