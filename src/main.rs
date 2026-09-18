pub mod dns;

use dns::{
    DnsMessage,
    RecordType,
};

fn main() {
    let packet = [
        0x12, 0x34, // ID
        0x01, 0x00, // flags
        0x00, 0x01, // QDCOUNT
        0x00, 0x00, // ANCOUNT
        0x00, 0x00, // NSCOUNT
        0x00, 0x00, // ARCOUNT

        // www.example.com
        0x03,
        b'w', b'w', b'w',

        0x07,
        b'e', b'x', b'a', b'm',
        b'p', b'l', b'e',

        0x03,
        b'c', b'o', b'm',

        0x00,

        // QTYPE = A
        0x00, 0x01,

        // QCLASS = IN
        0x00, 0x01,
    ];

    let message = DnsMessage::parse(&packet)
        .unwrap();

    println!("{message:#?}");

    assert_eq!(
        message.questions[0].name,
        "www.example.com"
    );

    assert_eq!(
        message.questions[0].qtype,
        RecordType::A
    );
}