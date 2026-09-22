use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Rcode {
    NoError,
    FormErr,
    ServFail,
    NXDomain,
    NotImp,
    Refused,
    Unknown(u8),
}

impl Rcode {
    pub fn from_u8(value: u8) -> Self {
        match value {
            0 => Self::NoError,
            1 => Self::FormErr,
            2 => Self::ServFail,
            3 => Self::NXDomain,
            4 => Self::NotImp,
            5 => Self::Refused,
            value => Self::Unknown(value),
        }
    }

    pub fn to_u8(self) -> u8 {
        match self {
            Self::NoError => 0,
            Self::FormErr => 1,
            Self::ServFail => 2,
            Self::NXDomain => 3,
            Self::NotImp => 4,
            Self::Refused => 5,
            Self::Unknown(value) => value,
        }
    }

    pub fn is_success(self) -> bool {
        matches!(self, Self::NoError)
    }
}

impl fmt::Display for Rcode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NoError => write!(f, "NOERROR"),
            Self::FormErr => write!(f, "FORMERR"),
            Self::ServFail => write!(f, "SERVFAIL"),
            Self::NXDomain => write!(f, "NXDOMAIN"),
            Self::NotImp => write!(f, "NOTIMP"),
            Self::Refused => write!(f, "REFUSED"),
            Self::Unknown(value) => write!(f, "UNKNOWN({value})"),
        }
    }
}