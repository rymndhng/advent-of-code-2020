use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::num::ParseIntError;
use std::str::FromStr;

#[derive(Debug, PartialEq)]
struct Cmd {
    c: char,
    v: i32,
}

impl FromStr for Cmd {
    type Err = ParseIntError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut chars = s.chars();
        let c = chars.next().unwrap();
        let v = chars.collect::<String>().parse::<i32>()?;
        Ok(Cmd { c, v })
    }
}

#[allow(dead_code)]
pub fn main() -> std::io::Result<()> {
    let file = File::open("input/12.txt")?;
    let reader = BufReader::new(file);
    let cmds = reader
        .lines()
        .map(|s| s.unwrap().parse::<Cmd>().unwrap())
        .collect::<Vec<_>>();

    dbg!(part_1(&cmds));
    dbg!(part_2(&cmds));

    Ok(())
}

fn part_1(cmds: &[Cmd]) -> i32 {
    let mut pos: (i32, i32) = (0, 0);
    let mut dir_index = 0;
    let directions: Vec<(i32, i32)> = vec![(1, 0), (0, -1), (-1, 0), (0, 1)];

    for cmd in cmds.iter() {
        match cmd.c {
            'N' => pos.1 += cmd.v,
            'S' => pos.1 -= cmd.v,
            'E' => pos.0 += cmd.v,
            'W' => pos.0 -= cmd.v,
            'L' => {
                let turns = cmd.v / 90;
                dir_index -= turns;
            }
            'R' => {
                let turns = cmd.v / 90;
                dir_index += turns;
            }
            'F' => {
                let direction = directions.get(dir_index.rem_euclid(4) as usize).unwrap();
                pos.0 += direction.0 * cmd.v;
                pos.1 += direction.1 * cmd.v;
            }
            _ => panic!("uxpected input {:?}", cmd),
        }
    }

    pos.0.abs() + pos.1.abs()
}

fn part_2(cmds: &[Cmd]) -> i32 {
    let mut pos: (i32, i32) = (0, 0);
    let mut waypoint: (i32, i32) = (10, 1);

    for cmd in cmds.iter() {
        match cmd.c {
            'N' => waypoint.1 += cmd.v,
            'S' => waypoint.1 -= cmd.v,
            'E' => waypoint.0 += cmd.v,
            'W' => waypoint.0 -= cmd.v,
            'L' => match cmd.v {
                90  => waypoint = (-waypoint.1,  waypoint.0),
                180 => waypoint = (-waypoint.0, -waypoint.1),
                270 => waypoint = ( waypoint.1, -waypoint.0),
                _ => panic!("unexpected input {:?}", cmd),
            },
            'R' => match cmd.v {
                90  => waypoint = ( waypoint.1, -waypoint.0),
                180 => waypoint = (-waypoint.0, -waypoint.1),
                270 => waypoint = (-waypoint.1,  waypoint.0),
                _ => panic!("unexpected input {:?}", cmd),
            },
            'F' => {
                pos.0 += waypoint.0 * cmd.v;
                pos.1 += waypoint.1 * cmd.v;
            }
            _ => panic!("uxpected input {:?}", cmd),
        }
    }

    pos.0.abs() + pos.1.abs()
}
