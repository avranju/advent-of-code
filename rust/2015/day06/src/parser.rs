use pest::prelude::*;
use types::{Coord, Instruction};

impl_rdp! {
  grammar! {
    expression = _{
      (turn_on | turn_off | toggle) ~ coord_pair
    }

    // positive whole numbers
    number     = @{ ["0"] | ['1'..'9'] ~ ['0'..'9']* }
    comma      = _{ [","] }
    turn       = { ["turn"] }
    toggle     = { ["toggle"] }
    through    = _{ ["through"] }
    on         = { ["on"] }
    off        = { ["off"] }
    turn_on    = { turn ~ on }
    turn_off   = { turn ~ off }
    coord      = { number ~ comma ~ number }
    coord_pair = { coord ~ through ~ coord }

    whitespace = _{ [" "] }
  }
}

pub fn parse(input: &str) -> Result<Instruction, String> {
  // parse the input and see if it passes muster
  let mut parser = Rdp::new(StringInput::new(input));
  if !parser.expression() {
      let (_, pos) = parser.expected();
      let (line_no, col_no) = parser.input().line_col(pos);
      return Err(format!("Invalid input syntax at line {}, column {}", line_no, col_no));
  }

  let tokens = parser.queue();
  if tokens.len() == 0 {
    return Err(format!("Invalid input - no tokens found"));
  }

  match tokens[0].rule {
    Rule::turn_on => parse_instruction(input, tokens, 3, |cp| Instruction::TurnOn(cp)),
    Rule::turn_off => parse_instruction(input, tokens, 3, |cp| Instruction::TurnOff(cp)),
    Rule::toggle => parse_instruction(input, tokens, 1, |cp| Instruction::Toggle(cp)),
    _ => Err(format!("Unexpected token found"))
  }
}

fn parse_instruction<F>(input: &str, tokens: &Vec<Token<Rule>>, start_index: usize, make_instr: F) -> Result<Instruction, String>
    where F: Fn((Coord, Coord)) -> Instruction {

  if tokens.len() <= start_index || tokens[start_index].rule != Rule::coord_pair {
    return Err(format!("Invalid input - expected coord pair"));
  }

  parse_coord_pair(input, tokens, start_index + 1).map(make_instr)
}

fn parse_coord_pair(input: &str, tokens: &Vec<Token<Rule>>, index: usize) -> Result<(Coord, Coord), String> {
  let r1 = parse_coord(input, tokens, index);
  let r2 = parse_coord(input, tokens, index + 3);
  match(r1, r2) {
    (Ok(c1), Ok(c2)) => Ok((c1, c2)),
    _ => Err(format!("Could not parse coord pair"))
  }
}

fn parse_coord(input: &str, tokens: &Vec<Token<Rule>>, index: usize) -> Result<Coord, String> {
  // we are expecting 2 number tokens here
  if tokens.len() <= index + 2 {
    return Err(format!("Invalid input - unexpected end of token stream while parsing coord"));
  }

  if tokens[index].rule != Rule::coord {
    return Err(format!("Invalid input - expected coord"));
  }

  let r1 = parse_number(input, tokens, index + 1);
  let r2 = parse_number(input, tokens, index + 2);
  match(r1, r2) {
    (Ok(n1), Ok(n2)) => Ok(Coord::new(n1, n2)),
    _ => Err(format!("Could not parse coord numbers"))
  }
}

fn parse_number(input: &str, tokens: &Vec<Token<Rule>>, index: usize) -> Result<u32, String> {
  let token = tokens[index];
  input.get(token.start..token.end)
       .ok_or(format!("Invalid input - unexpected end of input when parsing number"))
       .map(|s| s.parse::<u32>().map_err(|_| format!("Could not parse input as number")))
       .and_then(|r| r)
}