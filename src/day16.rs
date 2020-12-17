use crate::utils::ParseError;
use itertools::Itertools;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fmt;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::iter::FromIterator;
use std::str::FromStr;

#[allow(dead_code)]
pub fn main() -> std::io::Result<()> {
    let file = File::open("input/16.txt")?;
    let reader = BufReader::new(file);
    let mut lines = reader.lines().map(|s| s.unwrap());

    let mut rules = Vec::new();
    while let Some(s) = lines.next() {
        if s == "" {
            break;
        }
        rules.push(s.parse::<Rule>().unwrap());
    }

    lines.next(); // skip over your ticket:
    let your_ticket: Vec<u64> = lines
        .next()
        .unwrap()
        .split(',')
        .map(|x| x.parse::<u64>().unwrap())
        .collect();

    lines.next(); // skip over empty line
    lines.next(); // skip over nearby_tickets
    let mut nearby_tickets: Vec<Vec<u64>> = Vec::new();
    for line in lines {
        let ticket = line.split(',').map(|x| x.parse::<u64>().unwrap()).collect();
        nearby_tickets.push(ticket);
    }
    dbg!(part_1(&rules, &nearby_tickets));
    let valid_nearby_tickets = nearby_tickets
        .iter()
        .filter(|x| valid_ticket(&rules, x))
        .cloned()
        .collect::<Vec<_>>();
    dbg!(part_2_fun(&rules, &valid_nearby_tickets, &your_ticket));
    Ok(())
}

fn part_1(rules: &[Rule], nearby_tickets: &[Vec<u64>]) -> u64 {
    let mut scanning_error_rate = 0;
    for ticket in nearby_tickets {
        for ticket_value in ticket {
            let mut valid = false;
            for rule in rules {
                if rule.valid(*ticket_value) {
                    valid = true;
                    break;
                }
            }
            if !valid {
                scanning_error_rate += ticket_value;
            }
        }
    }
    scanning_error_rate
}

#[allow(dead_code)]
fn part_2_naive(rules: &[Rule], tickets: &[Vec<u64>]) -> Option<Vec<Rule>> {
    for permutation in permute_valid_2(tickets, rules, 0).unwrap() {
        let mut all_valid = true;
        'outer: for ticket in tickets {
            for (v, rule) in ticket.iter().zip(permutation.iter()) {
                if !rule.valid(*v) {
                    all_valid = false;
                    break 'outer;
                }
            }
        }

        if all_valid {
            return Some(permutation);
        }
    }
    None
}

fn part_2_fun(rules: &[Rule], tickets: &[Vec<u64>], your_ticket: &[u64]) -> u64 {
    // Convert to columns
    let mut cols: Vec<Vec<u64>> = Vec::new();
    for _ in 0..tickets[0].len() {
        cols.push(Vec::new());
    }

    for ticket in tickets {
        for (i, v) in ticket.iter().enumerate() {
            cols[i].push(*v);
        }
    }

    // Calculate rule appearances
    let mut rule_appears_in = cols
        .iter()
        .map(|col| {
            rules
                .iter()
                .filter(|rule| col.iter().find(|&v| !rule.valid(*v)).is_some())
                .collect::<Vec<_>>()
        })
        .enumerate()
        .collect::<Vec<_>>();
    rule_appears_in.sort_unstable_by_key(|(_, v)| v.len());

    // Produce Results
    let mut result: HashMap<String, usize> = HashMap::new();
    let mut seen: HashSet<&Rule> = HashSet::new();
    for (col, rules) in rule_appears_in {
        for rule in rules {
            if !seen.contains(rule) {
                result.insert(rule.name.to_string(), col);
                seen.insert(&rule);
            }
        }
    }

    result
        .into_iter()
        .filter(|(k, _)| k.starts_with("departure"))
        .inspect(|x| println!("{:?}, {}", x, your_ticket[x.1]))
        .map(|(_k, v)| your_ticket[v])
        .product()
}

// TODO: figure out how to write a more elegant permutations
fn permute_valid_2(
    tickets: &[Vec<u64>],
    rules: &[Rule],
    start_index: usize,
) -> Option<Vec<Vec<Rule>>> {
    if rules.is_empty() {
        return Some(Vec::new());
    }
    if rules.len() == 1 {
        for ticket in tickets {
            if !rules[0].valid(ticket[start_index]) {
                return None;
            }
        }
        return Some(vec![vec![rules[0].clone()]]);
    }

    // multi item step
    let mut collected: Vec<Vec<Rule>> = Vec::new();
    for rule in rules {
        for ticket in tickets {
            if !rules[0].valid(ticket[start_index]) {
                break;
            }
        }

        let mut remaining: HashSet<Rule> = HashSet::from_iter(rules.iter().cloned());
        remaining.remove(rule);
        let remaining = remaining.into_iter().collect::<Vec<_>>();

        if let Some(valid_permutations) = permute_valid_2(&tickets, &remaining, start_index + 1) {
            for mut p in valid_permutations {
                let mut arr: Vec<Rule> = vec![rule.clone()];
                arr.append(&mut p);
                collected.push(arr);
            }
        }
    }

    if collected.is_empty() {
        None
    } else {
        Some(collected)
    }
}

fn valid_ticket(rules: &[Rule], ticket: &[u64]) -> bool {
    for ticket_value in ticket {
        let mut valid = false;
        for rule in rules {
            if rule.valid(*ticket_value) {
                valid = true;
                break;
            }
        }
        if !valid {
            return false;
        }
    }
    true
}

#[derive(PartialEq, Debug, Eq, Clone, Hash)]
struct Rule {
    name: String,
    rule1: std::ops::RangeInclusive<u64>,
    rule2: std::ops::RangeInclusive<u64>,
}

impl fmt::Display for Rule {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {:?} or {:?}", self.name, self.rule1, self.rule2)
    }
}

impl Rule {
    fn valid(&self, value: u64) -> bool {
        self.rule1.contains(&value) || self.rule2.contains(&value)
    }
}

impl FromStr for Rule {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let colon = s.find(':').unwrap();
        let name = s[..colon].to_string();
        let mut words = s[(colon + 1)..].split_ascii_whitespace();
        let (rule1, _, rule2) = (
            words.next().ok_or(ParseError::EOF)?,
            words.next().ok_or(ParseError::EOF)?,
            words.next().ok_or(ParseError::EOF)?,
        );

        let parse_range = |x: &str| {
            x.splitn(2, '-')
                .filter_map(|x| x.parse::<u64>().ok())
                .collect_tuple()
                .map(|(a, b)| a..=b)
                .ok_or_else(|| ParseError::InvalidSyntax(x.to_string()))
        };

        let rule1 = parse_range(rule1)?;
        let rule2 = parse_range(rule2)?;

        Ok(Rule { name, rule1, rule2 })
    }
}
