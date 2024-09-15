use crate::{err::ConversionError, types::U6};

const BASE64_VALUES: [u8; 64] = [
    b'A', b'B', b'C', b'D', b'E', b'F', b'G', b'H', b'I', b'J',
    b'K', b'L', b'M', b'N', b'O', b'P', b'Q', b'R', b'S', b'T',
    b'U', b'V', b'W', b'X', b'Y', b'Z', b'a', b'b', b'c', b'd',
    b'e', b'f', b'g', b'h', b'i', b'j', b'k', b'l', b'm', b'n',
    b'o', b'p', b'q', b'r', b's', b't', b'u', b'v', b'w', b'x',
    b'y', b'z', b'0', b'1', b'2', b'3', b'4', b'5', b'6', b'7',
    b'8', b'9', b'+', b'/'
];

fn in_base64(value: U6) -> u8 {
    BASE64_VALUES[value.to_u8() as usize]
}

fn append_full_chunk(encoded: &mut Vec<u8>, chunk: &[u8]) {
    assert!(chunk.len() == 3);

    let values = [
        chunk[0] >> 2,
        chunk[0] << 4 | chunk[1] >> 4,
        chunk[1] << 2 | chunk[2] >> 6,
        chunk[2],
    ];

    for value in values {
        encoded.push(in_base64(U6::from(value)));
    }
}

pub fn to_base64(input: &[u8]) -> String {
    let mut encoded: Vec<u8> = Vec::with_capacity(((input.len() + 2) / 3) * 4);

    let mut chunks_iter = input.chunks_exact(3);
    for chunk in chunks_iter.by_ref() {
        append_full_chunk(&mut encoded, chunk);
    }
    if chunks_iter.remainder().len() > 0 {
        let mut remainder: Vec<u8> = Vec::from(chunks_iter.remainder());
        remainder.resize(3, 0);
        append_full_chunk(&mut encoded, remainder.as_ref());
    }

    let encoded_len = encoded.len();
    let substitute_cnt = (3 - chunks_iter.remainder().len()) % 3;
    for i in 0..substitute_cnt {
        encoded[encoded_len - i - 1] = b'=';
    }

    String::from_utf8(encoded).expect("Conversion of any byte array to base 64 should be valid.")
}

fn out_base64(value: u8) -> Result<u8, ConversionError> {
    match value {
        b'A'..=b'Z' => Ok(value - b'A'),
        b'a'..=b'z' => Ok(value - b'a' + 26),
        b'0'..=b'9' => Ok(value - b'0' + 52),
        b'+' => Ok(62),
        b'/' => Ok(63),
        b'=' => Ok(0),
        _ => Err(ConversionError::InvalidCharacter),
    }
}

pub fn from_base64(input: &str) -> Result<Vec<u8>, ConversionError> {
    if input.len() % 4 != 0 {
        return Err(ConversionError::InvalidLength);
    }

    let padding_len = input.as_bytes().iter().rev().take(2).take_while(|&&b| b == b'=').count();
    for b in input.as_bytes().iter().take(input.len() - padding_len) {
        match b {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'+' | b'/' => continue,
            _ => return Err(ConversionError::InvalidCharacter),
        }
    }

    let mut decoded = Vec::with_capacity(3 * input.len() / 4);
    for chunk in input.as_bytes().chunks_exact(4) {
        let chunk_bytes = chunk
            .iter()
            .try_fold(vec![], |mut acc, &x| -> Result<Vec<u8>, ConversionError> {
                acc.push(out_base64(x)?);
                Ok(acc)
            })?;

        decoded.push(chunk_bytes[0] << 2 | chunk_bytes[1] >> 4);
        decoded.push(chunk_bytes[1] << 4 | chunk_bytes[2] >> 2);
        decoded.push(chunk_bytes[2] << 6 | chunk_bytes[3]);
    }

    Ok(decoded)
}

#[cfg(test)]
mod tests {
    use crate::hex::{from_hex, to_hex};
    use super::*;

    #[test]
    fn crypto_problem_1() {
        let input = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
        let expected = "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t";
    
        let bytes = from_hex(input).unwrap();
        let result = to_base64(bytes.as_ref());

        assert_eq!(result, expected);
    }

    #[test]
    fn crypto_problem_1_rev() {
        let input = "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t";
        let expected = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";

        let bytes = from_base64(input).unwrap();
        let result = to_hex(bytes.as_ref());

        assert_eq!(result, expected);
    }
}
