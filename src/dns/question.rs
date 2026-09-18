use super::{
    error::DnsError,
    name::{encode_name, parse_name},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u16)]
pub enum RecordType {
    A = 1,
    NS = 2,
    CNAME = 5,
    SOA = 6,
    PTR = 12,
    MX = 15,
    TXT = 16,
    AAAA = 28,
}

impl TryFrom<u16> for RecordType {
    type Error = DnsError;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(Self::A),
            2 => Ok(Self::NS),
            5 => Ok(Self::CNAME),
            6 => Ok(Self::SOA),
            12 => Ok(Self::PTR),
            15 => Ok(Self::MX),
            16 => Ok(Self::TXT),
            28 => Ok(Self::AAAA),
            value => Err(DnsError::InvalidRecordType(value)),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DnsClass {
    IN = 1,
}

impl TryFrom<u16> for DnsClass {
    type Error = DnsError;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(Self::IN),
            _ => Err(DnsError::InvalidPacket("unsupported DNS class")),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Question {
    pub name: String,
    pub qtype: RecordType,
    pub class: DnsClass,
}

impl Question {
    pub fn parse(
        packet: &[u8],
        offset: &mut usize,
    ) -> Result<Self, DnsError> {
        let name = parse_name(packet, offset)?;

        if *offset + 4 > packet.len() {
            return Err(DnsError::Truncated);
        }

        let qtype = u16::from_be_bytes([
            packet[*offset],
            packet[*offset + 1],
        ]);

        let class = u16::from_be_bytes([
            packet[*offset + 2],
            packet[*offset + 3],
        ]);

        *offset += 4;

        Ok(Self {
            name,
            qtype: RecordType::try_from(qtype)?,
            class: DnsClass::try_from(class)?,
        })
    }

    pub fn encode(&self, buf: &mut Vec<u8>) -> Result<(), DnsError> {
        encode_name(&self.name, buf)?;

        buf.extend_from_slice(&(self.qtype as u16).to_be_bytes());
        buf.extend_from_slice(&(self.class as u16).to_be_bytes());

        Ok(())
    }
}