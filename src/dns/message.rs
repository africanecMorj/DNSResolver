use super::{
    class::DnsClass,
    cursor::Cursor,
    error::DnsError,
    header::DnsHeader,
    question::Question,
    record::{RecordType, ResourceRecord},
};

#[derive(Debug)]
pub struct DnsMessage {
    pub header: DnsHeader,
    pub questions: Vec<Question>,
    pub answers: Vec<ResourceRecord>,
    pub authorities: Vec<ResourceRecord>,
    pub additionals: Vec<ResourceRecord>,
}

impl DnsMessage {
    pub fn encode(&self) -> Result<Vec<u8>, DnsError> {
        let mut out = Vec::with_capacity(512);

        // Header
        out.extend_from_slice(&self.header.id.to_be_bytes());
        out.extend_from_slice(&self.header.flags.to_be_bytes());

        out.extend_from_slice(
            &self.header.qd_count.to_be_bytes()
        );

        out.extend_from_slice(
            &self.header.an_count.to_be_bytes()
        );

        out.extend_from_slice(
            &self.header.ns_count.to_be_bytes()
        );

        out.extend_from_slice(
            &self.header.ar_count.to_be_bytes()
        );

        // Questions
        for question in &self.questions {
            question.encode(&mut out)?;
        }

        Ok(out)
    }
}

impl DnsMessage {
    pub fn query(
        id: u16,
        name: &str,
        qtype: RecordType,
    ) -> Result<Self, DnsError> {
        Ok(Self {
            header: DnsHeader {
                id,
                flags: 0x0100, // RD
                qd_count: 1,
                an_count: 0,
                ns_count: 0,
                ar_count: 0,
            },

            questions: vec![
                Question {
                    name: name.to_string(),
                    qtype,
                    class: DnsClass::IN,
                },
            ],

            answers: Vec::new(),
            authorities: Vec::new(),
            additionals: Vec::new(),
        })
    }

    pub fn parse(
        packet: &[u8],
    ) -> Result<Self, DnsError> {
        let mut cursor = Cursor::new(packet);

        let header = DnsHeader::parse(&mut cursor)?;

        let mut questions = Vec::new();

        for _ in 0..header.qd_count {
            questions.push(
                Question::parse(&mut cursor)?
            );
        }

        let mut answers = Vec::new();

        for _ in 0..header.an_count {
            answers.push(
                ResourceRecord::parse(&mut cursor)?
            );
        }

        let mut authorities = Vec::new();

        for _ in 0..header.ns_count {
            authorities.push(
                ResourceRecord::parse(&mut cursor)?
            );
        }

        let mut additionals = Vec::new();

        for _ in 0..header.ar_count {
            additionals.push(
                ResourceRecord::parse(&mut cursor)?
            );
        }

        Ok(Self {
            header,
            questions,
            answers,
            authorities,
            additionals,
        })
    }
}