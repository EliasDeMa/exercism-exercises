pub fn reverse(input: &str) -> String {
    // base case, string was fully traversed
    // return empty string
    if input.len() == 0 {
        return String::from(input);
    } else {
        // recursively call reverse function on string slice
        // to second from last character.
        let character = input.len() - 1;
        return reverse(&input[..character]);
    }
}
