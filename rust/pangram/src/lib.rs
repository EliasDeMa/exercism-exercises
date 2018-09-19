/// Determine whether a sentence is a pangram.
pub fn is_pangram(sentence: &str) -> bool {
    ('a' as u32..= 'z' as u32)  
        .all(|x| sentence
                    .to_lowercase()
                    .contains(std::char::from_u32(x).unwrap())
            )
}
