pub fn raindrops(n: usize) -> String {
    let mut answer = String::new();
    if n % 3 == 0 {
        answer += "Pling";
    }
    if n % 5 == 0 {
        answer += "Plang";
    }
    if n % 7 == 0 {
        answer += "Plong";
    }
    match answer.len() {
        0 => n.to_string(),
        _ => answer,
    }
}

