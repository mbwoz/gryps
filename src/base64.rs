use crate::err::ConversionError;
use crate::bytes;
use crate::types::U6;

const BASE64_VALUES: [char; 64] = [
    'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J',
    'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T',
    'U', 'V', 'W', 'X', 'Y', 'Z', 'a', 'b', 'c', 'd',
    'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n',
    'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x',
    'y', 'z', '0', '1', '2', '3', '4', '5', '6', '7',
    '8', '9', '+', '/'
];

fn in_base64(value: U6) -> char {
    BASE64_VALUES[value.to_u8() as usize]
}

pub fn hex_to_base64(hex: &str) -> Result<String, ConversionError> {
    let mut hex_in_bytes: Vec<u8> = bytes::hex_to_bytes(hex)?;
    hex_in_bytes.append(&mut vec![0, 0]);

    let mut encoded: Vec<char> = Vec::with_capacity(((hex.len() + 5) / 6) * 4);
    for chunk in hex_in_bytes.chunks_exact(3) {
        let val1 = chunk[0] >> 2;
        let val2 = (chunk[0] << 4) | (chunk[1] >> 4);
        let val3 = (chunk[1] << 2) | (chunk[2] >> 6);
        let val4 = chunk[2];
        encoded.append(&mut vec![
            in_base64(U6::from(val1)),
            in_base64(U6::from(val2)),
            in_base64(U6::from(val3)),
            in_base64(U6::from(val4)),
        ]);
    }

    let encoded_len = encoded.len();
    let substitute_cnt = (3 - ((hex.len() / 2) % 3)) % 3;
    for i in 0..substitute_cnt {
        encoded[encoded_len - i - 1] = '=';
    }

    Ok(encoded.into_iter().collect())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn convert_hex_to_base64() {
        let hex = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
        let expected = "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t";
    
        let result = hex_to_base64(hex).unwrap();
        assert_eq!(result, expected);
    }
}
