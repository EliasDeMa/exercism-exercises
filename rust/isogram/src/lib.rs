pub fn check(candidate: &str) -> bool {
    let candidate = candidate
                        .chars()
                        .filter(|&c| c.is_alphabetic())
                        .collect::<String>()
                        .to_lowercase();
    candidate.chars()
             .all(|char| count(&candidate, char) == 1)

}

fn count(s: &str, m: char) -> usize {
    s.chars().filter(|&c| c == m).count()
}