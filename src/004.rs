use std::fs::File;
use std::io::BufReader;
use std::collections::HashSet;
use std::io::prelude::*;

#[derive(Debug)]
struct Passport {
    ecl: String,
    pid: String,
    eyr: String,
    hcl: String,
    byr: String,
    iyr: String,
    cid: Option<String>,
    hgt: String,
}

// TODO: how to turn this into a FnOnce
fn between(x: i32, min: i32, max: i32) -> bool { min <= x && x <= max }

fn valid_height(s: &String) -> bool {
    let mut chars = s.chars().peekable();

    let mut height = 0;

    loop {
        match chars.peek() {
            Some(c) => if c.is_digit(10) { height = height * 10 + c.to_digit(10).unwrap()} else { break },
            None => break
        }
        chars.next();
    }

    let unit: String = chars.by_ref().collect();

    println!("height: {}, unit: {}", height, unit);

    match unit.as_str() {
        "cm" => between(height as i32, 150, 193),
        "in" => between(height as i32, 59, 76),
        _ => false,
    }
}

fn valid_color(s: &String) -> bool {
    if s.len() != 7 || !s.starts_with("#") {
        return false;
    }

    let mut chars = s.chars();
    chars.next(); // throw away #

    for char in chars {
        if !char.is_ascii_hexdigit() {
            return false;
        }

    }
    return true;
}

// TODO: hand-rolling a parser is very painful
fn parse_line(s: String) -> Result<Passport, String> {
    println!("parse_line: {}", s);

    let valid_ecl: HashSet<&'static str> =
        ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].iter().cloned().collect();

    // expected
    let mut byr: Option<String> = None;
    let mut iyr: Option<String> = None;
    let mut eyr: Option<String> = None;
    let mut hgt: Option<String> = None;
    let mut hcl: Option<String> = None;
    let mut ecl: Option<String> = None;
    let mut pid: Option<String> = None;
    let mut cid: Option<String> = None; // optional

    for item in s.trim().split_whitespace() {
        let (key, val) = item.split_at(item.find(':').expect("missing :"));
        let val = String::from(&val[1..]);

        println!("{} -> ({},{})", item, key, val);

        match key {
            "ecl" => if valid_ecl.contains(val.as_str()) { ecl = Some(val) },
            "pid" => if val.len() == 9 { pid = val.parse::<i32>().ok().map(|_| val)},
            "eyr" => if val.len() == 4 { eyr = val.parse::<i32>().ok().filter(|x| between(*x, 2020, 2030)).map(|_| val) },
            "hcl" => if valid_color(&val) { hcl = Some(val) },
            "byr" => if val.len() == 4 { byr = val.parse::<i32>().ok().filter(|x| between(*x, 1920, 2002)).map(|_| val) },
            "iyr" => if val.len() == 4 { iyr = val.parse::<i32>().ok().filter(|x| between(*x, 2010, 2020)).map(|_| val) },
            "cid" => cid = Some(val),
            "hgt" => if valid_height(&val) { hgt = Some(val)},
            _ => (),
        }
    }

    println!("byr:{:?}, iyr:{:?}, eyr:{:?}, hgt:{:?}, hcl:{:?}, ecl:{:?}, pid:{:?}, cid:{:?}", byr, iyr, eyr, hgt, hcl, ecl, pid, cid);

    match (byr, iyr, eyr, hgt, hcl, ecl, pid, cid) {
        (Some(byr), Some(iyr), Some(eyr), Some(hgt), Some(hcl), Some(ecl), Some(pid), cid) =>
            Ok(Passport {
                ecl, pid, eyr, hcl, byr, iyr, hgt, cid
            }),
        _ => {
            Err(s)
        }
    }
}

fn main () -> std::io::Result<()> {
    let file = File::open("input/004.next.txt")?;
    let reader = BufReader::new(file);

    let count = reader.lines()
        .map(|x| x.unwrap())
        .chain(vec![String::from("")]) // inject null value so that scan can gracefully terminate
        .scan(String::new(), |acc, x| {
            if x == "" {
                let ret = Some(Some(acc.clone()));
                // println!("emitting! {:?}", ret);
                acc.clear();
                ret
            } else {
                // println!("collecting! {}", x);
                acc.push(' ');
                acc.push_str(&x);
                return Some(None);
            }
        }).filter_map(|x| match x {
            Some(x) => match parse_line(x) {
                Ok(passport) => Some(passport),
                Err(err) => {
                    println!("INVALID: {}", err);
                    None
                }
            },
            None => None
        })
        .fold(0, |acc,x| {
            println!("passport: {:?}", x);
            acc + 1
        });

    println!("count={}", count);

    Ok(())
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn test_iyr() {
        assert!(between(200,200,2001))
    }

    #[test]
    fn test_height() {
        assert!(valid_height(&String::from("181cm")))
    }

    #[test]
    fn test_should_parse() {
        assert!(parse_line(String::from("iyr:2010 ecl:gry hgt:181cm pid:591597745 byr:1920 hcl:#6b5442 eyr:2029 cid:123")).is_ok())
    }
}
