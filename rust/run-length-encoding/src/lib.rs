pub fn encode(source: &str) -> String {
    match source.len() {
        0 => source.into(),
        _ => {
            let mut encoded = String::new();
            let mut count = 1;
            let mut current = source.chars().nth(0).unwrap();

            for c in source.chars().skip(1) {
                if c == current {
                    count += 1;
                } else {
                    encoded.push_str(&match_number(count));
                    encoded.push(current);
                    current = c;
                    count = 1;
                }
            }
            encoded.push_str(&match_number(count));
            encoded.push(current);
            
            encoded
        }
       
    }
}

pub fn decode(source: &str) -> String {
    let mut amount = String::new();
    let mut decoded = String::new();

    for c in source.chars() {
        if c.is_numeric() {
            amount.push(c)
        } else {
            let num: usize = amount.parse().unwrap_or(1);
            decoded.push_str(&c.to_string().repeat(num));
            amount.clear();
        }
    }

    decoded
}

fn match_number(count: u32) -> String {
    match count {
        1 => "".into(),
        _ => count.to_string()
    }
}