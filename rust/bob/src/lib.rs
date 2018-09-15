pub fn reply(message: &str) -> &str {
    let message = message.trim();
    let no_alphabet = message.chars().all(|c| !c.is_alphabetic());

    match message {
        _ if message.is_empty() => "Fine. Be that way!",
        _ if message.chars().last() == Some('?') => {
            match message {
                _ if message.to_uppercase() == message && !no_alphabet => "Calm down, I know what I'm doing!",
                _ => "Sure.",
            }
        
        },
        _ => {
            match message {
                _ if message.to_uppercase() == message && !no_alphabet => "Whoa, chill out!",
                _ => "Whatever.",
            }
        }
    }
}
