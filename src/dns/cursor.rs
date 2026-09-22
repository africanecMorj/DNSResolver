use super::error::DnsError;

pub struct Cursor<'a> {
    packet: &'a [u8],
    offset: usize,
}

impl<'a> Cursor<'a> {
    pub fn new(packet: &'a [u8]) -> Self {
        Self {
            packet,
            offset: 0,
        }
    }

    pub fn position(&self) -> usize {
        self.offset
    }

    pub fn remaining(&self) -> usize {
        self.packet.len() - self.offset
    }

    pub fn read_u8(&mut self) -> Result<u8, DnsError> {
        if self.remaining() < 1 {
            return Err(DnsError::UnexpectedEof);
        }

        let value = self.packet[self.offset];
        self.offset += 1;

        Ok(value)
    }

    pub fn read_u16(&mut self) -> Result<u16, DnsError> {
        if self.remaining() < 2 {
            return Err(DnsError::UnexpectedEof);
        }

        let value = u16::from_be_bytes([
            self.packet[self.offset],
            self.packet[self.offset + 1],
        ]);

        self.offset += 2;

        Ok(value)
    }

    pub fn read_u32(&mut self) -> Result<u32, DnsError> {
        if self.remaining() < 4 {
            return Err(DnsError::UnexpectedEof);
        }

        let value = u32::from_be_bytes([
            self.packet[self.offset],
            self.packet[self.offset + 1],
            self.packet[self.offset + 2],
            self.packet[self.offset + 3],
        ]);

        self.offset += 4;

        Ok(value)
    }

    pub fn read_bytes(&mut self, len: usize) -> Result<&'a [u8], DnsError> {
        if self.remaining() < len {
            return Err(DnsError::UnexpectedEof);
        }

        let start = self.offset;
        self.offset += len;

        Ok(&self.packet[start..start + len])
    }

    pub fn packet(&self) -> &'a [u8] {
        self.packet
    }
}