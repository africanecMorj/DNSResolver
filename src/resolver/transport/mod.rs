use async_trait::async_trait;

pub mod tcp;
pub mod udp;

pub use tcp::TcpTransport;
pub use udp::UdpTransport;

#[derive(Debug)]
pub enum TransportError {
    Io(std::io::Error),
    Timeout,
}

impl std::fmt::Display for TransportError {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        match self {
            Self::Io(err) => {
                write!(f, "transport I/O error: {err}")
            }

            Self::Timeout => {
                write!(f, "transport timeout")
            }
        }
    }
}

impl std::error::Error for TransportError {}

impl From<std::io::Error> for TransportError {
    fn from(err: std::io::Error) -> Self {
        Self::Io(err)
    }
}

#[async_trait]
pub trait DnsTransport {
    async fn exchange(
        &self,
        packet: &[u8],
    ) -> Result<Vec<u8>, TransportError>;
}