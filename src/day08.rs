use std::collections::HashSet;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

#[derive(Hash, Clone, Debug, Eq, PartialEq)]
enum Cmd {
    Nop,
    Acc,
    Jmp,
}

#[derive(Hash, Clone, Debug, Eq, PartialEq)]
struct Command {
    cmd: Cmd,
    arg: i32,
}

pub fn main() -> std::io::Result<()> {
    let file = File::open("input/008.txt")?;
    let reader = BufReader::new(file);

    let commands = reader
        .lines()
        .map(|s| parse_line(&s.unwrap()))
        .collect::<Vec<_>>();

    #[allow(unused_must_use)]
    {
        dbg!(part_1(&commands));
        dbg!(part_2(&commands));
    }

    Ok(())
}

fn part_1(commands: &[Command]) -> Result<i32, (String, i32)> {
    let mut acc: i32 = 0;
    let mut next_instruction: i32 = 0;
    let mut previously_executed: HashSet<i32> = HashSet::new();

    while let Some(cmd) = commands.get(next_instruction as usize) {
        if previously_executed.contains(&next_instruction) {
            return Err((format!("ERROR: Looped at {}", next_instruction), acc));
        }
        previously_executed.insert(next_instruction);

        match cmd.cmd {
            Cmd::Acc => {
                acc = acc + cmd.arg;
                next_instruction = next_instruction + 1;
            }
            Cmd::Nop => {
                next_instruction = next_instruction + 1;
            }
            Cmd::Jmp => {
                next_instruction = next_instruction + cmd.arg;
            }
        }
    }

    return Ok(acc);
}

fn part_2(commands: &[Command]) -> Result<i32, &str> {
    for (idx, command) in commands.iter().enumerate() {
        let mutated_commands = match command.cmd {
            Cmd::Nop => {
                let mut new_commands = Vec::with_capacity(commands.len());
                new_commands.extend_from_slice(&commands[0..idx]);
                new_commands.push(Command {
                    cmd: Cmd::Jmp,
                    arg: command.arg,
                });
                new_commands.extend_from_slice(&commands[idx + 1..]);
                Some(new_commands)
            }
            Cmd::Jmp => {
                let mut new_commands = Vec::with_capacity(commands.len());
                new_commands.extend_from_slice(&commands[0..idx]);
                new_commands.push(Command {
                    cmd: Cmd::Nop,
                    arg: command.arg,
                });
                new_commands.extend_from_slice(&commands[idx + 1..]);
                Some(new_commands)
            }
            _ => None,
        };

        if let Some(mutated_commands) = mutated_commands {
            if let Ok(ret) = part_1(&mutated_commands) {
                return Ok(ret);
            }
        }
    }

    return Err("Could not fix corrupted commands");
}

fn parse_line(s: &str) -> Command {
    let (cmd, arg) = s.split_at(3);

    let cmd = match cmd {
        "nop" => Cmd::Nop,
        "acc" => Cmd::Acc,
        "jmp" => Cmd::Jmp,
        _ => panic!("unexpected input! {}", cmd),
    };

    let mut chars = arg.trim().chars();

    let sign = match chars.next() {
        Some('+') => 1,
        Some('-') => -1,
        _ => unreachable!()
    };

    let arg = chars.collect::<String>().parse::<i32>().unwrap() * sign;

    Command { cmd, arg }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day8_parse_line() {
        let expected = Command {
            cmd: Cmd::Nop,
            arg: 0,
        };
        assert_eq!(expected, parse_line("nop +0"));
        assert_eq!(
            Command {
                cmd: Cmd::Acc,
                arg: 1
            },
            parse_line("acc +1")
        );
    }
}
