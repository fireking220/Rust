pub fn collatz(n: u64) -> Option<u64> {
    let mut steps: u64 = 0;
    let mut m = n;
    if m == 0 {
        return None;
    }
    loop {
        if m == 1 {
            break;
        }
        steps += 1;
        if m % 2 == 0 {
            m /= 2;
        } else {
            m = 3 * m + 1;
        }
    }
    Some(steps)
}
