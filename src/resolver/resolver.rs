use super::{
    error::ResolverError,
    transport::DnsTransport,
};

use crate::dns::protocol::{
    message::DnsMessage,
    record::RecordType,
    response::DnsResponse,
};

pub struct Resolver<U, T> {
    udp: U,
    tcp: T,
}

impl<U, T> Resolver<U, T>
where
    U: DnsTransport,
    T: DnsTransport,
{
    pub fn new(udp: U, tcp: T) -> Self {
        Self { udp, tcp }
    }

    pub async fn lookup(
        &self,
        name: &str,
        record_type: RecordType,
    ) -> Result<DnsResponse, ResolverError> {
        let id = self.generate_id();

        let query = DnsMessage::query(
            id,
            name,
            record_type,
        )?;

        let packet = query.encode()?;

        // 1. Спочатку UDP
        let response_packet =
            self.udp.exchange(&packet).await?;

        let response =
            DnsResponse::parse(&response_packet)?;

        response.validate_id(id)?;

        // 2. UDP response має TC=1.
        //
        // Це означає, що відповідь була обрізана
        // і її потрібно повторити через TCP.
        if response.is_truncated() {
            let tcp_response_packet =
                self.tcp.exchange(&packet).await?;

            let tcp_response =
                DnsResponse::parse(&tcp_response_packet)?;

            tcp_response.validate_id(id)?;

            return Ok(tcp_response);
        }

        Ok(response)
    }

    fn generate_id(&self) -> u16 {
        use std::time::{
            SystemTime,
            UNIX_EPOCH,
        };

        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .subsec_nanos() as u16
    }
}