/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    let code = code.replace(" ", "");
    let mut replaced: Vec<u32> = vec![];

    if code.len() == 1 {
        return false;
    } else if !code.chars().all(|s| s.is_numeric()) {
        return false;
    }
    
    for (i, c) in code.chars().rev().enumerate() {
        if i & 1 == 0 {
            replaced.push(c.to_string().parse::<u32>().unwrap());
        } else {
            let mut number = c.to_string().parse::<u32>().unwrap();
            number *= 2;

            if number >= 10 {
                number -= 9;
            }
            replaced.push(number);
        }
    }
    let number = replaced.iter().fold(0, |acc, x| acc + x);
    println!("{}", number);
    number % 10 == 0
}
