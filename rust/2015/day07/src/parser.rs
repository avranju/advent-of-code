use crate::types::{Instruction, Source, ValueSource};
use pest::{iterators::Pairs, Parser, RuleType};
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "grammar.pest"]
struct InputParser;

pub fn parse(input: &str) -> Instruction {
    let mut instruction = InputParser::parse(Rule::instruction, input).unwrap();

    let src = instruction.next().unwrap();
    let dest = instruction.next().unwrap();

    let src = match src.as_rule() {
        Rule::and_expr => parse_binary_instr(src.into_inner(), Source::And),
        Rule::or_expr => parse_binary_instr(src.into_inner(), Source::Or),
        Rule::lshift_expr => parse_binary_instr(src.into_inner(), Source::LeftShift),
        Rule::rshift_expr => parse_binary_instr(src.into_inner(), Source::RightShift),
        Rule::not_expr => Source::Not(src.into_inner().as_str().parse().unwrap()),
        Rule::value_expr => Source::Value(src.into_inner().as_str().parse().unwrap()),
        _ => unreachable!(),
    };

    Instruction::new(src, dest.as_str().to_owned())
}

fn parse_binary_instr<'i, R, F>(mut pairs: Pairs<'i, R>, make_source: F) -> Source
where
    R: RuleType,
    F: Fn(ValueSource, ValueSource) -> Source,
{
    // 'pairs' MUST have 2 values for a binary instruction
    let t1 = pairs.next().unwrap().as_str();
    let t2 = pairs.next().unwrap().as_str();

    make_source(t1.parse().unwrap(), t2.parse().unwrap())
}
