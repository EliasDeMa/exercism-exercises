pub fn is_armstrong_number(num: u32) -> bool {
    let mut digits = Vec::new();
    let mut test = num;
    while test > 0 {
        digits.push(test % 10);
        test /= 10;
    }
    let length = digits.len() as u32;
    num == digits.iter().map(|x| x.pow(length)).sum()
}
