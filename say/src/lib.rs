#![allow(unused)]
use std::collections::HashMap;

pub fn encode(n: u64) -> String {
    let mut m: u64 = n;
    if m == 0 {
        "zero".to_string()
    } else if m >e 0 && m <= u64::max_value() {
        let mut num_list: Vec<u64> = Vec::new();
        let mut ions_hash: HashMap<u64, &str> = HashMap::new();
        let mut s = String::new();
        ions_hash.insert(6, "quintillion");
        ions_hash.insert(5, "quadrillion");
        ions_hash.insert(4, "trillion");
        ions_hash.insert(3, "billion");
        ions_hash.insert(2, "million");
        ions_hash.insert(1, "thousand");
        ions_hash.insert(0, "");

        while m != 0 {
            num_list.push(m % 1000);
            m /= 1000;
        }

        for (y, x) in num_list.iter().enumerate().rev() {
            let hundered = x / 100;
            let tens = x % 100;
            if hundered == 0 && *x != 0 {
                s += &format!("{} ", to_eng(tens));
                s += &format!("{} ", ions_hash[&(y as u64)]);
            } else if *x != 0 {
                s += &format!("{} hundred ", to_eng(hundered));
                if tens != 0 {
                    s += &format!("{} ", to_eng(tens));
                }
                s += &format!("{} ", ions_hash[&(y as u64)]);
            }
        }
        s.trim().to_string()
    } else {
        "won't compile".to_string()
    }
}

pub fn to_eng(num: u64) -> String {
    match num {
        1 => "one".to_owned(),
        2 => "two".to_owned(),
        3 => "three".to_owned(),
        4 => "four".to_owned(),
        5 => "five".to_owned(),
        6 => "six".to_owned(),
        7 => "seven".to_owned(),
        8 => "eight".to_owned(),
        9 => "nine".to_owned(),
        10 => "ten".to_owned(),
        11 => "eleven".to_owned(),
        12 => "twelve".to_owned(),
        13 => "thirteen".to_owned(),
        14 => "fourteen".to_owned(),
        15 => "fifteen".to_owned(),
        16 => "sixteen".to_owned(),
        17 => "seventeen".to_owned(),
        18 => "eighteen".to_owned(),
        19 => "nineteen".to_owned(),
        _ => {
            if num == 20 {
                "twenty".to_owned()
            } else if num > 20 && num < 30 {
                "twenty-".to_owned() + &to_eng(num % 10)
            } else if num == 30 {
                "thirty".to_owned()
            } else if num > 30 && num < 40 {
                "thirty-".to_owned() + &to_eng(num % 10)
            } else if num == 40 {
                "forty".to_owned()
            } else if num > 40 && num < 50 {
                "forty-".to_owned() + &to_eng(num % 10)
            } else if num == 50 {
                "fifty".to_owned()
            } else if num > 50 && num < 60 {
                "fifty-".to_owned() + &to_eng(num % 10)
            } else if num == 60 {
                "sixty".to_owned()
            } else if num > 60 && num < 70 {
                "sixty-".to_owned() + &to_eng(num % 10)
            } else if num == 70 {
                "seventy".to_owned()
            } else if num > 70 && num < 80 {
                "seventy-".to_owned() + &to_eng(num % 10)
            } else if num == 80 {
                "eighty".to_owned()
            } else if num > 80 && num < 90 {
                "eighty-".to_owned() + &to_eng(num % 10)
            } else if num == 90 {
                "ninety".to_owned()
            } else {
                "ninety-".to_owned() + &to_eng(num % 10)
            }
        }
    }
}
