use std::collections::vec_deque::VecDeque;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

#[allow(dead_code)]
pub fn main() -> std::io::Result<()> {
    let file = File::open("input/009.txt")?;
    let reader = BufReader::new(file);

    let transmission = reader
        .lines()
        .map(|s| s.unwrap().parse::<u64>().unwrap())
        .collect::<Vec<_>>();

    #[allow(unused_must_use)]
    {
        // dbg!(part_1(&transmission, 5));
        // dbg!(part_2_brute_force(&transmission, 127));

        dbg!(part_1(&transmission, 25));
        dbg!(part_2_brute_force(&transmission, 1639024365));
    };

    Ok(())
}

fn part_1(input: &[u64], preamble_len: usize) -> Result<u64, &str> {
    let mut preamble = VecDeque::with_capacity(preamble_len);
    preamble.extend(&input[0..preamble_len]);

    for &n in &input[preamble_len..] {
        if is_data(preamble.make_contiguous(), n) {
            return Ok(n);
        }

        preamble.pop_front();
        preamble.push_back(n);
    }

    Err("Unable to find data")
}

fn is_data(preamble: &[u64], data: u64) -> bool {
    // dbg!(preamble, data);
    for a in preamble {
        for b in preamble {
            pif a == b {
                continue;
            }
            if data == (a + b) {
                return false;
            }
        }
    }
    return true;
}

fn part_2_brute_force(input: &[u64], expected_sum: u64) -> Result<u64, &str> {
    for window_len in 2..input.len() {
        let result = input
            .windows(window_len)
            .find(|&window| window.iter().sum::<u64>() == expected_sum);

        if let Some(window) = result {
            dbg!(window);
            return Ok(window.iter().min().unwrap() + window.iter().max().unwrap());
        }
    }
    return Err("failed to terminate");
}
