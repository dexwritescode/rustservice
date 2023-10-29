use settings::Settings;
use std::net::SocketAddr;

mod app;
mod settings;
mod shutdown;

#[tokio::main]
async fn main() {
    let app = app::create_app().await;

    let settings = Settings::new().expect("Error setting up the configurations.");
    println!("Using: server.port: {}", settings.server.port);

    let address = SocketAddr::from(([127, 0, 0, 1], settings.server.port));

    let rx = shutdown::register();

    axum::Server::bind(&address)
        .serve(app.into_make_service())
        .with_graceful_shutdown(async {
            rx.await.ok();
            println!("Gracefully shutting down the system!");
            println!(
                "Should close resources, drain REST calls, shutdown event handler gracefully...etc"
            );
        })
        .await
        .expect("Failed to start server");
}
