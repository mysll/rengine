use std::future::Future;

use tokio::sync::{broadcast, mpsc};
use tracing::{error, info};

use crate::{
    core::Core,
    options::{load_option, Options},
    tokio_util::run_local,
};

pub struct Runtime {}

impl Runtime {
    pub fn new() -> Self {
        let runtime = Self {};
        runtime
    }
}

pub async fn core_run(port: i32, shutdown: impl Future) {
    let (notify_shutdown, _) = broadcast::channel(1);
    let (shutdown_complete_tx, shutdown_complete_rx) = mpsc::channel(1);

    let mut server = Core {
        notify_shutdown,
        shutdown_complete_tx,
        shutdown_complete_rx,
        quit: false,
    };

    tokio::select! {
        res = server.run(port) => {
            if let Err(err) = res {
                error!(cause = %err, "failed to accept");
            }
        }
        _ = shutdown => {
            info!("shutting down");
        }
    }

    server.shutdown();

    let Core {
        mut shutdown_complete_rx,
        shutdown_complete_tx,
        notify_shutdown,
        ..
    } = server;

    drop(notify_shutdown);

    drop(shutdown_complete_tx);

    shutdown_complete_rx.recv().await;
}

pub fn run(options: &[impl Fn(&mut Options)], shutdown: impl Future) {
    let options = load_option(options);
    run_local(async {
        core_run(options.port, shutdown).await;
    });
}
