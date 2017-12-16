extern crate md5;

use std::ops::Range;

struct HashInput<'a> {
    input: String,
    range: Range<u32>,
    zero_string: &'a str,
}

fn main() {
    let zero_string = "000000";

    // read the key string
    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("failed to read line");
    let hinput = HashInput {
        input,
        range: 1..80000000,
        zero_string,
    };

    match search(hinput) {
        Some((salt, hash)) => println!("\n{} {:x}", salt, hash),
        None => println!("No dice"),
    }
}

fn search(hash_input: HashInput) -> Option<(u32, md5::Digest)> {
    let zero_string_len = hash_input.zero_string.len();
    for salt in hash_input.range {
        let hash = md5::compute(format!("{}{}", hash_input.input, salt));
        if &format!("{:x}", hash)[..zero_string_len] == hash_input.zero_string {
            return Some((salt, hash));
        }
    }

    None
}
