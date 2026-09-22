use std::net::{Ipv4Addr, Ipv6Addr};

use super::{
    class::DnsClass,
    cursor::Cursor,
    error::DnsError,
    name::{encode_name, parse_name},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RecordType {
    A,
    NS,
    CNAME,
    SOA,
    PTR,
    MX,
    TXT,
    AAAA,
    Unknown(u16),
}

impl RecordType {
    pub fn from_u16(value: u16) -> Self {
        match value {
            1 => Self::A,
            2 => Self::NS,
            5 => Self::CNAME,
            6 => Self::SOA,
            12 => Self::PTR,
            15 => Self::MX,
            16 => Self::TXT,
            28 => Self::AAAA,
            value => Self::Unknown(value),
        }
    }

    pub fn to_u16(self) -> u16 {
        match self {
            Self::A => 1,
            Self::NS => 2,
            Self::CNAME => 5,
            Self::SOA => 6,
            Self::PTR => 12,
            Self::MX => 15,
            Self::TXT => 16,
            Self::AAAA => 28,
            Self::Unknown(value) => value,
        }
    }
}

#[derive(Debug)]
pub enum RecordData {
    A(Ipv4Addr),

    AAAA(Ipv6Addr),

    NS {
        host: String,
    },

    CNAME {
        host: String,
    },

    PTR {
        host: String,
    },

    MX {
        preference: u16,
        exchange: String,
    },

    SOA {
        mname: String,
        rname: String,
        serial: u32,
        refresh: u32,
        retry: u32,
        expire: u32,
        minimum: u32,
    },

    TXT {
        chunks: Vec<Vec<u8>>,
    },

    Unknown {
        bytes: Vec<u8>,
    },
}

#[derive(Debug)]
pub struct ResourceRecord {
    pub name: String,
    pub record_type: RecordType,
    pub class: DnsClass,
    pub ttl: u32,
    pub data: RecordData,
}

impl ResourceRecord {
    pub fn parse(
        cursor: &mut Cursor<'_>,
    ) -> Result<Self, DnsError> {
        let name = parse_name(cursor)?;

        let record_type =
            RecordType::from_u16(cursor.read_u16()?);

        let class =
            DnsClass::from_u16(cursor.read_u16()?);

        let ttl = cursor.read_u32()?;

        let rd_length =
            cursor.read_u16()? as usize;

        let rdata = cursor.read_bytes(rd_length)?;

        let data = match record_type {
            RecordType::A => {
                if rdata.len() != 4 {
                    return Err(DnsError::InvalidPacket(
                        "invalid A record length".to_string(),
                    ));
                }

                RecordData::A(Ipv4Addr::new(
                    rdata[0],
                    rdata[1],
                    rdata[2],
                    rdata[3],
                ))
            }

            RecordType::AAAA => {
                if rdata.len() != 16 {
                    return Err(DnsError::InvalidPacket(
                        "invalid AAAA record length".to_string(),
                    ));
                }

                RecordData::AAAA(Ipv6Addr::from([
                    rdata[0], rdata[1],
                    rdata[2], rdata[3],
                    rdata[4], rdata[5],
                    rdata[6], rdata[7],
                    rdata[8], rdata[9],
                    rdata[10], rdata[11],
                    rdata[12], rdata[13],
                    rdata[14], rdata[15],
                ]))
            }

            RecordType::TXT => {
                let mut chunks = Vec::new();
                let mut offset = 0;

                while offset < rdata.len() {
                    let len = rdata[offset] as usize;
                    offset += 1;

                    if offset + len > rdata.len() {
                        return Err(DnsError::InvalidPacket(
                            "invalid TXT record".to_string(),
                        ));
                    }

                    chunks.push(
                        rdata[offset..offset + len].to_vec()
                    );

                    offset += len;
                }

                RecordData::TXT { chunks }
            }

            _ => {
                RecordData::Unknown {
                    bytes: rdata.to_vec(),
                }
            }
        };

        Ok(Self {
            name,
            record_type,
            class,
            ttl,
            data,
        })
    }
}

impl ResourceRecord {
    pub fn encode(
        &self,
        buf: &mut Vec<u8>,
    ) -> Result<(), DnsError> {
        encode_name(&self.name, buf)?;

        buf.extend_from_slice(
            &self.record_type.to_u16().to_be_bytes()
        );

        buf.extend_from_slice(
            &self.class.to_u16().to_be_bytes()
        );
        
        buf.extend_from_slice(
            &self.ttl.to_be_bytes()
        );

        let mut rdata = Vec::new();

        match &self.data {
            RecordData::A(addr) => {
                rdata.extend_from_slice(
                    &addr.octets()
                );
            }

            RecordData::AAAA(addr) => {
                rdata.extend_from_slice(
                    &addr.octets()
                );
            }

            RecordData::MX {
                preference,
                exchange,
            } => {
                rdata.extend_from_slice(
                    &preference.to_be_bytes()
                );

                encode_name(exchange, &mut rdata)?;
            }

            RecordData::SOA {
                mname,
                rname,
                serial,
                refresh,
                retry,
                expire,
                minimum,
            } => {
                encode_name(mname, &mut rdata)?;
                encode_name(rname, &mut rdata)?;

                rdata.extend_from_slice(
                    &serial.to_be_bytes()
                );

                rdata.extend_from_slice(
                    &refresh.to_be_bytes()
                );

                rdata.extend_from_slice(
                    &retry.to_be_bytes()
                );

                rdata.extend_from_slice(
                    &expire.to_be_bytes()
                );

                rdata.extend_from_slice(
                    &minimum.to_be_bytes()
                );
            }

            RecordData::NS { host }
            | RecordData::CNAME { host }
            | RecordData::PTR { host } => {
                encode_name(host, &mut rdata)?;
            }

            RecordData::TXT { chunks } => {
                for chunk in chunks {
                    if chunk.len() > 255 {
                        return Err(DnsError::InvalidPacket(
                            "TXT record too long".to_string(),
                        ));
                    }

                    rdata.push(chunk.len() as u8);
                    rdata.extend_from_slice(&chunk);
                }
            }

                RecordData::Unknown { bytes } => {
                rdata.extend_from_slice(bytes);
            }
        }

        buf.extend_from_slice(
            &(rdata.len() as u16).to_be_bytes()
        );

        buf.extend_from_slice(&rdata);

        Ok(())
    }
}