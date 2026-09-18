use super::error::DnsError;

pub fn parse_name(
    packet: &[u8],
    offset: &mut usize,
) -> Result<String, DnsError> {
    let mut labels = Vec::new();
    let mut pos = *offset;

    // Захист від pointer loops.
    let mut jumps = 0;
    let mut jumped = false;

    loop {
        if pos >= packet.len() {
            return Err(DnsError::Truncated);
        }

        let len = packet[pos];

        // Compression pointer: 11xxxxxx
        if len & 0xC0 == 0xC0 {
            if pos + 1 >= packet.len() {
                return Err(DnsError::Truncated);
            }

            let pointer = (((len as usize) & 0x3F) << 8)
                | packet[pos + 1] as usize;

            if pointer >= packet.len() {
                return Err(DnsError::InvalidName);
            }

            if !jumped {
                *offset = pos + 2;
                jumped = true;
            }

            pos = pointer;

            jumps += 1;

            if jumps > 32 {
                return Err(DnsError::InvalidName);
            }

            continue;
        }

        // Reserved compression bits.
        if len & 0xC0 != 0 {
            return Err(DnsError::InvalidName);
        }

        pos += 1;

        // End of name.
        if len == 0 {
            if !jumped {
                *offset = pos;
            }

            break;
        }

        let len = len as usize;

        if len > 63 {
            return Err(DnsError::InvalidLabel);
        }

        if pos + len > packet.len() {
            return Err(DnsError::Truncated);
        }

        let label = &packet[pos..pos + len];

        let label = std::str::from_utf8(label)
            .map_err(|_| DnsError::InvalidLabel)?;

        labels.push(label.to_string());

        pos += len;
    }

    if labels.is_empty() {
        Ok(".".to_string())
    } else {
        Ok(labels.join("."))
    }
}

pub fn encode_name(
    name: &str,
    buf: &mut Vec<u8>,
) -> Result<(), DnsError> {
    if name == "." {
        buf.push(0);
        return Ok(());
    }

    for label in name.trim_end_matches('.').split('.') {
        if label.is_empty() {
            return Err(DnsError::InvalidName);
        }

        if label.len() > 63 {
            return Err(DnsError::InvalidLabel);
        }

        buf.push(label.len() as u8);
        buf.extend_from_slice(label.as_bytes());
    }

    buf.push(0);

    Ok(())
}