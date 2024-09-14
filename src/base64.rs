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

pub fn to_base64(input: &[u8]) -> String {
    let mut encoded: Vec<char> = Vec::with_capacity(((input.len() + 2) / 3) * 4);
    for chunk in input.chunks(3) {
        let elems = if chunk.len() == 3 {
            chunk
        } else {
            &[
                chunk.get(0).map_or(0, |&x| x),
                chunk.get(1).map_or(0, |&x| x),
                chunk.get(2).map_or(0, |&x| x),
            ]
        };
        let val1 = elems[0] >> 2;
        let val2 = (elems[0] << 4) | (elems[1] >> 4);
        let val3 = (elems[1] << 2) | (elems[2] >> 6);
        let val4 = elems[2];
        encoded.append(&mut vec![
            in_base64(U6::from(val1)),
            in_base64(U6::from(val2)),
            in_base64(U6::from(val3)),
            in_base64(U6::from(val4)),
        ]);
    }

    let encoded_len = encoded.len();
    let substitute_cnt = (3 - (input.len() % 3)) % 3;
    for i in 0..substitute_cnt {
        encoded[encoded_len - i - 1] = '=';
    }

    encoded.into_iter().collect()
}

pub fn from_base64(_input: &str) -> Vec<u8> {
    unimplemented!();
}

#[cfg(test)]
mod tests {
    use crate::hex::from_hex;
    use super::*;

    #[test]
    fn crypto_problem_1() {
        let input = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
        let expected = "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t";
    
        let bytes = from_hex(input).unwrap();
        let result = to_base64(bytes.as_ref());

        assert_eq!(result, expected);
    }
}
