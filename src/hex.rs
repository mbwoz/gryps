use crate::err::ConversionError;
use crate::types::U4;

const HEX_VALUES: [u8; 16] = [b'0', b'1', b'2', b'3', b'4', b'5', b'6', b'7', b'8', b'9', b'a', b'b', b'c', b'd', b'e', b'f'];

fn in_hex(val: U4) -> u8 {
    HEX_VALUES[val.to_u8() as usize]
}

pub fn from_hex(hex: &str) -> Result<Vec<u8>, ConversionError> {
    if hex.len() % 2 == 1 {
        return Err(ConversionError::InvalidLength);
    }

    let mut hex_in_bytes: Vec<u8> = vec![0; hex.len() / 2];
    for (i, c) in hex.bytes().enumerate() {
        let byte_value = match c {
            b'0'..=b'9' => c - b'0',
            b'A'..=b'F' => c - b'A' + 10,
            b'a'..=b'f' => c - b'a' + 10,
            _ => return Err(ConversionError::InvalidCharacter),
        };
        hex_in_bytes[i / 2] |= byte_value << (4 * ((!i) & 1));
    }

    Ok(hex_in_bytes)
}

pub fn to_hex(bytes: &[u8]) -> String {
    let encoded = bytes
        .iter()
        .flat_map(|&val| [
                in_hex(U4::from(val >> 4)),
                in_hex(U4::from(val)),
        ])
        .collect();
    String::from_utf8(encoded).expect("Conversion of any byte array to hex should be valid.")
}
