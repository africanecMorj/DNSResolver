use std::{
    net::SocketAddr,
    time::Duration,
};

use async_trait::async_trait;
use tokio::{
    net::UdpSocket,
    time::timeout,
};

use super::{
    DnsTransport,
    TransportError,
};

pub struct UdpTransport {
    server: SocketAddr,
    timeout: Duration,
}

impl UdpTransport {
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
}

#[async_trait]
impl DnsTransport for UdpTransport {
    async fn exchange(
        &self,
        packet: &[u8],
    ) -> Result<Vec<u8>, TransportError> {
        let socket =
            UdpSocket::bind("0.0.0.0:0").await?;

        socket
            .send_to(packet, self.server)
            .await?;

        let mut buffer = vec![0u8; 4096];

        let (size, _) = timeout(
            self.timeout,
            socket.recv_from(&mut buffer),
        )
        .await
        .map_err(|_| TransportError::Timeout)??;

        buffer.truncate(size);

        Ok(buffer)
    }
}