/// Contains Part 1 of Day 17
///
/// In the interest of time, I copy/pasted the code and added an additional
/// dimension to Point.
///
/// Fortunately, the code still executed quickly enough I didn't have to wait
/// too long!
use std::collections::HashMap;
use std::collections::HashSet;
use std::fs;

pub fn main() -> std::io::Result<()> {
    let input = fs::read_to_string("input/17.txt")?;
    dbg!(part_1(&input));

    Ok(())
}

fn part_1(grid: &str) -> usize {
    let cycles = 6;
    let (mut grid, mut dim) = parse(grid);
    println!(
        "iter: {}, count: {}",
        0,
        grid.iter().filter(|(_, &active)| active).count()
    );

    for cycle in 1..=cycles {
        grid = next_state(&grid, dim);
        println!(
            "iter: {}, dim: {}, count: {}",
            cycle, dim,
            grid.iter().filter(|(_, &active)| active).count()
        );
        // dbg!(&grid);
        dim += 1;
    }

    grid.iter().filter(|(_, &active)| active).count()
}

type Point = (i32, i32, i32);

fn parse(grid: &str) -> (HashMap<Point, bool>, i32) {
    let mut points: HashMap<Point, bool> = HashMap::new();
    let dim = grid.lines().next().unwrap().len();
    let dim = (dim / 2) as i32;

    for (y, row) in grid.lines().enumerate() {
        for (x, on) in row.chars().enumerate() {
            let point = (x as i32 - dim, y as i32 - dim, 0);
            points.insert(point, on == '#');
        }
    }

    (points, dim as i32 + 2)
}

fn next_state(grid: &HashMap<Point, bool>, dim: i32) -> HashMap<Point, bool> {
    let mut next_state: HashMap<Point, bool> = HashMap::new();

    for point in grid_points(dim).iter() {
        let active = match grid.get(point).unwrap_or(&false) {
            true => [2, 3].contains(
                &surrounding_points(*point)
                    .iter()
                    .filter(|p| *grid.get(p).unwrap_or(&false))
                    .count(),
            ),
            false => {
                3 == surrounding_points(*point)
                    .iter()
                    .filter(|p| *grid.get(p).unwrap_or(&false))
                    .count()
            }
        };

        next_state.insert(*point, active);
    }
    next_state
}

fn grid_points(dim: i32) -> HashSet<Point> {
    let mut points = HashSet::new();
    for x in -dim..dim {
        for y in -dim..dim {
            for z in -dim..dim {
                points.insert((x, y, z));
            }
        }
    }
    dbg!(&points.len());
    points
}

fn surrounding_points(point: Point) -> HashSet<Point> {
    let (x, y, z) = point;
    let mut surrounding = HashSet::new();

    let xs = [x - 1, x, x + 1];
    let ys = [y - 1, y, y + 1];
    let zs = [z - 1, z, z + 1];

    for &x in &xs {
        for &y in &ys {
            for &z in &zs {
                if (x, y, z) != point {
                    surrounding.insert((x, y, z));
                }
            }
        }
    }

    surrounding
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample() {
        let input = ".#.
..#
###";

        let (hsh, _) = parse(input);
        assert_eq!(9, hsh.len());
        dbg!(&hsh);
        assert_eq!(true, *hsh.get(&(1, 0, 0)).unwrap());
    }

    #[test]
    fn test_surrounding_points() {
        assert_eq!(26, surrounding_points((0,0,0)).len());
    }
}
