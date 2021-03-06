pub fn collatz(n: u64) -> Option<u64> {
    let mut n = n;
    match n {
        0 => None,
        _ => {
            let mut counter = 0;
            while n != 1 {
                match n & 1 {
                    0 => n /= 2,
                    _ => n = 3 * n + 1,
                }
                counter += 1;
            };
            Some(counter)
        },
    }
}

