#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DnsClass {
    IN,
    Unknown(u16),
}

impl DnsClass {
    pub fn from_u16(value: u16) -> Self {
        match value {
            1 => Self::IN,
            value => Self::Unknown(value),
        }
    }

    pub fn to_u16(self) -> u16 {
        match self {
            Self::IN => 1,
            Self::Unknown(value) => value,
        }
    }
}