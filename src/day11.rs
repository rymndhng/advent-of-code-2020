use std::fmt;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

#[derive(PartialEq, Clone)]
struct Grid(Vec<Vec<char>>);

impl fmt::Display for Grid {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut s = String::new();
        for row in &self.0 {
            for c in row {
                s.push(*c);
            }
            s.push('\n');
        }
        write!(f, "{}", s)
    }
}

impl Grid {
    fn filled_seats(&self) -> u32 {
        let mut count = 0;
        for row in &self.0 {
            for c in row {
                if *c == '#' {
                    count = count + 1;
                }
            }
        }
        count
    }

    fn seat_at(&self, pos: (i32,i32)) -> Option<&char> {
        let (x,y) = pos;
        self.0.get(x as usize).and_then(|row| row.get(y as usize))
    }
}


pub fn main() -> std::io::Result<()> {
    let file = File::open("input/011.txt")?;
    let reader = BufReader::new(file);
    let grid = reader
        .lines()
        .map(|s| s.unwrap().chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let grid = Grid(grid);

    let part1_result = solve(&grid, part_1);
    println!("{}", part1_result);
    dbg!(&part1_result.filled_seats());

    let part2_result = solve(&grid, part_2);
    println!("{}", part2_result);
    dbg!(&part2_result.filled_seats());

    Ok(())
}

fn solve<F>(grid: &Grid, solver: F) -> Grid
  where F: Fn(&Grid) -> Grid
{
    let mut current_grid = grid.clone();
    loop {
        let next_grid = solver(&current_grid);
        if current_grid == next_grid {
            return current_grid;
        }
        current_grid = next_grid;
    }
}

lazy_static! {
    static ref VECTORS: Vec<(i32,i32)> =
        vec!((-1,-1), (-1, 0), (-1, 1),
             (0, -1),          ( 0, 1),
             (1, -1), ( 1, 0), ( 1, 1));
}

fn part_1(grid: &Grid) -> Grid {
    let mut new_grid = Vec::new();

    for (row_id, row) in grid.0.iter().enumerate() {
        let mut new_row = Vec::new();
        for (col_id, value) in row.iter().enumerate() {
            let mut occupied_adjacent_seats = 0;
            // println!("for index: {}, {}", row_id, col_id);
            for (row_offset, col_offset) in VECTORS.iter() {
                let adj_row_id = row_id as i32 - row_offset;
                let adj_col_id = col_id as i32 - col_offset;

                // println!("checking index: {}, {}", adj_row_id, adj_col_id);
                if let Some(a) = grid.seat_at((adj_row_id, adj_col_id)) {
                    if *a == '#' {
                        occupied_adjacent_seats = occupied_adjacent_seats + 1
                    }
                }
            }

            let next_value = match value {
                'L' if occupied_adjacent_seats == 0 => '#',
                '#' if occupied_adjacent_seats >= 4 => 'L',
                a => a.clone(),
            };
            new_row.push(next_value);
        }
        new_grid.push(new_row);
    }

    Grid(new_grid)
}

fn permute(pos: &(i32, i32), vector: &(i32, i32), dim: &(i32, i32)) -> Vec<(i32, i32)> {
    let mut vec = Vec::new();
    let mut pos = (pos.0 + vector.0, pos.1 + vector.1);
    while 0 <= pos.0 && pos.0 < dim.0 && 0 <= pos.1 && pos.1 < dim.1 {
        vec.push(pos);
        pos = (pos.0 + vector.0, pos.1 + vector.1);
    }
    vec
}

fn part_2(grid: &Grid) -> Grid {
    let mut new_grid = Vec::new();

    for (row_id, row) in grid.0.iter().enumerate() {
        let mut new_row = Vec::new();
        for (col_id, value) in row.iter().enumerate() {
            let dim = (grid.0.len() as i32, row.len() as i32);
            let mut occupied_adjacent_seats = 0;
            // println!("for index: {}, {}", row_id, col_id);
            for vector in VECTORS.iter() {
                let pos = (row_id as i32, col_id as i32);
                // dbg!(&pos, vector, &dim);
                let line_of_sight = permute(&pos, vector, &dim);

                // println!("checking index: {}, {}", adj_row_id, adj_col_id);
                for adj_pos in line_of_sight {
                    if let Some(a) = grid.seat_at(adj_pos) {
                        match *a {
                            '#' => {
                                occupied_adjacent_seats = occupied_adjacent_seats + 1;
                                break;
                            },
                            'L' => break,
                            _ => ()
                        }
                    }
                }
            }
            let next_value = match value {
                'L' if occupied_adjacent_seats == 0 => '#',
                '#' if occupied_adjacent_seats >= 5 => 'L',
                a => a.clone(),
            };
            new_row.push(next_value);
        }
        new_grid.push(new_row);
    }

    Grid(new_grid)
}
