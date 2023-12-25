use std::{
    cmp, env,
    io::{self, BufRead},
    str::FromStr,
};

use anyhow::{anyhow, Error, Result};

#[derive(Debug)]
enum DrawColour {
    Blue,
    Red,
    Green,
}

impl FromStr for DrawColour {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "blue" => Ok(DrawColour::Blue),
            "red" => Ok(DrawColour::Red),
            "green" => Ok(DrawColour::Green),
            _ => Err(anyhow!("Couldn't parse draw colour")),
        }
    }
}

#[derive(Debug)]
struct Draw {
    colour: DrawColour,
    count: usize,
}

#[derive(Debug)]
struct Game {
    id: usize,
    draws: Vec<Vec<Draw>>,
}

impl FromStr for Game {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut tokens = s.split(": ");
        let id = parse_game_id(tokens.next().ok_or(anyhow!("no game id"))?)?;
        let draws = parse_draw_set(tokens.next().ok_or(anyhow!("no draw set"))?)?;

        Ok(Game { id, draws })
    }
}

fn parse_game_id(s: &str) -> Result<usize, Error> {
    Ok(s.split(' ')
        .nth(1)
        .map(|v| v.parse::<usize>())
        .ok_or(anyhow!("Couldn't parse game ID"))??)
}

fn parse_draw_set(s: &str) -> Result<Vec<Vec<Draw>>, Error> {
    s.split("; ")
        .map(parse_draws)
        .collect::<Result<Vec<Vec<Draw>>, Error>>()
}

fn parse_draws(s: &str) -> Result<Vec<Draw>, Error> {
    s.split(", ")
        .map(parse_draw)
        .collect::<Result<Vec<Draw>, Error>>()
}

fn parse_draw(s: &str) -> Result<Draw, Error> {
    let mut tokens = s.split(' ');
    let count = tokens
        .next()
        .map(|v| v.parse::<usize>())
        .ok_or(anyhow!("Couldn't parse draw colour count"))??;
    let colour = tokens
        .next()
        .ok_or(anyhow!("Couldn't parse draw colour"))?
        .parse()?;

    Ok(Draw { colour, count })
}

type Part = fn(String) -> Result<usize>;

fn main() -> Result<()> {
    let part: Part = env::args()
        .nth(1)
        .and_then(|p| match p.as_str() {
            "part1" => Some::<Part>(part1),
            "part2" => Some::<Part>(part2),
            _ => None,
        })
        .unwrap_or_else(|| {
            usage();
            std::process::exit(1);
        });

    let stdin = io::stdin();

    let val = stdin
        .lock()
        .lines()
        .map(|l| l.unwrap())
        .filter(|l| !l.is_empty())
        .map(part)
        .collect::<Result<Vec<usize>>>()?
        .iter()
        .sum::<usize>();

    println!("{}", val);

    Ok(())
}

fn part1(line: String) -> Result<usize> {
    const NUM_REDS: usize = 12;
    const NUM_GREENS: usize = 13;
    const NUM_BLUES: usize = 14;

    let game = line.parse::<Game>()?;

    for draw in game.draws.iter() {
        let mut reds = 0;
        let mut greens = 0;
        let mut blues = 0;

        for d in draw.iter() {
            match d.colour {
                DrawColour::Red => reds += d.count,
                DrawColour::Green => greens += d.count,
                DrawColour::Blue => blues += d.count,
            }
        }

        if reds > NUM_REDS || greens > NUM_GREENS || blues > NUM_BLUES {
            return Ok(0);
        }
    }

    Ok(game.id)
}

fn part2(line: String) -> Result<usize> {
    let game = line.parse::<Game>()?;

    let mut reds = 0;
    let mut greens = 0;
    let mut blues = 0;

    for draw in game.draws.iter() {
        for d in draw.iter() {
            match d.colour {
                DrawColour::Red => reds = cmp::max(d.count, reds),
                DrawColour::Green => greens = cmp::max(d.count, greens),
                DrawColour::Blue => blues = cmp::max(d.count, blues),
            }
        }
    }

    Ok(reds * greens * blues)
}

fn usage() {
    println!("Usage: {} <part1|part2>", env!("CARGO_PKG_NAME"));
}
