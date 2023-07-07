use std::net::SocketAddr;
use settings::Settings;

mod app;
mod settings;

#[tokio::main]
async fn main() {
    let app = app::create_app().await;

    let settings = Settings::new().expect("Error setting up the configurations.");
    println!("Using: server.port: {}", settings.server.port);

    let address = SocketAddr::from(([127, 0, 0, 1], settings.server.port));

    axum::Server::bind(&address)
    .serve(app.into_make_service())
    .await
    .expect("Failed to start server");
}
