use std::collections::HashMap;
use std::collections::HashSet;
use std::fs;
use std::hash::Hash;

pub fn main() -> std::io::Result<()> {
    let input = fs::read_to_string("input/17.txt")?;
    dbg!(part_1(&input));
    dbg!(part_2(&input));
    Ok(())
}

fn part_1(grid: &str) -> usize {
    let cycles = 6;
    let mut grid = Point3::parse(grid);
    println!("iter: {}, count: {}", 0, grid.len());
    for cycle in 1..=cycles {
        grid = next_state(&grid);
        println!("iter: {}, count: {}", cycle, grid.len());
    }
    grid.len()
}

// (5 29 60 320 188 1056 848)
fn part_2(grid: &str) -> usize {
    let cycles = 6;
    let mut grid = Point4::parse(grid);
    println!("iter: {}, count: {}", 0, grid.len());
    for cycle in 1..=cycles {
        grid = next_state(&grid);
        println!("iter: {}, count: {}", cycle, grid.len());
    }
    grid.len()
}

fn next_state<A: Clone + Hash + Eq + ConwayCube>(grid: &HashSet<A>) -> HashSet<A> {
    let frequencies =
        grid.iter()
            .flat_map(|p| p.neighbors())
            .fold(HashMap::new(), |mut acc, point| {
                let counter = acc.entry(point).or_insert(0);
                *counter += 1;
                acc
            });

    frequencies
        .iter()
        .filter(|(p, &count)| (count == 3) || (grid.contains(p) && count == 2))
        .map(|(p, _)| p.clone())
        .collect()
}

trait ConwayCube {
    fn neighbors(&self) -> HashSet<Self>
    where
        Self: std::marker::Sized;

    fn parse(grid: &str) -> HashSet<Self>
    where
        Self: std::marker::Sized;
}

type Point3 = (i32, i32, i32);

lazy_static! {
    static ref RANGE: [i32; 3] = [-1, 0, 1];
}

impl ConwayCube for Point3 {
    fn neighbors(&self) -> HashSet<Point3> {
        // HOW to implement an iterator instead?
        RANGE
            .iter()
            .flat_map(|x| RANGE.iter().map(move |y| (x, y)))
            .flat_map(|(x, y)| RANGE.iter().map(move |z| (x, y, z)))
            .map(|(x, y, z)| (self.0 + x, self.1 + y, self.2 + z))
            .filter(|&p| p != *self)
            .collect()
    }

    fn parse(grid: &str) -> HashSet<Point3> {
        let mut points: HashSet<Point3> = HashSet::new();
        for (y, row) in grid.lines().enumerate() {
            for (x, c) in row.chars().enumerate() {
                if c == '#' {
                    let point = (x as i32, y as i32, 0);
                    points.insert(point);
                }
            }
        }
        points
    }
}

type Point4 = (i32, i32, i32, i32);
impl ConwayCube for Point4 {
    fn neighbors(&self) -> HashSet<Point4> {
        RANGE
            .iter()
            .flat_map(|x| RANGE.iter().map(move |y| (x, y)))
            .flat_map(|(x, y)| RANGE.iter().map(move |z| (x, y, z)))
            .flat_map(|(x, y, z)| RANGE.iter().map(move |w| (x, y, z, w)))
            .map(|(x, y, z, w)| (self.0 + x, self.1 + y, self.2 + z, self.3 + w))
            .filter(|&p| p != *self)
            .collect()
    }

    fn parse(grid: &str) -> HashSet<Point4> {
        let mut points: HashSet<Point4> = HashSet::new();
        for (y, row) in grid.lines().enumerate() {
            for (x, c) in row.chars().enumerate() {
                if c == '#' {
                    let point = (x as i32, y as i32, 0, 0);
                    points.insert(point);
                }
            }
        }
        points
    }
}
