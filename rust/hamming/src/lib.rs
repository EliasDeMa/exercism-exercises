/// Return the Hamming distance between the strings,
/// or None if the lengths are mismatched.
pub fn hamming_distance(s1: &str, s2: &str) -> Option<usize> {
    if s1.len() != s2.len() {
        return None;
    } else {
        let result = &s1.chars()
            .enumerate()
            .filter(|(i, s)| s != &s2.chars().nth(*i).unwrap())
            .map(|(_i,s)| s)
            .collect::<String>();
        return Some(result.len());         
    }
}
