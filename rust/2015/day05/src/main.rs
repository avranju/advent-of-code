use std::collections::BTreeMap;
use std::io::{self, BufRead};

fn main() {
    let mut nice_strings = Vec::new();
    let stdin = io::stdin();
    // for line in ["qjhvhtzxzqqjkmpb"].iter().map(|s| String::from(*s)) {
    for line in stdin.lock().lines().map(|l| l.unwrap()) {
        if line.len() > 0 {
            if is_nice2(&line) {
                nice_strings.push(line);
            }
        }
    }
    println!("Number of nice strings: {}", nice_strings.len());
}

fn is_nice2(line: &String) -> bool {
    two_pairs_count(&line) > 0 && fenced_letter_count(&line) > 0
}

fn two_pairs_count(input: &str) -> usize {
    let mut two_pairs = BTreeMap::new();
    let mut sinp = input;
    while sinp.len() > 2 {
        let (s1, s2) = sinp.split_at(2);
        if s2.contains(s1) {
            let count = two_pairs.entry(s1).or_insert(0);
            *count += 1;
        }
        sinp = sinp.split_at(1).1;
    }
    two_pairs.len()
}

fn fenced_letter_count(input: &str) -> usize {
    let sinp: Vec<char> = input.chars().collect();
    let mut count = 0;
    let mut index = 0;
    while index + 2 < sinp.len() {
        if sinp[index] == sinp[index + 2] {
            count += 1
        }
        index += 1
    }
    count
}

#[test]
fn test_fenced_letter_count() {
    assert_eq!(fenced_letter_count("xyx"), 1);
    assert_eq!(fenced_letter_count("abcdefeghi"), 1);
    assert_eq!(fenced_letter_count("aaa"), 1);
    assert_eq!(fenced_letter_count("xyyxaax"), 0);
}

#[test]
fn test_two_pairs_count() {
    assert_eq!(two_pairs_count("xyxy"), 1);
    assert_eq!(two_pairs_count("aabcdefgaa"), 1);
    assert_eq!(two_pairs_count("aaa"), 0);
    assert_eq!(two_pairs_count("aaaa"), 1);
    assert_eq!(two_pairs_count("aaaaa"), 1);
    assert_eq!(two_pairs_count("aaaaaa"), 1);
    assert_eq!(two_pairs_count("abcdabzyxwvzyab"), 2);
}

#[allow(dead_code)]
fn is_nice1(line: &String) -> bool {
    let (vowels_count, doubles_count, forbidden) = check_line1(&line);
    !forbidden && vowels_count > 2 && doubles_count > 0
}

fn check_line1(line: &String) -> (i32, i32, bool) {
    let mut vowels_count = 0;
    let mut doubles_count = 0;
    let mut forbidden = false;
    let mut chars = line.chars();
    let mut prev_char = chars.next().unwrap();
    if is_vowel(prev_char) {
        vowels_count += 1;
    }
    for ch in chars {
        if is_vowel(ch) {
            vowels_count += 1;
        }
        if ch == prev_char {
            doubles_count += 1;
        }
        if is_forbidden(&format!("{}{}", prev_char, ch)) {
            forbidden = true;
            break;
        }
        prev_char = ch;
    }

    (vowels_count, doubles_count, forbidden)
}

fn is_vowel(ch: char) -> bool {
    let vowels = "aeiou";
    for c in vowels.chars() {
        if ch == c {
            return true;
        }
    }
    false
}

fn is_forbidden(inp: &str) -> bool {
    let forbidden_strings = ["ab", "cd", "pq", "xy"];
    for s in forbidden_strings.iter() {
        if *s == inp {
            return true;
        }
    }
    false
}
