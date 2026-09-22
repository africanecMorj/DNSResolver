use super::{
    cursor::Cursor,
    error::DnsError,
};

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
    pub fn parse(
        cursor: &mut Cursor<'_>,
    ) -> Result<Self, DnsError> {
        Ok(Self {
            id: cursor.read_u16()?,

            flags: cursor.read_u16()?,

            qd_count: cursor.read_u16()?,

            an_count: cursor.read_u16()?,

            ns_count: cursor.read_u16()?,

            ar_count: cursor.read_u16()?,
        })
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
}