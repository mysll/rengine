
use tokio::{
    net::TcpListener,
    sync::{broadcast, mpsc},
};
use tracing::info;

use crate::{connection::Connection, shutdown::Shutdown};

pub struct Listener {
    pub listener: TcpListener,
    pub shutdown_complete_tx: mpsc::Sender<()>,
}

impl Listener {
    pub async fn run(&mut self, mut shutdown: Shutdown) {
        let (notify_shutdown, _) = broadcast::channel(1);
        let (tx, mut rx) = mpsc::channel(1);
        while !shutdown.is_shutdown() {
            tokio::select! {
                res = async {
                        if let Ok((conn, addr)) = self.listener.accept().await {
                            let mut connection = Connection::new(
                                conn,
                                addr,
                                Shutdown::new(notify_shutdown.subscribe()),
                                tx.clone(),
                            );
                            tokio::spawn(async move {
                                _ = connection.io_loop().await;
                                info!("client closed");
                            });
                            return Ok(())
                        }
                        Err(())
                } => {
                    if let Err(()) = res {
                        break;
                    }
                }
                _ = shutdown.recv() => {}
            }
        }
        drop(tx);
        drop(notify_shutdown);
        let _ = rx.recv().await;
        info!("listener closed");
    }
}

pub fn run_server(mut server: Listener, shutdown: Shutdown) -> crate::Result<()> {
    let thread_builder = std::thread::Builder::new().name(format!("net io"));
    // Spawn it
    thread_builder.spawn(move || {
        let rt = tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .worker_threads(4)
            .build()
            .unwrap();
        _ = rt.block_on(async {
            server.run(shutdown).await;
        });
    })?;
    Ok(())
}
