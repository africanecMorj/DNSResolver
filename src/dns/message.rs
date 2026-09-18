use super::{
    error::DnsError,
    header::DnsHeader,
    question::Question,
};

#[derive(Debug)]
pub struct DnsMessage {
    pub header: DnsHeader,
    pub questions: Vec<Question>,
}

impl DnsMessage {
    pub fn parse(packet: &[u8]) -> Result<Self, DnsError> {
        let header = DnsHeader::parse(packet)?;

        let mut offset = DnsHeader::SIZE;

        let mut questions =
            Vec::with_capacity(header.qd_count as usize);

        for _ in 0..header.qd_count {
            questions.push(
                Question::parse(packet, &mut offset)?
            );
        }

        Ok(Self {
            header,
            questions,
        })
    }

    pub fn encode(&self) -> Result<Vec<u8>, DnsError> {
        let mut buf = Vec::with_capacity(512);

        let mut header = self.header;

        header.qd_count = self.questions.len() as u16;
        header.an_count = 0;
        header.ns_count = 0;
        header.ar_count = 0;

        header.encode(&mut buf);

        for question in &self.questions {
            question.encode(&mut buf)?;
        }

        Ok(buf)
    }
}