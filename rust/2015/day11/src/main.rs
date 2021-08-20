use std::str;

use itertools::Itertools;

const PASSWORDS_NEEDED: usize = 2;

fn main() {
    let mut count = 0;
    let mut pwd: [u8; 8] = *b"hxbxwxba";
    next_pwd(pwd.as_mut(), 0, &mut |buf| {
        if password_ok(&buf) {
            let s = str::from_utf8(buf);
            println!("{} ", s.unwrap());
            count += 1;
            count < PASSWORDS_NEEDED
        } else {
            true
        }
    });
}

fn password_ok(pwd: &[u8]) -> bool {
    has_three(pwd) && !has_invalid_chars(pwd) && count_pairs(pwd) >= 2
}

fn has_three(pwd: &[u8]) -> bool {
    for (a, b, c) in pwd.iter().tuple_windows() {
        if (b > a && b - a == 1) && (c > b && c - b == 1) {
            return true;
        }
    }

    false
}

fn has_invalid_chars(pwd: &[u8]) -> bool {
    pwd.iter().any(|&c| c == b'i' || c == b'o' || c == b'l')
}

fn count_pairs(pwd: &[u8]) -> u32 {
    let mut count = 0;
    let mut skip = false;
    let mut found_pairs = vec![];
    for i in 1..pwd.len() {
        if skip {
            skip = false;
            continue;
        }
        if pwd[i] == pwd[i - 1] && found_pairs.iter().find(|c| **c == pwd[i]).is_none() {
            count += 1;
            skip = true;
            found_pairs.push(pwd[i]);
        }
    }

    count
}

fn next_pwd<F>(inp: &mut [u8], index: usize, cb: &mut F) -> bool
where
    F: FnMut(&[u8]) -> bool,
{
    if index == inp.len() - 1 {
        if !cb(inp) {
            return false;
        }
        let mut n = next_char(inp[index]);
        while let Some(ch) = n {
            inp[index] = ch;
            if !cb(inp) {
                return false;
            }
            n = next_char(inp[index]);
        }
    } else {
        loop {
            if !next_pwd(inp, index + 1, cb) {
                return false;
            }
            let ch = next_char(inp[index]);
            if ch.is_none() {
                break;
            }
            inp[index] = ch.unwrap();
        }
    }
    inp[index] = b'a';

    true
}

fn next_char(ch: u8) -> Option<u8> {
    if ch == 'z' as u8 {
        None
    } else {
        Some(ch + 1)
    }
}
