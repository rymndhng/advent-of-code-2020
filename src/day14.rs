use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::num;
use std::str::FromStr;

#[allow(dead_code)]
pub fn main() -> std::io::Result<()> {
    let file = File::open("input/14.txt")?;
    let reader = BufReader::new(file);
    let cmds = reader.lines().map(|s| s.unwrap().parse::<Cmd>().unwrap()).collect::<Vec<_>>();
    dbg!(part_1(&cmds));
    dbg!(part_2(&cmds));

    Ok(())
}

fn part_2(cmds: &[Cmd]) -> u64 {
    let mut memory_map: HashMap<u64,u64> = HashMap::new();
    let mut mask = BitMask {ones: !0b0, zeros: 0};
    for cmd in cmds {
        match cmd {
            Cmd::Mask(m) => mask = *m,
            Cmd::Mem(loc, data) => {
                for location in mask.floating(*loc) {
                    memory_map.insert(location, *data);
                }
            }
        }
    }

    memory_map.values().sum()
}

fn part_1(cmds: &[Cmd]) -> u64 {
    let mut mask = BitMask {ones: !0b0, zeros: 0};
    let mut registers = Vec::with_capacity(65536);
    for _ in 1..65536 {
        registers.push(0);
    }

    for cmd in cmds {
        match cmd {
            Cmd::Mask(m) => mask = *m,
            Cmd::Mem(loc, data) => {
                let data = mask.mask(*data);
                registers[*loc as usize] = data;
            }
        };
    }
    registers.iter().sum()
}

#[derive(Debug)]
enum ParseError {
    EOF,
    InvalidSyntax,              // TODO: think about how to make this easier to debug
    Parse(num::ParseIntError)
}

#[derive(PartialEq, Debug, Clone, Copy)]
struct BitMask {
    ones: u64,
    zeros: u64,
}

static BASE36_MASK:u64 = 68719476735; // 2^36 - 1

impl BitMask {
    // need to mask up to only 0x36
    fn mask(&self, data: u64) -> u64 {
        (data | self.ones) & !self.zeros & BASE36_MASK
    }

    /// Returns a vector of the "floating" permutations data
    ///
    /// # Arguments
    /// * `data` - A 36bit number to find permutations for
    fn floating(&self, data: u64) -> Vec<u64> {
        let floating_mask = !(self.ones | self.zeros) & BASE36_MASK;
        let mut floating_values: Vec<u64> = vec!(data | self.ones);

        for i in 0..36 {
            if (floating_mask >> i & 1) == 1 {
                let mut permutations: Vec<u64> = Vec::new();
                for mask in &floating_values {
                    let float_with_one = mask | 1 << i;
                    let float_with_zero = mask & !(1 << i) & BASE36_MASK;
                    permutations.push(float_with_one);
                    permutations.push(float_with_zero);
                }
                floating_values = permutations;
            }
        }

        floating_values
    }
}

#[derive(PartialEq, Debug)]
enum Cmd {
    Mask(BitMask),
    Mem(u64, u64),
}

#[derive(Debug)]
struct Machine {
    registers: Vec<u64>,
    mask: BitMask,
}

impl FromStr for BitMask {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut ones = 0;
        let mut zeros = 0;

        for c in s.chars() {
            ones = ones << 1;
            zeros = zeros << 1;
            if c == '1' {
                ones += 1;
            } else if c == '0' {
                zeros += 1;
            }
        }

        // if ones >= 2_u64.pow(35) || zeros >= 2_u64.pow(35) {
        //     return Err(ParseError::OverflowErr(String::from(format!("unexpected overflow {}", s))));
        // }

        Ok(BitMask { ones, zeros })
    }
}

impl FromStr for Cmd {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut input = s.split_ascii_whitespace();

        let (cmd, space, arg) = (
            input.next().ok_or(ParseError::EOF)?,
            input.next().ok_or(ParseError::EOF)?,
            input.next().ok_or(ParseError::EOF)?,
        );

        if space != "=" {
            return Err(ParseError::InvalidSyntax);
        }

        if cmd == "mask" {
            let mask = arg.parse::<BitMask>()?;
            return Ok(Cmd::Mask(mask));
        } else if cmd.starts_with("mem") {
            let idx = cmd[4..cmd.len()-1]
                .parse::<u64>()
                .map_err(ParseError::Parse)?;

            let value = arg.parse::<u64>().map_err(ParseError::Parse)?;

            return Ok(Cmd::Mem(idx, value));
        }

        return Err(ParseError::InvalidSyntax);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_input() {
        let input = "mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X
mem[8] = 11
mem[7] = 101
mem[8] = 0";

        let result = input
            .lines()
            .map(|x| x.parse::<Cmd>().unwrap())
            .collect::<Vec<_>>();
        assert_eq!(Cmd::Mask(BitMask { ones: 64, zeros: 2 }), result[0]);
    }

    #[test]
    fn test_parse_bitmask() {
        let input  = "1X0X00101111100010100X001010XX0X0XXX";
        let ones   = 0b100000101111100010100000101000000000;
        let zeros  = 0b001011010000011101011011010100101000;

        assert_eq!(BitMask { ones , zeros }, input.parse::<BitMask>().unwrap());
    }

    #[test]
    fn test_bitmask() {
        let mask = BitMask { ones: 64, zeros: 2 };
        // println!("{:#036b}", 11);
        // println!("{:#036b}", mask.ones);
        // println!("{:#036b}", mask.zeros);
        // println!("{:#036b}", 73);
        // println!("{:#036b}", 75);
        assert_eq!(73, mask.mask(11));
    }

    #[test]
    fn test_floating_mask() {
        let mask = "000000000000000000000000000000X1001X".parse::<BitMask>().unwrap();
        let floating_vs = mask.floating(42);
        // for v in &floating_vs {
        //     println!("{:#08b}", v);
        // }
        assert_eq!(vec!(59,27,58,26), floating_vs);
    }
}
