use std::io::{self, BufRead};

use regex::Regex;

const TRAVEL_SECS: usize = 2503;

#[derive(Debug)]
struct Reindeer {
    name: String,
    speed: usize,
    flight_duration: usize,
    rest_duration: usize,
    travel_distance: usize,
    is_resting: bool,
    points: usize,
    rest_start_time: usize,
    flight_start_time: usize,
}

fn main() {
    let re = Regex::new("([^ ]+) can fly ([0-9]+) km/s for ([0-9]+) seconds, but then must rest for ([0-9]+) seconds.")
            .unwrap();

    let stdin = io::stdin();
    let mut deers = vec![];

    for line in stdin
        .lock()
        .lines()
        .map(|l| l.unwrap())
        .filter(|l| !l.is_empty())
    {
        for cap in re.captures_iter(&line) {
            let deer = Reindeer {
                name: cap[1].to_string(),
                speed: usize::from_str_radix(&cap[2], 10).unwrap(),
                flight_duration: usize::from_str_radix(&cap[3], 10).unwrap(),
                rest_duration: usize::from_str_radix(&cap[4], 10).unwrap(),
                travel_distance: 0,
                is_resting: false,
                points: 0,
                rest_start_time: 0,
                flight_start_time: 0,
            };
            deers.push(deer);
        }
    }

    for i in 1..=TRAVEL_SECS {
        for deer in deers.iter_mut() {
            if !deer.is_resting {
                deer.travel_distance += deer.speed;

                if i - deer.flight_start_time == deer.flight_duration {
                    deer.is_resting = true;
                    deer.rest_start_time = i;
                }
            }

            if deer.is_resting {
                if i - deer.rest_start_time == deer.rest_duration {
                    deer.is_resting = false;
                    deer.flight_start_time = i;
                }
            }
        }

        let max_distance = deers
            .iter()
            .max_by(|a, b| a.travel_distance.cmp(&b.travel_distance))
            .map(|d| d.travel_distance)
            .unwrap();
        for deer in deers
            .iter_mut()
            .filter(|d| d.travel_distance == max_distance)
        {
            deer.points += 1;
        }
    }

    let points_winner = deers.iter().max_by(|a, b| a.points.cmp(&b.points)).unwrap();
    println!(
        "\n---\n  Winning deer by points = {} - traveled: {} - points: {}",
        points_winner.name, points_winner.travel_distance, points_winner.points
    );

    let distance_winner = deers
        .iter()
        .max_by(|a, b| a.travel_distance.cmp(&b.travel_distance))
        .unwrap();
    println!(
        "  Winning deer by distance = {} - traveled: {} - points: {}",
        distance_winner.name, distance_winner.travel_distance, distance_winner.points
    );
}
