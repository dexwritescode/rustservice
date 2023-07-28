use std::{net::SocketAddr, sync::{atomic::{AtomicBool, Ordering}, Arc}};
use settings::Settings;

use signal_hook::consts::signal::{SIGHUP, SIGINT, SIGQUIT, SIGTERM};
use signal_hook_tokio::Signals;

use futures::stream::StreamExt;

mod app;
mod settings;

async fn handle_signals(mut signals: Signals, recieved: Arc<AtomicBool>) {
    while let Some(signal) = signals.next().await {
        match signal {
            SIGHUP => {
                // Reload configuration, reopen the log file...etc 
            }
            SIGTERM | SIGINT | SIGQUIT => {
                // Set the received boolean flag
                recieved.store(true, Ordering::SeqCst);
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
    let recieved = Arc::new(AtomicBool::new(false));
    let signals_task = tokio::spawn(handle_signals(signals, Arc::clone(&recieved)));

    axum::Server::bind(&address)
    .serve(app.into_make_service())
    .with_graceful_shutdown(async {
        signals_task.await.unwrap();
        if recieved.load(Ordering::SeqCst) {
            println!("Gracefully shutting down the system!");
            println!("Should close resources, drain REST calls, shutdown event handler gracefully...etc");
        }
    })
    .await
    .expect("Failed to start server");

}
