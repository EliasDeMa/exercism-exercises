use std::iter;

pub fn find() -> Option<u32> {
    let sum = 1000;
    (2..sum)
        .flat_map(|c| {
            let a = sum - c;
            (1..a).zip(iter::repeat(c))
        }).map(|(a, c)| (a, sum - c - a, c))
        .find(|&(a, b, c)| a * a + b * b == c * c)
        .map(|(a, b, c)| a * b * c)
}
