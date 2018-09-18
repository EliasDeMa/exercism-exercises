/// Determines whether the supplied string is a valid ISBN number
pub fn is_valid_isbn(isbn: &str) -> bool {
    let isbn = isbn.replace("-", "");
    match isbn.ends_with("X") || (0..=9).any(|s| isbn.ends_with(&s.to_string())) {
        true => {
            let x = isbn.ends_with("X");
            let isbn = isbn.replace("X", "1");
            match isbn.parse::<u32>() {
                Ok(num) => {
                    return check_value(num, x);
                },
                Err(_) => return false,
            };
        },
        false => false,
    }
}

fn check_value(isbn: u32, has_x: bool) -> bool {
    if isbn.to_string().len() != 10 {
        return false;
    }
    let mut digits: Vec<u32> = isbn.to_string()
                             .chars()
                             .map(|d| d.to_digit(10).unwrap())
                             .collect();
    if has_x {
        digits[9] = 10;
    }
    let result = (1..=10).fold(0, |acc, s| acc + (s * digits[(s - 1) as usize])) % 11;
    println!("{}", result);
    result == 0
}