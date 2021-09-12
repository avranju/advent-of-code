use std::{
    collections::BTreeMap,
    io::{self, BufRead},
};

use regex::Regex;

#[derive(Debug)]
struct Aunt {
    num: u32,
    props: BTreeMap<String, u32>,
}

enum MatchType {
    Equals(u32),
    GreaterThan(u32),
    LesserThan(u32),
}

struct Rule {
    prop_name: String,
    match_type: MatchType,
}

impl Rule {
    fn new(prop_name: &str, match_type: MatchType) -> Self {
        Rule {
            prop_name: prop_name.to_string(),
            match_type,
        }
    }

    fn matches(&self, val: u32) -> bool {
        match self.match_type {
            MatchType::Equals(v) => val == v,
            MatchType::GreaterThan(v) => val > v,
            MatchType::LesserThan(v) => val < v,
        }
    }
}

fn main() {
    // Sue 1: children: 1, cars: 8, vizslas: 7
    let re = Regex::new("Sue ([0-9]+): (.*)").unwrap();
    let stdin = io::stdin();

    let mut aunts = vec![];

    for line in stdin
        .lock()
        .lines()
        .map(|l| l.unwrap())
        .filter(|l| !l.is_empty())
    {
        for cap in re.captures_iter(&line) {
            let num = u32::from_str_radix(&cap[1], 10).unwrap();
            let props_str = &cap[2];

            let mut props = BTreeMap::new();
            for tok1 in props_str.split(", ") {
                let toks = tok1.split(": ").collect::<Vec<_>>();
                props.insert(
                    toks[0].to_string(),
                    u32::from_str_radix(toks[1], 10).unwrap(),
                );
            }
            aunts.push(Aunt { num, props });
        }
    }

    let rules = vec![
        Rule::new("children", MatchType::Equals(3)),
        Rule::new("cats", MatchType::GreaterThan(7)),
        Rule::new("samoyeds", MatchType::Equals(2)),
        Rule::new("pomeranians", MatchType::LesserThan(3)),
        Rule::new("akitas", MatchType::Equals(0)),
        Rule::new("vizslas", MatchType::Equals(0)),
        Rule::new("goldfish", MatchType::LesserThan(5)),
        Rule::new("trees", MatchType::GreaterThan(3)),
        Rule::new("cars", MatchType::Equals(2)),
        Rule::new("perfumes", MatchType::Equals(1)),
    ];

    // for every aunt, if the aunt has a prop matching one of the props in
    // the rules then apply the rule to that aunt and exclude her if the rule
    // doesn't match
    let matching_aunts = aunts
        .iter()
        .filter(|aunt| {
            !rules.iter().any(|rule| {
                aunt.props.contains_key(&rule.prop_name)
                    && !rule.matches(aunt.props[&rule.prop_name])
            })
        })
        .map(|aunt| aunt.num);

    let ma = matching_aunts.collect::<Vec<_>>();
    println!("{:#?}", ma);
}
