use super::{
    error::DnsError,
    message::DnsMessage,
    record::{RecordData, RecordType, ResourceRecord},
    Rcode,
};

#[derive(Debug)]
pub struct DnsResponse {
    message: DnsMessage,
}

impl DnsResponse {
    pub fn parse(packet: &[u8]) -> Result<Self, DnsError> {
        let message = DnsMessage::parse(packet)?;

        if !message.header.is_response() {
            return Err(DnsError::InvalidPacket(
                "packet is not a DNS response".to_string(),
            ));
        }

        Ok(Self { message })
    }

    pub fn id(&self) -> u16 {
        self.message.header.id
    }

    pub fn rcode(&self) -> Rcode {
        Rcode::from_u8(self.message.header.rcode())
    }

    pub fn is_success(&self) -> bool {
        self.rcode().is_success()
    }

    pub fn is_truncated(&self) -> bool {
        self.message.header.truncated()
    }

    pub fn recursion_available(&self) -> bool {
        self.message.header.recursion_available()
    }

    pub fn questions(&self) -> &[super::question::Question] {
        &self.message.questions
    }

    pub fn answers(&self) -> &[ResourceRecord] {
        &self.message.answers
    }

    pub fn authorities(&self) -> &[ResourceRecord] {
        &self.message.authorities
    }

    pub fn additionals(&self) -> &[ResourceRecord] {
        &self.message.additionals
    }

    pub fn records(&self, record_type: RecordType) -> Vec<&ResourceRecord> {
        self.answers()
            .iter()
            .filter(|record| record.record_type == record_type)
            .collect()
    }

    pub fn a_records(&self) -> Vec<&ResourceRecord> {
        self.records(RecordType::A)
    }

    pub fn aaaa_records(&self) -> Vec<&ResourceRecord> {
        self.records(RecordType::AAAA)
    }

    pub fn cname_records(&self) -> Vec<&ResourceRecord> {
        self.records(RecordType::CNAME)
    }

    pub fn ipv4_addresses(&self) -> Vec<std::net::Ipv4Addr> {
        self.a_records()
            .into_iter()
            .filter_map(|record| match &record.data {
                RecordData::A(address) => Some(*address),
                _ => None,
            })
            .collect()
    }

    pub fn ipv6_addresses(&self) -> Vec<std::net::Ipv6Addr> {
        self.aaaa_records()
            .into_iter()
            .filter_map(|record| match &record.data {
                RecordData::AAAA(address) => Some(*address),
                _ => None,
            })
            .collect()
    }

    pub fn cname(&self) -> Option<&str> {
        self.cname_records()
            .into_iter()
            .find_map(|record| match &record.data {
                RecordData::CNAME { host } => Some(host.as_str()),
                _ => None,
            })
    }

    pub fn into_message(self) -> DnsMessage {
        self.message
    }

    pub fn message(&self) -> &DnsMessage {
        &self.message
    }
}

impl DnsResponse {
    pub fn matches_query(
        &self,
        name: &str,
        record_type: RecordType,
    ) -> bool {
        self.questions().iter().any(|question| {
            question.name.eq_ignore_ascii_case(name)
                && question.qtype == record_type
        })
    }
}