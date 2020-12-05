use std::fmt;

use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

struct PasswordPolicy {
    min: u32,
    max: u32,
    c: char,
    password: String,
}

impl fmt::Display for PasswordPolicy {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({},{},{},{})", self.min, self.max, self.c, self.password)
    }
}

fn parse_line(s: String) -> PasswordPolicy {
    let mut chars = s.chars();

    let min = chars.by_ref().take_while(|&c| c != '-').collect::<String>().parse::<u32>()
        .expect("a number");

    let max = chars.by_ref().take_while(|&c| c != ' ').collect::<String>().parse::<u32>()
        .expect("a number");

    let c = chars.next().expect("a character");

    // consume unused chars
    chars.next(); chars.next();

    let password = chars.collect::<String>();

    PasswordPolicy {
        min,
        max,
        c,
        password,
    }
}

#[allow(dead_code)]
fn valid_policy_part1(p: &PasswordPolicy) -> bool {
    let count = p.password.chars().filter(|&c| c == p.c).count();
    (p.min as usize) <= count && count <= (p.max as usize)
}

fn valid_policy_part2(p: &PasswordPolicy) -> bool {
    let contains_char_at = |pos: u32| match p.password.chars().nth(pos as usize) {
        Some(c) => c == p.c,
        None => false
    };

    contains_char_at(p.min - 1) ^ contains_char_at(p.max - 1)
}

#[allow(dead_code)]
fn main () -> std::io::Result<()> {
    let file = File::open("input/002.txt")?;
    let reader = BufReader::new(file);

    let valid_passwords = reader.lines()
        .map(|x| parse_line(x.unwrap()))
        .filter(valid_policy_part2)
        .map(|x| println!("valid: {}", x)); // can't iterate twice

    println!("valid count {}", valid_passwords.count());

    Ok(())
}
