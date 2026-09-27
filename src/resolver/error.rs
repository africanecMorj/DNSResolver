use crate::dns::protocol::error::DnsError;
use super::transport::TransportError;

#[derive(Debug)]
pub enum ResolverError {
    Transport(TransportError),
    Dns(DnsError),
}

impl From<TransportError> for ResolverError {
    fn from(err: TransportError) -> Self {
        Self::Transport(err)
    }
}

impl From<DnsError> for ResolverError {
    fn from(err: DnsError) -> Self {
        Self::Dns(err)
    }
}

impl std::fmt::Display for ResolverError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Transport(err) => {
                write!(f, "DNS transport error: {err}")
            }
            Self::Dns(err) => {
                write!(f, "DNS protocol error: {err}")
            }
        }
    }
}

impl std::error::Error for ResolverError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Transport(err) => Some(err),
            Self::Dns(err) => Some(err),
        }
    }
}