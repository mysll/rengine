use bytes::{Bytes, BytesMut};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt, BufWriter},
    net::TcpStream,
};
use tracing::info;

use crate::MAX_LEN;

pub struct Message {
    pub msgcode: i32,
    pub body: Option<Bytes>,
}

#[allow(dead_code)]
impl Message {
    pub fn new(msgcode: i32, body: Bytes) -> Self {
        Self {
            msgcode,
            body: Some(body),
        }
    }

    pub fn new_no_body(msgcode: i32) -> Self {
        Self {
            msgcode,
            body: None,
        }
    }
}

pub struct Package {
    stream: BufWriter<TcpStream>,
    buffer: BytesMut,
}

impl Package {
    pub fn new(socket: TcpStream) -> Self {
        Self {
            stream: BufWriter::new(socket),
            buffer: BytesMut::with_capacity(MAX_LEN),
        }
    }

    pub async fn read_message(&mut self) -> crate::Result<Option<Message>> {
        let mut size = self.stream.read_i32_le().await? as usize;
        if size < 4 || size > self.buffer.capacity() {
            return Err("size error".into());
        }
        let msgcode = self.stream.read_i32_le().await?;
        size -= 4;
        if size == 0 {
            return Ok(Some(Message {
                msgcode: msgcode,
                body: None,
            }));
        }
        self.buffer.clear();
        self.buffer.resize(size, 0);
        let body_size = self.stream.read_exact(&mut self.buffer).await?;
        if body_size != (size).try_into().unwrap() {
            return Err("read body error".into());
        }

        Ok(Some(Message {
            msgcode: msgcode,
            body: Some(Bytes::copy_from_slice(&self.buffer)),
        }))
    }

    pub async fn write_message(&mut self, mut messgae: Message) -> crate::Result<()> {
        let mut size: usize = 4;
        let msgcode = messgae.msgcode;
        if let Some(mut body) = messgae.body.take() {
            size += body.len();
            if size + 4 >= MAX_LEN {
                return Err("message size exceed".into());
            }
            self.stream.write_i32_le(size.try_into().unwrap()).await?;
            self.stream.write_i32_le(msgcode).await?;
            self.stream.write_buf(&mut body).await?;
        } else {
            self.stream.write_i32_le(size.try_into().unwrap()).await?;
            self.stream.write_i32_le(msgcode).await?;
        }
        self.stream.flush().await?;
        info!("send message code {} len {}", msgcode, size);
        Ok(())
    }
}
