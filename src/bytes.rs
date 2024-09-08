use crate::err::ConversionError;
use crate::types::U4;

const HEX_VALUES: [char; 16] = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'a', 'b', 'c', 'd', 'e', 'f'];

fn in_hex(val: U4) -> char {
    HEX_VALUES[val.to_u8() as usize]
}

pub fn hex_to_bytes(hex: &str) -> Result<Vec<u8>, ConversionError> {
    if hex.len() % 2 == 1 {
        return Err(ConversionError::InvalidLength);
    }

    let mut hex_in_bytes: Vec<u8> = vec![0; hex.len() / 2];
    for (i, c) in hex.bytes().enumerate() {
        let byte_value = match c {
            b'0'..=b'9' => c - b'0',
            b'a'..=b'f' => c - b'a' + 10,
            b'A'..=b'F' => c - b'A' + 10,
            _ => return Err(ConversionError::InvalidCharacter),
        };
        hex_in_bytes[i / 2] |= byte_value << (4 * ((!i) & 1));
    }

    Ok(hex_in_bytes)
}

pub fn bytes_to_hex(bytes: &[u8]) -> String {
    bytes
        .into_iter()
        .flat_map(|&val| [
                in_hex(U4::from(val >> 4)),
                in_hex(U4::from(val)),
        ])
        .collect()
}
