use std::str::FromStr;
use std::collections::HashSet;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

#[derive(Clone, Debug,PartialEq)]
enum Cmd {
    Nop(i32),
    Acc(i32),
    Jmp(i32),
}

#[allow(dead_code)]
pub fn main() -> std::io::Result<()> {
    let file = File::open("input/008.txt")?;
    let reader = BufReader::new(file);

    let commands = reader
        .lines()
        .map(|s| s.unwrap().parse::<Cmd>().unwrap())
        .collect::<Vec<_>>();

    #[allow(unused_must_use)]
    {
        dbg!(part_1(&commands));
        dbg!(part_2(&commands));
    }

    Ok(())
}

fn part_1(commands: &[Cmd]) -> Result<i32, (String, i32)> {
    let mut acc: i32 = 0;
    let mut next_instruction: i32 = 0;
    let mut executed_instructions: HashSet<i32> = HashSet::new();

    while let Some(cmd) = commands.get(next_instruction as usize) {
        if executed_instructions.contains(&next_instruction) {
            return Err((format!("ERROR: Looped at {}", next_instruction), acc));
        }
        executed_instructions.insert(next_instruction);

        match cmd {
            Cmd::Acc(arg) => {
                acc = acc + arg;
                next_instruction = next_instruction + 1;
            }
            Cmd::Nop(_) => {
                next_instruction = next_instruction + 1;
            }
            Cmd::Jmp(arg) => {
                next_instruction = next_instruction + arg;
            }
        }
    }

    return Ok(acc);
}

fn part_2(commands: &[Cmd]) -> Result<i32, &str> {
    for (idx, cmd) in commands.iter().enumerate() {
        let mutated_command = match cmd {
            Cmd::Nop(arg) => Some(Cmd::Jmp(*arg)),
            Cmd::Jmp(arg) => Some(Cmd::Nop(*arg)),
            _ => None,
        };

        if let Some(mutated_command) = mutated_command {
            let mut new_commands = Vec::with_capacity(commands.len());
            new_commands.extend_from_slice(&commands[0..idx]);
            new_commands.push(mutated_command);
            new_commands.extend_from_slice(&commands[idx + 1..]);

            if let Ok(ret) = part_1(&new_commands) {
                return Ok(ret);
            }
        }
    }

    return Err("Could not fix corrupted commands");
}

impl FromStr for Cmd {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (cmd, arg) = s.split_at(3);

        let cmd = match cmd {
            "nop" => Cmd::Nop,
            "acc" => Cmd::Acc,
            "jmp" => Cmd::Jmp,
            _ => return Err("unknown command"),
        };

        let mut chars = arg.trim().chars();

        let sign = match chars.next() {
            Some('+') => 1,
            Some('-') => -1,
            _ => return Err("unreachable!"),
        };

        let arg = chars.collect::<String>().parse::<i32>().unwrap() * sign;

        Ok(cmd(arg))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day8_parse_line() {
        assert_eq!(Cmd::Nop(0), "nop +0".parse().unwrap());
        assert_eq!(Cmd::Acc(1), "acc +1".parse().unwrap());
    }
}
