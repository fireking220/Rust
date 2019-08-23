pub fn reply(message: &str) -> &str {
    let m = message.trim();
    if m.is_empty() {
        return "Fine. Be that way!";
    }
    let p = m.to_string().pop().unwrap();
    let m: String = m.chars().filter(|x| x.is_alphabetic()).collect();
    if p == '?' {
        if m.to_uppercase() == m && !m.is_empty() {
            "Calm down, I know what I'm doing!"
        } else {
            "Sure."
        }
    } else if m.to_uppercase() == m && !m.is_empty() {
        "Whoa, chill out!"
    } else {
        "Whatever."
    }
}
