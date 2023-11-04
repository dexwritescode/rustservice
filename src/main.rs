use configs::Configurations;
use std::net::SocketAddr;

mod app;
mod configs;
mod shutdown;

#[tokio::main]
async fn main() {
    let app = app::create_app().await;

    let config = Configurations::new().expect("Error loading the configurations.");
    println!("Using: server.port: {}", config.server.port);

    let address: SocketAddr = format!("{}:{}", config.server.host, config.server.port)
        .parse()
        .expect("Unable to parse socket address");

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
