use super::error::DnsError;

#[derive(Debug, Clone, Copy)]
pub struct DnsHeader {
    pub id: u16,
    pub flags: u16,

    pub qd_count: u16,
    pub an_count: u16,
    pub ns_count: u16,
    pub ar_count: u16,
}

impl DnsHeader {
    pub const SIZE: usize = 12;

    pub fn parse(buf: &[u8]) -> Result<Self, DnsError> {
        if buf.len() < Self::SIZE {
            return Err(DnsError::Truncated);
        }

        Ok(Self {
            id: u16::from_be_bytes([buf[0], buf[1]]),
            flags: u16::from_be_bytes([buf[2], buf[3]]),
            qd_count: u16::from_be_bytes([buf[4], buf[5]]),
            an_count: u16::from_be_bytes([buf[6], buf[7]]),
            ns_count: u16::from_be_bytes([buf[8], buf[9]]),
            ar_count: u16::from_be_bytes([buf[10], buf[11]]),
        })
    }

    pub fn encode(&self, buf: &mut Vec<u8>) {
        buf.extend_from_slice(&self.id.to_be_bytes());
        buf.extend_from_slice(&self.flags.to_be_bytes());

        buf.extend_from_slice(&self.qd_count.to_be_bytes());
        buf.extend_from_slice(&self.an_count.to_be_bytes());
        buf.extend_from_slice(&self.ns_count.to_be_bytes());
        buf.extend_from_slice(&self.ar_count.to_be_bytes());
    }

    pub fn is_response(&self) -> bool {
        self.flags & 0x8000 != 0
    }

    pub fn recursion_desired(&self) -> bool {
        self.flags & 0x0100 != 0
    }

    pub fn recursion_available(&self) -> bool {
        self.flags & 0x0080 != 0
    }

    pub fn truncated(&self) -> bool {
        self.flags & 0x0200 != 0
    }

    pub fn rcode(&self) -> u8 {
        (self.flags & 0x000F) as u8
    }

    pub fn opcode(&self) -> u8 {
        ((self.flags >> 11) & 0x0F) as u8
    }
}