pub fn factors(n: u64) -> Vec<u64> {
    let mut n = n;
    let mut answer: Vec<u64> = vec![];
    let mut next_prime: u64 = 2;
    while n != 1 {
        if check_prime(next_prime){
            if n % next_prime == 0 {
                answer.push(next_prime);
                n /= next_prime;
            } else {
                next_prime += 1;
            }
        } else {
            next_prime += 1;
        }    
    }
    answer
}

fn check_prime(n: u64) -> bool {
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