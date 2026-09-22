use std::fmt;

#[derive(Debug)]
pub enum DnsError {
    Truncated,
    UnexpectedEof,

    InvalidPacket(String),

    InvalidName,
    InvalidLabelLength,
    InvalidUtf8,
    CompressionLoop,

    InvalidRecordType(u16),
    InvalidRecordClass(u16),
}

impl fmt::Display for DnsError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Truncated => write!(f, "DNS packet is truncated"),

            Self::UnexpectedEof => {
                write!(f, "unexpected end of DNS packet")
            }

            Self::InvalidPacket(msg) => {
                write!(f, "invalid DNS packet: {msg}")
            }

            Self::InvalidName => {
                write!(f, "invalid DNS name")
            }

            Self::InvalidLabelLength => {
                write!(f, "invalid DNS label length")
            }

            Self::InvalidUtf8 => {
                write!(f, "invalid UTF-8")
            }

            Self::CompressionLoop => {
                write!(f, "DNS compression loop")
            }

            Self::InvalidRecordType(value) => {
                write!(f, "invalid DNS record type: {value}")
            }

            Self::InvalidRecordClass(value) => {
                write!(f, "invalid DNS record class: {value}")
            }
        }
    }
}

impl std::error::Error for DnsError {}