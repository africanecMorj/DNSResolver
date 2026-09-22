use tokio::net::UdpSocket;

mod dns;

use dns::{
    DnsMessage,
    DnsResponse,
    RecordType,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let query = DnsMessage::query(
        1234,
        "google.com",
        RecordType::A,
    )?;

    let packet = query.encode()?;

    let socket = UdpSocket::bind("0.0.0.0:0").await?;

    socket
        .send_to(&packet, "8.8.8.8:53")
        .await?;

    let mut buffer = [0u8; 512];

    let (size, addr) = socket.recv_from(&mut buffer).await?;

    println!("received {size} bytes from {addr}");

    let response = DnsResponse::parse(&buffer[..size])?;

    println!("transaction id: {}", response.id());
    println!("rcode: {}", response.rcode());
    println!("success: {}", response.is_success());
    println!("truncated: {}", response.is_truncated());

    for address in response.ipv4_addresses() {
        println!("IPv4: {address}");
    }

    Ok(())
}