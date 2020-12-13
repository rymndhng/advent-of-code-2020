use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;

pub fn main() -> std::io::Result<()> {
    let file = File::open("input/13.txt")?;
    let reader = BufReader::new(file);
    let cmds = reader
        .lines()
        .map(|s| s.unwrap())
        .collect::<Vec<_>>();

    let t = cmds.get(0).and_then(|x| x.parse::<usize>().ok()).unwrap();
    let schedules = cmds.get(1).map(|x| x.split(','))
        .unwrap()
        .enumerate()
        .filter_map(|(i,s)| s.parse::<usize>().map(|s| (i,s)).ok())
        .collect::<Vec<_>>();

    // dbg!(&schedules);
    dbg!(part_1(t, &schedules.iter().map(|s| s.1).collect::<Vec<_>>()));
    // dbg!(part_2_naive(&schedules, 1));
    dbg!(part_2_chinese_remainder_theory(&schedules));
    // let part_2 = 1068781;
    // for sched in schedules {
    //     println!("\nExpected/Actual");
    //     println!("{} % {} = {}", part_2, sched.1, sched.0);
    //     println!("{} % {} = {}", part_2, sched.1, part_2 % sched.1);
    // }

    Ok(())
}

pub fn part_1(t: usize, schedules: &[usize]) -> usize {
    let mut schedules = schedules.iter().map(|x| {
        let next_time = ((t / x) + 1) * x;
        (x, next_time)
    }).collect::<Vec<_>>();
    schedules.sort_by(|a,b| a.1.partial_cmp(&b.1).unwrap());

    let earliest_bus = *schedules.first().unwrap();
    earliest_bus.0 * ( earliest_bus.1 - t)
}


// Solve for "largest" fit. Doesn't work for real input.
#[allow(dead_code)]
pub fn part_2_naive(schedules: &[(usize, usize)], jump: u64) -> u64 {
    let mut iter: u64 = 0;
    loop {
        // always advance by largest size-ish
        iter += 1;
        let time: u64 = iter * jump;

        // find cloest number and see if it fits
        let done = schedules.iter().find(|(i,x)| {
            (time + (*i as u64)) % (*x as u64) != 0
        }).is_none();

        if done {
            return time;
        }
    }
}

// Gave up and looked at reddit for hints
// See https://www.geeksforgeeks.org/chinese-remainder-theorem-set-2-implementation/
pub fn part_2_chinese_remainder_theory(schedules: &[(usize, usize)]) -> usize {
    let product = schedules.iter().fold(1, |acc, x| acc * x.1);

    let mut sum = 0;
    for (offset, num) in schedules {
        let rem = num - (offset % num); // same as (num - offset) without overflow
        let pp = product / num;


        let mut inv: usize = 0;
        for i in 0..*num {  // O(n) search,

            if (pp * i) % num == 1 {
                inv = i;
                break;
            }
        }
        sum += rem * pp * inv;
    }

    sum % product
}
