use std::net::SocketAddr;
use settings::Settings;

use tokio::sync::{oneshot, oneshot::Sender, oneshot::Receiver};

use signal_hook::consts::signal::{SIGHUP, SIGINT, SIGQUIT, SIGTERM};
use signal_hook_tokio::Signals;

use futures::stream::StreamExt;

mod app;
mod settings;

async fn handle_signals(mut signals: Signals, tx: Sender<()>) {
    while let Some(signal) = signals.next().await {
        match signal {
            SIGHUP => {
                // Reload configuration, reopen the log file...etc 
            }
            SIGTERM | SIGINT | SIGQUIT => {
                // Set the received boolean flag
                let _ = tx.send(());
                return ;
            },
            _ => unreachable!(),
        }
    }
}


#[tokio::main]
async fn main() {
    let app = app::create_app().await;

    let settings = Settings::new().expect("Error setting up the configurations.");
    println!("Using: server.port: {}", settings.server.port);

    let address = SocketAddr::from(([127, 0, 0, 1], settings.server.port));

    let signals = Signals::new(&[
        SIGHUP,
        SIGTERM,
        SIGINT,
        SIGQUIT,
    ]).unwrap();
    signals.handle();

    let (tx, rx): (Sender<()>, Receiver<()>) = oneshot::channel();

    let _signals_task = tokio::spawn(handle_signals(signals, tx));

    axum::Server::bind(&address)
    .serve(app.into_make_service())
    .with_graceful_shutdown(async {
        rx.await.ok();
        println!("Gracefully shutting down the system!");
        println!("Should close resources, drain REST calls, shutdown event handler gracefully...etc");
        
    })
    .await
    .expect("Failed to start server");

}
