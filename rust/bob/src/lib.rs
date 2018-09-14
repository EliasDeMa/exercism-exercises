pub fn reply(message: &str) -> &str {
    let message = message.trim();
    if message.is_empty() {
        "Fine. Be that way!"
    } else if message.chars().last() == Some('?') {
        if message.chars().all(|c| !c.is_alphabetic()) {
            "Sure." 
        } else {
            if message.to_uppercase() == message {
                "Calm down, I know what I'm doing!"
            } else {
                "Sure." 
            }          
        }
    } else {
        if message.chars().all(|c| !c.is_alphabetic()) {
            "Whatever." 
        } else {
            if message.to_uppercase() == message {
                "Whoa, chill out!"
            } else {
                "Whatever." 
            }          
        }
    }
}
