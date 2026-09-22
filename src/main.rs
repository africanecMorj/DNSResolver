use tokio::net::UdpSocket;

use crate::dns::{
    DnsMessage,
    RecordType,
};

mod dns;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let message = DnsMessage::query(
        1234,
        "google.com",
        RecordType::A,
    )?;

    println!("{message:#?}");

    let socket =
        UdpSocket::bind("0.0.0.0:0").await?;

    socket
        .send_to(
            &[],
            "8.8.8.8:53",
        )
        .await?;

    Ok(())
}