use tokio::{
    net::TcpListener,
    sync::{broadcast, mpsc},
    time::{sleep, Duration},
};
use tracing::info;

use crate::{
    shutdown::Shutdown,
    tcp_server::{self, Listener},
};

pub struct Core {
    pub notify_shutdown: broadcast::Sender<()>,
    pub shutdown_complete_rx: mpsc::Receiver<()>,
    pub shutdown_complete_tx: mpsc::Sender<()>,
    pub quit: bool,
}

impl Core {
    pub async fn run(&mut self, port: i32) -> crate::Result<()> {
        let address = format!("0.0.0.0:{}", port);
        let listener = TcpListener::bind(&address).await?;
        info!("listen on {}", address);
        let (notify_shutdown, _) = broadcast::channel(1);

        let server = Listener {
            listener,
            shutdown_complete_tx: self.shutdown_complete_tx.clone(),
        };
        let shutdown = Shutdown::new(notify_shutdown.subscribe());
        tcp_server::run_server(server, shutdown)?;
        self.core_loop().await;
        drop(notify_shutdown);
        Ok(())
    }

    pub async fn core_loop(&mut self) {
        while !self.quit {
            sleep(Duration::from_millis(100)).await;
        }
    }

    pub fn shutdown(&mut self) {
        self.quit = true;
    }
}
