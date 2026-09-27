use std::{
    net::SocketAddr,
    time::Duration,
};

use async_trait::async_trait;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
    time::timeout,
};

use super::{DnsTransport, TransportError};

pub struct TcpTransport {
    server: SocketAddr,
    timeout: Duration,
}

impl TcpTransport {
    pub fn new(server: SocketAddr) -> Self {
        Self {
            server,
            timeout: Duration::from_secs(5),
        }
    }

    pub fn with_timeout(
        server: SocketAddr,
        timeout: Duration,
    ) -> Self {
        Self {
            server,
            timeout,
        }
    }

    async fn connect(&self) -> Result<TcpStream, TransportError> {
        timeout(
            self.timeout,
            TcpStream::connect(self.server),
        )
        .await
        .map_err(|_| TransportError::Timeout)?
        .map_err(TransportError::Io)
    }
}

#[async_trait]
impl DnsTransport for TcpTransport {
    async fn exchange(
        &self,
        packet: &[u8],
    ) -> Result<Vec<u8>, TransportError> {
        if packet.len() > u16::MAX as usize {
            return Err(TransportError::Io(
                std::io::Error::new(
                    std::io::ErrorKind::InvalidInput,
                    "DNS packet is too large for TCP length prefix",
                ),
            ));
        }

        let mut stream = self.connect().await?;

        let length = packet.len() as u16;

        stream
            .write_all(&length.to_be_bytes())
            .await?;

        stream.write_all(packet).await?;

        let mut length_buf = [0u8; 2];

        timeout(
            self.timeout,
            stream.read_exact(&mut length_buf),
        )
        .await
        .map_err(|_| TransportError::Timeout)?
        .map_err(TransportError::Io)?;

        let response_len =
            u16::from_be_bytes(length_buf) as usize;

        if response_len == 0 {
            return Err(TransportError::Io(
                std::io::Error::new(
                    std::io::ErrorKind::InvalidData,
                    "DNS TCP response has zero length",
                ),
            ));
        }

        let mut response = vec![0u8; response_len];

        timeout(
            self.timeout,
            stream.read_exact(&mut response),
        )
        .await
        .map_err(|_| TransportError::Timeout)?
        .map_err(TransportError::Io)?;

        Ok(response)
    }
}