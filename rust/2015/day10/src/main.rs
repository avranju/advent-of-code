fn main() {
    let mut inp = "1113122113".to_string();
    for i in 0..50 {
        println!("{} - {}", i, inp.len());
        let mut cur = inp.chars().nth(0).unwrap();
        let mut count = 1;
        let mut outp = String::new();
        for ch in inp.chars().skip(1) {
            if ch == cur {
                count += 1;
            } else {
                outp.push(char::from_digit(count, 10).unwrap());
                outp.push(cur);
                cur = ch;
                count = 1;
            }
        }

        outp.push(char::from_digit(count, 10).unwrap());
        outp.push(cur);
        inp = outp.clone();
        outp.clear();
    }

    println!("{}", inp.len());
}
