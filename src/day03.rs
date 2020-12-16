use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

type ForestGrid = Vec<Vec<bool>>;

struct Slope {
    right: u32,
    down: u32
}

fn parse_line(s: String) -> Vec<bool> {
    s.chars().map(|c| c == '#').collect()
}

fn count_trees(grid: &[Vec<bool>], slope: &Slope) -> u32 {
    let mut count: u32 = 0;

    let mut row_n = 0;

    for (i, row) in grid.iter().enumerate() {
        if (i % slope.down as usize) != 0 {
            // println!("skipping line {}", i);
            continue;
        }

        let rowlen = row.len();

        if Some(&true) == row.get(row_n * (slope.right as usize) % rowlen) {
            count += 1
        }

        row_n += 1;
    }

    count
}

#[allow(dead_code)]
fn main () -> std::io::Result<()> {
    let file = File::open("input/003.txt")?;
    let reader = BufReader::new(file);

    let grid: ForestGrid = reader.lines()
        .map(|x| parse_line(x.unwrap()))
        .collect();

    // part 1
    println!("part 1 {:?}", count_trees(&grid, &Slope { right: 3, down: 1}));

    let slopes = vec![
        Slope { right: 1, down: 1},
        Slope { right: 3, down: 1},
        Slope { right: 5, down: 1},
        Slope { right: 7, down: 1},
        Slope { right: 1, down: 2}
    ];

    let result = slopes.iter()
        .map(|x| {
            let n = count_trees(&grid, x);
            println!("{}", n);
            n
        })
        .product::<u32>();

    println!("part 2: {}", result);

    Ok(())
}
