use std::fmt;

#[derive(Debug)]
pub enum DnsError {
    Truncated,
    InvalidPacket(&'static str),
    InvalidName,
    InvalidLabel,
    InvalidRecordType(u16),
}

impl fmt::Display for DnsError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Truncated => write!(f, "truncated DNS packet"),
            Self::InvalidPacket(msg) => write!(f, "invalid DNS packet: {msg}"),
            Self::InvalidName => write!(f, "invalid DNS name"),
            Self::InvalidLabel => write!(f, "invalid DNS label"),
            Self::InvalidRecordType(v) => {
                write!(f, "invalid DNS record type: {v}")
            }
        }
    }
}

impl std::error::Error for DnsError {}