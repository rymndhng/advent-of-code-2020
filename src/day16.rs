use crate::utils::ParseError;
use std::collections::HashMap;
use std::collections::HashSet;
use std::iter::FromIterator;
use std::io::BufReader;
use std::fs::File;
use std::str::FromStr;
use std::io::prelude::*;
use std::fmt;
use itertools::Itertools;

#[derive(PartialEq, Debug, Eq, Clone, Hash)]
struct Rule {
    name: String,
    rule1: (u64,u64),
    rule2: (u64,u64),
}

impl fmt::Display for Rule {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}-{} or {}-{}", self.name, self.rule1.0, self.rule1.1, self.rule2.0, self.rule2.1)
    }
}

impl Rule {
    fn valid(&self, value: u64) -> bool {
        (self.rule1.0 ..= self.rule1.1).contains(&value)
            || (self.rule2.0 ..= self.rule2.1).contains(&value)
    }
}

impl FromStr for Rule {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let colon = s.find(":").unwrap();
        let name = s[..colon].to_string();
        let mut words = s[(colon+1)..].split_ascii_whitespace();
        let (rule1, _, rule2) = (
            words.next().unwrap(),
            words.next().unwrap(),
            words.next().unwrap(),
        );

        let rule1 = rule1.splitn(2, "-").map(|x| x.parse::<u64>().unwrap()).collect::<Vec<_>>();
        let rule1 = (rule1[0], rule1[1]);

        let rule2 = rule2.splitn(2, "-").map(|x| x.parse::<u64>().unwrap()).collect::<Vec<_>>();
        let rule2 = (rule2[0], rule2[1]);

        Ok(Rule { name: name.to_string(), rule1, rule2})
    }
}

pub fn main() -> std::io::Result<()> {
    let file = File::open("input/16.txt")?;
    let reader = BufReader::new(file);
    let mut lines = reader
        .lines()
        .map(|s| s.unwrap());

    let mut rules: Vec<Rule> = Vec::new();
    loop {
        if let Some(s) = lines.next() {
            if s == "" {
                break;
            }
            rules.push(s.parse::<Rule>().unwrap());
        }
    }

    lines.next();               // skip over your ticket:

    let your_ticket = lines.next()
        .unwrap()
        .split(",")
        .map(|x| x.parse::<u64>().unwrap())
        .collect::<Vec<_>>();

    lines.next();               // skip over newline
    lines.next();               // skip over nearby_tickets

    let mut nearby_tickets: Vec<Vec<u64>> = Vec::new();
    for line in lines {
        let ticket = line.split(",").map(|x| x.parse::<u64>().unwrap())
            .collect::<Vec<_>>();
        nearby_tickets.push(ticket);
    }
    // dbg!(part_1(&rules, &nearby_tickets));

    // dbg!(&your_ticket);
    let valid_nearby_tickets = nearby_tickets.iter().filter(|x| valid_ticket(&rules, x)).map(|x| x.clone()).collect::<Vec<_>>();
    // // dbg!(permute_valid(&your_ticket, &rules).unwrap().len());
    // for p in part_2(&rules, &valid_nearby_tickets).unwrap() {
    //     println!("{}", &p);
    // }

    part_2_fun(&rules, &valid_nearby_tickets);
    Ok(())
}

