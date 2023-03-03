use std::net::SocketAddr;

use tokio::{
    io::{AsyncBufReadExt, BufReader},
    net::{tcp::ReadHalf, TcpStream},
    select,
    sync::mpsc,
};
use tracing::info;

use crate::shutdown::Shutdown;

pub struct Connection {
    conn: TcpStream,
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
            conn: conn,
            addr: addr,
            shutdown: shutdown,
            _shutdown_complete: shutdown_complete,
        }
    }

    pub async fn io_loop(&mut self) {
        let (read_stream, _) = self.conn.split();
        let mut read_stream: BufReader<ReadHalf> = BufReader::new(read_stream);
        info!("new client {}", self.addr);
        while !self.shutdown.is_shutdown() {
            select! {
                res = async {
                    let mut data = String::new();
                    let res = read_stream.read_line(&mut data).await;
                    if let Ok(number) =  res {
                        if number == 0 || data == "quit\r\n" {
                            return Err(())
                        }
                        Ok(data)
                    } else {
                        Err(())
                    }
                } => {
                    if let Ok(data) = res {
                        info!(data);
                        continue;
                    }
                    break;
                }
                _ = self.shutdown.recv() => {}
            };
        }
    }
}
