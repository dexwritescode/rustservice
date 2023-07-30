use tokio::sync::{oneshot, oneshot::Sender, oneshot::Receiver};
use signal_hook::consts::signal::{SIGHUP, SIGINT, SIGQUIT, SIGTERM};
use signal_hook_tokio::Signals;

use futures::stream::StreamExt;

pub fn register() -> Receiver<()> {
    let signals = Signals::new(&[
        SIGHUP,
        SIGTERM,
        SIGINT,
        SIGQUIT,
    ]).unwrap();
    signals.handle();

    let (tx, rx): (Sender<()>, Receiver<()>) = oneshot::channel();

    let _signals_task = tokio::spawn(handle_signals(signals, tx));

    return rx
}

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