fn part_1(rules: &[Rule], nearby_tickets: &Vec<Vec<u64>>) -> u64 {
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

fn part_2(rules: &[Rule], tickets: &Vec<Vec<u64>>) -> Option<Vec<Rule>> {
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

fn part_2_fun(rules: &[Rule], tickets: &Vec<Vec<u64>>) -> Vec<Rule> {
    let mut solved_rows: HashMap<&str,usize> = HashMap::new();
    solved_rows.insert("wagon", 12);
    solved_rows.insert("route", 13);
    solved_rows.insert("class", 0);

    let mut row_appears_in: HashMap<String,Vec<usize>> = HashMap::new();

    let mut cols: Vec<Vec<u64>> = Vec::new();
    for _ in 0..tickets[0].len() {
        cols.push(Vec::new());
    }

    for ticket in tickets {
        for (i,v) in ticket.iter().enumerate() {
            cols[i].push(*v);
        }
    }

    // sort the columns
    for (i,col) in cols.iter_mut().enumerate() {
        // sort first
        col.sort_unstable();

        let min = col[0];
        let max = col[col.len()-1];

        // do basic bounds check
        println!("Col: {}, min: {}, max: {}", i, min, max);
        for rule in rules {
            let mut valid = true;
            for v in col.iter() {
                if !rule.valid(*v) {
                    valid = false;
                    break;
                }
            }
            if valid {
                row_appears_in.entry(rule.name.to_string()).and_modify(|x| x.push(i)).or_insert(vec!(i));
                println!("{}", rule);
            }
        }
        println!("");
    }

    dbg!(row_appears_in);

    let v = row_appears_in.to_vec();

    vec!()
}


fn permutations(rules: &[Rule]) -> Vec<Vec<Rule>> {
    if rules.is_empty() {
        return Vec::new();
    } else if rules.len() == 1 {
        return vec!(vec!(rules.first().unwrap().clone()));
    } else {
        let mut collected: Vec<Vec<Rule>> = Vec::new();
        for rule in rules {
            let mut remaining: HashSet<Rule> = HashSet::from_iter(rules.iter().cloned());
            remaining.remove(rule);

            let remaining = remaining.into_iter().collect::<Vec<_>>();
            for mut p in permutations(&remaining) {
                p.push(rule.clone());
                collected.push(p);
            }
        }
        collected
    }
}

fn permute_valid(ticket: &[u64], rules: &[Rule]) -> Option<Vec<Vec<Rule>>> {
    if rules.is_empty() {
        return Some(Vec::new());
    }
    if rules.len() == 1 {
        if !rules[0].valid(ticket[0]) {
            return None;
        } else {
            return Some(vec!(vec!(rules[0].clone())));
        }
    }

    // multi item step
    let mut collected: Vec<Vec<Rule>> = Vec::new();
    for rule in rules {
        if !rule.valid(ticket[0]) {
            continue;
        }

        let mut remaining: HashSet<Rule> = HashSet::from_iter(rules.iter().cloned());
        remaining.remove(rule);
        let remaining = remaining.into_iter().collect::<Vec<_>>();

        if let Some(valid_permutations) = permute_valid(&ticket[1..], &remaining) {
            for mut p in valid_permutations {
                let mut arr: Vec<Rule> = vec!(rule.clone());
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

fn permute_valid_2(tickets: &[Vec<u64>], rules: &[Rule], start_index: usize) -> Option<Vec<Vec<Rule>>> {
    if rules.is_empty() {
        return Some(Vec::new());
    }
    if rules.len() == 1 {
        for ticket in tickets {
            if !rules[0].valid(ticket[start_index]) {
                return None;
            }
        }
        return Some(vec!(vec!(rules[0].clone())));
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
                let mut arr: Vec<Rule> = vec!(rule.clone());
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
            return false
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_rule() {
        let rule = Rule { name: "class".to_string(), rule1: (1,3), rule2: (5,7) };
        assert_eq!(rule, "class: 1-3 or 5-7".parse::<Rule>().unwrap());
    }

    #[test]
    fn test_valid_rule() {
        let rule = Rule { name: "class:".to_string(), rule1: (1,3), rule2: (5,7) };
        assert!(rule.valid(1));
        assert!(rule.valid(2));
        assert!(rule.valid(3));
        assert!(!rule.valid(4));
        assert!(rule.valid(5));
        assert!(rule.valid(6));
        assert!(rule.valid(7));
        assert!(!rule.valid(8));
    }

    #[test]
    fn test_permutations() {
        let rule1 = Rule { name: "rule1".to_string(), rule1: (1,3), rule2: (5,7) };
        let rule2 = Rule { name: "rule2".to_string(), rule1: (1,3), rule2: (5,7) };
        let rule3 = Rule { name: "rule3".to_string(), rule1: (1,3), rule2: (5,7) };
        assert_eq!(6, permutations(&vec!(rule1, rule2, rule3)).len());
    }

    #[test]
    fn test_permute_single() {
        let rules = vec!(Rule { name: "seat".to_string(), rule1: (3,13), rule2: (16,19) });
        assert!(permute_valid(&vec!(1), &rules).is_none());

        let rules = vec!(Rule { name: "seat".to_string(), rule1: (0,13), rule2: (16,19) });
        assert_eq!(1, dbg!(permute_valid(&vec!(1), &rules)).unwrap().len());

        let rules = vec!(
            Rule { name: "class".to_string(), rule1: (0,1), rule2: (8,19) },
            Rule { name: "row".to_string(), rule1: (0,5), rule2: (8,19) },
        );
        assert!(permute_valid(&vec!(4,5), &rules).is_none());
    }

    #[test]
    fn test_sut() {
        let rules = vec!(
            Rule { name: "class".to_string(), rule1: (0,1), rule2: (4,19) },
            Rule { name: "row".to_string(), rule1: (0,5), rule2: (8,19) },
            Rule { name: "seat".to_string(), rule1: (0,13), rule2: (16,19) }
        );

        // should be 1, row,class,seat
        let passport = vec!(vec!(3,9,18));

        let permutations = permute_valid_2(&passport, &rules, 0).unwrap();
        dbg!(passport);
        for (i,rules) in permutations.iter().enumerate() {
            println!("permutation {}", i);
            for rule in rules {
                println!("{}", &rule);
            }
            println!("");
        }
        assert_eq!(1, permutations.len());
    }


}
