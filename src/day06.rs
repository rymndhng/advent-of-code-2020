use std::collections::HashSet;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

#[allow(dead_code)]
pub fn main() -> std::io::Result<()> {
    let file = File::open("input/006.txt")?;
    let reader = BufReader::new(file);

    // TODO: how to turn this pattern into a resuable trait (?)
    let votes = reader
        .lines()
        .map(|x| x.unwrap())
        .chain(vec![String::from("")]) // inject empty line to force termination
        .scan(String::new(), |acc, x| {
            if x == "" {
                let ret = Some(Some(acc.clone()));
                acc.clear();
                ret
            } else {
                acc.push_str(&x);
                acc.push('\n');
                // dbg!(acc);
                Some(None)
            }
        }).collect::<Vec<_>>();

    let part1 = votes.iter()
        .filter_map(|s| s.as_ref().map(|s| parse_answer(s)))
        .fold(0, |acc, answers| acc + answers.len());

    dbg!(part1);

    let part2 = votes.iter()
        .filter_map(|s| s.as_ref().map(|s| parse_answer_2(s)))
        .fold(0, |acc, answers| {
            // dbg!(&answers);
            acc + answers.len()
        });

    dbg!(part2);

    Ok(())
}

fn parse_answer(s: &str) -> HashSet<char> {
    let mut answers = HashSet::new();

    for c in s.chars() {
        if !c.is_whitespace() {
            answers.insert(c);
        }
    }

    answers
}

fn parse_answer_2(s: &str) -> HashSet<char> {
    s.lines()
        .map(|x| parse_answer(&x.to_string()))
        .fold(None, |acc, b| match acc {
            None => Some(b),
            Some(a) => Some(a.intersection(&b).cloned().collect::<HashSet<char>>()),
        })
        .unwrap_or_default()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_group() {
        assert_eq!(3, parse_answer(&String::from("abc")).len());
        assert_eq!(3, parse_answer(&String::from("a\nb\nc")).len());
        assert_eq!(3, parse_answer(&String::from("ab\nac")).len());
        assert_eq!(1, parse_answer(&String::from("a\na\na\na\na")).len());
    }

    #[test]
    fn test_parse_group_2() {
        // cool, the macro is much more succinct!
        assert_eq!(['a', 'b', 'c'].iter().cloned().collect::<HashSet<_>>(), parse_answer_2(&String::from("abc")));
        assert_eq!(hashset!('a', 'b', 'c'), parse_answer_2(&String::from("abc")));

        assert_eq!(HashSet::new(), parse_answer_2(&String::from("a\nb\nc")));
        assert_eq!(hashset!('a'), parse_answer_2(&String::from("ab\nac")));
        assert_eq!(hashset!('a'), parse_answer_2(&String::from("a\na\na\na\na")));
        assert_eq!(HashSet::default(), parse_answer_2(&String::from("\nci\nic\nic\nic\nic")));
        assert_eq!(hashset!('c', 'i'), parse_answer_2(&String::from("ci\nic\nic\nic\nic")));
    }
}
