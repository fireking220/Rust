use std::collections::HashMap;

pub fn check(candidate: &str) -> bool {
    let mut m = candidate.trim().to_lowercase();
    m = m.chars().filter(|x| x.is_alphabetic()).collect();
    let mut hash: HashMap<char, u64> = HashMap::new();
    for c in m.chars() {
        if c == ' ' {
            continue;
        } else if hash.contains_key(&c) {
            return false;
        }
        hash.insert(c, 1);
    }
    true
}
