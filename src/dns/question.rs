use super::{
    class::DnsClass,
    cursor::Cursor,
    error::DnsError,
    name::{encode_name, parse_name},
    record::RecordType,
};

#[derive(Debug)]
pub struct Question {
    pub name: String,
    pub qtype: RecordType,
    pub class: DnsClass,
}

impl Question {
    pub fn parse(
        cursor: &mut Cursor<'_>,
    ) -> Result<Self, DnsError> {
        let name = parse_name(cursor)?;

        let qtype = RecordType::from_u16(
            cursor.read_u16()?
        );

        let class = DnsClass::from_u16(
            cursor.read_u16()?
        );

        Ok(Self {
            name,
            qtype,
            class,
        })
    }

    pub fn encode(
        &self,
        out: &mut Vec<u8>,
    ) -> Result<(), DnsError> {
        encode_name(&self.name, out)?;

        out.extend_from_slice(
            &self.qtype
                .to_u16()
                .to_be_bytes()
        );

        out.extend_from_slice(
            &self.class
                .to_u16()
                .to_be_bytes()
        );

        Ok(())
    }
}