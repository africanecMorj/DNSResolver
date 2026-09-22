use super::{
    cursor::Cursor,
    error::DnsError,
};

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

    out.push(0);

    Ok(())
}

pub fn parse_name(cursor: &mut Cursor<'_>) -> Result<String, DnsError> {
    let mut labels = Vec::new();

    let mut pos = cursor.position();
    let mut jumped = false;

    let mut jumps = 0;

    loop {
        if jumps > 32 {
            return Err(DnsError::CompressionLoop);
        }

        let byte = *cursor
            .packet()
            .get(pos)
            .ok_or(DnsError::UnexpectedEof)?;

        // Normal label
        if byte & 0xC0 == 0 {
            let len = byte as usize;

            // Root label
            if len == 0 {
                if !jumped {
                    cursor.read_u8()?;
                }

                break;
            }

            if len > 63 {
                return Err(DnsError::InvalidLabelLength);
            }

            let start = pos + 1;
            let end = start + len;

            let label = cursor
                .packet()
                .get(start..end)
                .ok_or(DnsError::UnexpectedEof)?;

            let label = std::str::from_utf8(label)
                .map_err(|_| DnsError::InvalidUtf8)?;

            labels.push(label.to_string());

            pos = end;

            if !jumped {
                // We are consuming bytes from the real cursor.
                cursor.read_u8()?;
                cursor.read_bytes(len)?;
            }

            continue;
        }

        // Compression pointer
        if byte & 0xC0 == 0xC0 {
            let second = *cursor
                .packet()
                .get(pos + 1)
                .ok_or(DnsError::UnexpectedEof)?;

            let pointer =
                (((byte as u16) & 0x3F) << 8)
                | second as u16;

            if !jumped {
                // Important:
                // consume the pointer in the main stream.
                cursor.read_u8()?;
                cursor.read_u8()?;
                jumped = true;
            }

            pos = pointer as usize;
            jumps += 1;

            continue;
        }

        return Err(DnsError::InvalidName);
    }

    Ok(labels.join("."))
}