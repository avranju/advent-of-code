use std::io::{self, BufRead};

use pest::Parser;
use pest_derive::Parser;

fn main() {
    let mut raw_count = 0;
    let mut mem_count = 0;
    let mut encode_count = 0;

    let stdin = io::stdin();
    for line in stdin
        .lock()
        .lines()
        .map(|l| l.unwrap())
        .filter(|l| l.len() > 0)
    {
        let tokens = parse(&line).collect::<Vec<CharType>>();

        raw_count += line.len();
        mem_count += tokens.len();
        encode_count += encode_str(tokens.into_iter()).len();
    }

    println!(
        "{} {}",
        raw_count - mem_count,
        encode_count - raw_count
    );
}

fn encode_str(input: impl Iterator<Item = CharType>) -> String {
    let output = input
        .map(|c| match c {
            CharType::Literal(ch) => ch.to_string(),
            CharType::DoubleQuote => r#"\\\""#.to_string(),
            CharType::BackSlash => r#"\\\\"#.to_string(),
            CharType::HexNum(n) => format!(r#"\\x{:02x}"#, n),
        })
        .collect::<String>();

    format!(r#""\"{}\"""#, output)
}

#[derive(Debug)]
enum CharType {
    Literal(char),
    DoubleQuote,
    BackSlash,
    HexNum(u8),
}

#[derive(Parser)]
#[grammar = "grammar.pest"]
struct InputParser;

fn parse(input: &str) -> impl Iterator<Item = CharType> + '_ {
    InputParser::parse(Rule::char, input)
        .unwrap()
        .flat_map(|p| p.into_inner())
        .map(|p| match p.as_rule() {
            Rule::literal => CharType::Literal(p.as_str().chars().nth(0).unwrap()),
            Rule::double_quote => CharType::DoubleQuote,
            Rule::back_slash => CharType::BackSlash,
            Rule::hex_num => CharType::HexNum(u8::from_str_radix(&p.as_str()[2..], 16).unwrap()),
            _ => unreachable!(),
        })
}
