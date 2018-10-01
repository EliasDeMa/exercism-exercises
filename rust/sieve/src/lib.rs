pub fn primes_up_to(upper_bound: u64) -> Vec<u64> {
    
    let mut is_prime = vec![true; upper_bound as usize + 1];
    is_prime[0] = false;
    if upper_bound >= 1 { is_prime[1] = false }
 
    for num in 2..=upper_bound {
        if is_prime[num as usize] {
            let mut multiple = num*num;
            while multiple <= upper_bound {
                is_prime[multiple as usize] = false;
                multiple += num;
            }
        }
    }
 
    is_prime.iter().enumerate()
        .filter_map(|(pr, &is_pr)| if is_pr {Some(pr as u64)} else {None} )
        .collect()
}
