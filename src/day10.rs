use std::collections::HashMap;
use std::io::BufReader;
use std::fs::File;
use std::io::prelude::*;

#[allow(dead_code)]
pub fn main() -> std::io::Result<()> {
    let file = File::open("input/010.txt")?;
    let reader = BufReader::new(file);
    let mut joltage = reader
        .lines()
        .map(|s| s.unwrap().parse::<u64>().unwrap())
        .collect::<Vec<_>>();

    joltage.sort_unstable();

    joltage.insert(0, 0);
    joltage.push(joltage.last().unwrap() + 3);

    let mut ones: usize = 0;
    let mut twos: usize = 0;
    let mut threes: usize = 0;


    joltage
        .windows(2)
        .for_each(|a| match a[1] - a[0] {
            1 => ones += 1,
            2 => twos += 1,
            3 => threes += 1,
            _ => panic!("unexpected input")
        });

    dbg!(ones);
    dbg!(twos);
    dbg!(threes);
    dbg!(ones * threes);

    time_it!("part_2_hashmap", {
        dbg!(part_2_hashmap (&joltage));
    });
    // vector approach is 6x faster, presumably bc of less random access
    time_it!("part_2_vec",{
        dbg!(part_2_vec (&joltage));
    });

    time_it!("part_2_sliding_window", {
        dbg!(part_2_sliding_window(&joltage));
    });


    // experiemtn with usize only option
    let file = File::open("input/010.txt")?;
    let reader = BufReader::new(file);
    let mut joltage = reader
        .lines()
        .map(|s| s.unwrap().parse::<usize>().unwrap())
        .collect::<Vec<_>>();

    joltage.sort_unstable();

    joltage.insert(0, 0);
    joltage.push(joltage.last().unwrap() + 3);

    time_it!("part_2_sliding_window_usize", {
        dbg!(part_2_sliding_window_usize(&joltage));
    });

    Ok(())
}

pub fn part_2_hashmap(joltages: &[u64]) -> u64 {
    let mut paths_count: HashMap<u64,u64> = HashMap::new();
    paths_count.insert(0, 1);

    for (idx, joltage) in joltages.iter().enumerate() {
        if let Some(&current_paths) = paths_count.get(joltage) {
            for next_index in idx+1 .. idx+4 {
                if let Some(n) = joltages.get(next_index) {
                    if n - joltage <= 3 {
                        let entry = paths_count.entry(*n).or_insert(0);
                        *entry += current_paths;
                    }
                }
            }
        }
    }

    return *paths_count.get(joltages.last().unwrap()).unwrap();
}

pub fn part_2_vec(joltages: &[u64]) -> u64 {
    let mut paths_count: Vec<u64> = Vec::with_capacity(joltages.len());
    for _i in 0..joltages.len() {
        paths_count.push(0);
    }
    paths_count[0] = 1;

    for (idx, joltage) in joltages.iter().enumerate() {
        let current_paths = paths_count[idx];
        for next_index in idx+1 .. idx+4 {
            if let Some(n) = joltages.get(next_index) {
                if n - joltage <= 3 {
                    paths_count[next_index] += current_paths;
                }
            }
        }
    }
    paths_count[joltages.len()-1]
}


pub fn part_2_sliding_window(joltages: &[u64]) -> u64 {
    let mut array: [u64; 4] = [1, 0, 0, 0];
    let mut prev = 0;

    for (idx, joltage) in joltages.iter().enumerate() {
        let current_paths = array[*joltage as usize % 4];

        // set "skipped" over entries to 0
        for idx in prev .. *joltage {
            array[idx as usize % 4] = 0;
        }

        for next_index in idx+1 .. idx+4 {
            if let Some(n) = joltages.get(next_index) {
                if n - joltage <= 3 {
                    array[*n as usize % 4] += current_paths;
                }
            }
        }
        prev = *joltage;
    }
    return array[*joltages.last().unwrap() as usize % 4];
}

pub fn part_2_sliding_window_usize(joltages: &[usize]) -> usize {
    let mut array: [usize; 4] = [1, 0, 0, 0];
    let mut prev = 0;

    for (idx, joltage) in joltages.iter().enumerate() {
        let current_paths = array[*joltage % 4];

        // set "skipped" over entries to 0
        for idx in prev .. *joltage {
            array[idx % 4] = 0;
        }

        for next_index in idx+1 .. idx+4 {
            if let Some(n) = joltages.get(next_index) {
                if n - joltage <= 3 {
                    array[*n % 4] += current_paths;
                }
            }
        }
        prev = *joltage;
    }
    return array[*joltages.last().unwrap() % 4];
}
