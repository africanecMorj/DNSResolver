use super::{
    header::DnsHeader,
    question::Question,
    record::RecordType,
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
    pub fn query(
        id: u16,
        name: &str,
        record_type: RecordType,
    ) -> Self {
        Self {
            header: DnsHeader {
                id,
                flags: 0x0100, 
                qd_count: 1,
                an_count: 0,
                ns_count: 0,
                ar_count: 0,
            },

            questions: vec![
                Question {
                    name: name.to_string(),
                    qtype: record_type,
                    class: DnsClass::IN,
                }
            ],

            answers: Vec::new(),
            authorities: Vec::new(),
            additionals: Vec::new(),
        }
    }
}

pub fn encode_name(
    name: &str,
    out: &mut Vec<u8>,
) -> Result<(), DnsError> {
    let name = name.trim_end_matches('.');

    if name.is_empty() {
        out.push(0);
        return Ok(());
    }

    for label in name.split('.') {
        if label.is_empty() {
            return Err(DnsError::InvalidName);
        }

        if label.len() > 63 {
            return Err(DnsError::InvalidLabelLength);
        }

        out.push(label.len() as u8);
        out.extend_from_slice(label.as_bytes());
    }

    // Root terminator
    out.push(0);

    Ok(())
}


impl DnsClass {
    pub fn to_u16(self) -> u16 {
        match self {
            DnsClass::IN => 1,
            DnsClass::Unknown(value) => value,
        }
    }
}