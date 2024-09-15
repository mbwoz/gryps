pub fn hamming_distance(a: &[u8], b: &[u8]) -> u32 {
    assert_eq!(a.len(), b.len());

    a.iter()
        .zip(b.iter())
        .map(|(&x, &y)| (x^y).count_ones())
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn crypto_problem_6a() {
        let a = "this is a test";
        let b = "wokka wokka!!!";

        assert_eq!(hamming_distance(a.as_bytes(), b.as_bytes()), 37);
    }
}
