use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

#[allow(dead_code)]
pub fn main() -> std::io::Result<()> {
    let file = File::open("input/007.txt")?;
    let reader = BufReader::new(file);

    let entries: HashMap<String, Vec<BagSpec>> =
        reader.lines().map(|s| parse_line(s.unwrap())).collect();

    dbg!(part_1("shiny gold bags", &entries));
    dbg!(part_2("shiny gold bags", &entries) - 1); // subtract 1 to not count the outside

    Ok(())
}

pub fn part_1(key: &str, entries: &HashMap<String, Vec<BagSpec>>) -> usize {
    let mut reverse_lookup: HashMap<&str, HashSet<String>> = HashMap::new();
    for (bag_name, contains) in entries.iter() {
        for bag_spec in contains {
            let set = reverse_lookup
                .entry(&bag_spec.name)
                .or_insert_with(HashSet::new);
            set.insert(bag_name.clone());
        }
    }

    let mut parents = HashSet::new();
    let mut explore = VecDeque::new();
    explore.push_back(key);

    while let Some(key) = explore.pop_front() {
        if let Some(contained) = reverse_lookup.get(key) {
            for item in contained.iter() {
                if !parents.contains(item) {
                    parents.insert(item);
                    explore.push_back(item);
                }
            }
        }
    }

    parents.len()
}

pub fn part_2(key: &str, hsh: &HashMap<String, Vec<BagSpec>>) -> i32 {
    let contained = hsh.get(key).expect("should have had a value");
    let mut count = 1;
    for item in contained.iter() {
        count += item.count * (part_2(&item.name, &hsh));
    }
    count
}

#[derive(Hash, Clone, Debug, Eq, PartialEq)]
pub struct BagSpec {
    name: String,
    count: i32,
}

pub fn parse_line(s: String) -> (String, Vec<BagSpec>) {
    let mut bag_name = String::new();
    let mut contains_bags: Vec<BagSpec> = Vec::new();

    let mut words = s.split_whitespace();

    loop {
        match words.next() {
            Some("contain") => {
                bag_name = bag_name.trim().to_string();
                break;
            }
            Some(s) => {
                bag_name.push_str(s);
                bag_name.push(' ')
            }
            None => panic!("Unexpected end-of-line"),
        }
    }

    let mut name = String::new();
    let mut count: i32 = 0;
    for word in words {
        match word {
            "bag." | "bags." | "bag," | "bags," => {
                if count != 0 {
                    contains_bags.push(BagSpec {
                        name: name.clone() + "bags",
                        count
                    });
                }
                count = 0;
                name.clear();
            }
            s => match s.parse::<i32>() {
                Ok(c) => count = c,
                Err(_) => {
                    name.push_str(s);
                    name.push(' ')
                }
            },
        }
    }

    (bag_name, contains_bags)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day7_parse_group() {
        let bag1 = BagSpec {
            name: "bright white bags".to_string(),
            count: 1,
        };
        let bag2 = BagSpec {
            name: "muted yellow bags".to_string(),
            count: 2,
        };

        let expected = ("light red bags".to_string(), vec![bag1, bag2]);

        assert_eq!(
            expected,
            parse_line(
                "light red bags contain 1 bright white bag, 2 muted yellow bags.".to_string()
            )
        );
    }
}
