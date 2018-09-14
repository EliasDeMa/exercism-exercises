pub fn nth(n: u32) -> Option<u32> {
    let mut m = 0;
    let mut i = 1;
    if n < 1 { return None; }
    while m < n {
        i += 1;
        if check_prime(i) {m += 1};
    }
    Some(i)
}

fn check_prime(n: u32) -> bool {
    match n {
        _ if n <= 1 => false,
        _ if n <= 3 => true,
        _ if n % 2 == 0 => false,
        _ if n % 3 == 0 => false,
        _ => {
            let mut i = 5;
            while i * i <= n {
                if n % i == 0 || n % (i + 2) == 0 {
                    return false;
                }
                i = i + 6
            }
            true
        }
    }
}