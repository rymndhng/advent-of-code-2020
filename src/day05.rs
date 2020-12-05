use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::cmp;

#[derive(Debug)]
struct BoardingPass {
    row: i32,
    col: i32,
    id: i32
}

pub fn main () -> std::io::Result<()> {
    let file = File::open("input/005.txt")?;
    let reader = BufReader::new(file);

    let passports = reader.lines()
        .map(|x| x.unwrap())
        .filter_map(|x| parse_line(x).ok())
        .collect::<Vec<_>>();

    let part1 = passports.iter().fold(0, |acc,passport| cmp::max(acc, passport.id));

    dbg!(part1);

    let mut passport_ids: Vec<i32> = passports.iter().map(|x| x.id).collect();
    passport_ids.sort();

    let p1 = passport_ids.iter();
    let mut p2 = passport_ids.iter();
    p2.next();

    let part2 = dbg!(p1.zip(p2).find(|(&a,&b)| b - a == 2).unwrap());
    dbg!(part2.0 + 1);

    Ok(())
}

fn parse_line(s: String) -> Result<BoardingPass,String> {
    let (row, col) = s.split_at(7);

    let row = row.chars()
        .map(|c| match c {
            'B' => 1,
            'F' => 0,
            _ => panic!("Unexpected input! {}", c)
        })
        .fold(0, |acc,x| (acc << 1) + x);

    let col = col.chars()
        .map(|c| match c {
            'L' => 0,
            'R' => 1,
            _ => panic!("Unexpected input! {}", c)
        }).fold(0, |acc,x| (acc << 1) + x);

    Ok(BoardingPass { row, col, id: row * 8 + col})
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_thing() {
        assert_eq!(567, parse_line(String::from("BFFFBBFRRR")).unwrap().id);
        assert_eq!(119, parse_line(String::from("FFFBBBFRRR")).unwrap().id);
        assert_eq!(820, parse_line(String::from("BBFFBBFRLL")).unwrap().id);
    }
}
