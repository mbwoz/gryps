pub fn xor_single(input: &[u8], byte: u8) -> Vec<u8> {
    input.iter()
        .map(|&x| x^byte)
        .collect()
}

pub fn xor_sequence(input: &[u8], sequence: &[u8]) -> Vec<u8> {
    input.iter()
        .zip(sequence.iter().cycle())
        .map(|(&x, &y)| x^y)
        .collect()
}

pub fn xor_fixed(input: &[u8], sequence: &[u8]) -> Vec<u8> {
    assert_eq!(input.len(), sequence.len());
    xor_sequence(input, sequence)
}

fn get_englishness(input: &[u8]) -> f64 {
    const ENGLISH_FREQUENCIES: [f64; 26] = [
        8.55, 1.60, 3.16, 3.87, 12.10, 2.18, 2.09, 4.96, 7.33, 0.22, 0.81, 4.21, 2.53, 7.17, 7.47, 2.07, 0.10, 6.33, 6.73, 8.94, 2.68, 1.06, 1.83, 0.19, 1.72, 0.11
    ];

    let mut occurrences: [u64; 26] = [0; 26];
    for &byte in input {
        match byte {
            b'a'..=b'z' => occurrences[(byte - b'a') as usize] += 1,
            b'A'..=b'Z' => occurrences[(byte - b'A') as usize] += 1,
            _ => continue,
        }
    }

    occurrences
        .map(|cnt| (cnt as f64 * 100.0 / input.len() as f64))
        .iter()
        .zip(ENGLISH_FREQUENCIES)
        .map(|(a, b)| (a - b).abs())
        .sum()
}

pub fn find_xor_single(input: &[u8]) -> u8 {
    let mut result: (f64, u8) = (get_englishness(input), 0);
    for byte in 1..=255 {
        let decoded = xor_single(input, byte);
        let englishness = get_englishness(decoded.as_ref());
        if englishness < result.0 {
            result = (englishness, byte);
        }
    }
    result.1
}

#[cfg(test)]
mod tests {
    use std::fs;

    use crate::hex::{to_hex, from_hex};
    use super::*;

    #[test]
    fn crypto_problem_2() {
        let a = "1c0111001f010100061a024b53535009181c";
        let b = "686974207468652062756c6c277320657965";
        let expected = "746865206b696420646f6e277420706c6179";

        let x = from_hex(a).unwrap();
        let y = from_hex(b).unwrap();
        let result = to_hex(xor_fixed(x.as_ref(), y.as_ref()).as_ref());

        assert_eq!(result, expected);
    }

    #[test]
    fn crypto_problem_3() {
        let input = "1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736";

        let input_bytes = from_hex(input).unwrap();
        let byte = find_xor_single(input_bytes.as_ref());
        let decoded = String::from_utf8(xor_single(input_bytes.as_ref(), byte)).unwrap();

        println!("byte: 0x{:x?}, output: {:?}", byte, decoded);
    }

    #[test]
    fn crypto_problem_4() {
        let input = fs::read_to_string("resources/4.txt").unwrap();

        for line in input.lines() {
            let line_bytes = from_hex(line).unwrap();
            let byte = find_xor_single(line_bytes.as_ref());
            let decoded = String::from_utf8(xor_single(line_bytes.as_ref(), byte)).ok();
            if let Some(text) = decoded {
                println!("byte: 0x{:x?}, input: {}, output: {:?}", byte, line, text);
            }
        }
    }

    #[test]
    fn crypto_problem_5() {
        let input = 
            "Burning 'em, if you ain't quick and nimble\nI go crazy when I hear a cymbal";
        let seq = "ICE";
        let expected = 
            "0b3637272a2b2e63622c2e69692a23693a2a3c6324202d623d63343c2a26226324272765272a282b2f20430a652e2c652a3124333a653e2b2027630c692b20283165286326302e27282f";

        let result = xor_sequence(input.as_bytes(), seq.as_bytes());
        let hex_result = to_hex(result.as_ref());

        assert_eq!(expected, hex_result);
    }
}
