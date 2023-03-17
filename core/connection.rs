use std::net::SocketAddr;

use tokio::{net::TcpStream, select, sync::mpsc};
use tracing::info;

use crate::{
    package::{Message, Package},
    shutdown::Shutdown,
};

pub struct Connection {
    stream: Package,
    addr: SocketAddr,
    shutdown: Shutdown,
    _shutdown_complete: mpsc::Sender<()>,
}

impl Connection {
    pub fn new(
        conn: TcpStream,
        addr: SocketAddr,
        shutdown: Shutdown,
        shutdown_complete: mpsc::Sender<()>,
    ) -> Self {
        Self {
            stream: Package::new(conn),
            addr: addr,
            shutdown: shutdown,
            _shutdown_complete: shutdown_complete,
        }
    }

    pub async fn io_loop(&mut self) -> crate::Result<()> {
        info!("new client {}", self.addr);
        while !self.shutdown.is_shutdown() {
            let maybe_package = select! {
                 res = self.stream.read_message() => res?,
                _ = self.shutdown.recv() => {
                    return Ok(());
                }
            };

            let message = match maybe_package {
                Some(message) => message,
                None => {
                    return Ok(());
                }
            };
            let body = match message.body {
                Some(body) => body,
                None => return Ok(()),
            };

            info!(
                "new message, code {}, size {}, body {}",
                message.msgcode,
                body.len(),
                String::from_utf8(body.to_vec())?
            );

            self.stream.write_message(Message::new(2, body)).await?;
        }

        Ok(())
    }
}
