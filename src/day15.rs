use std::collections::HashMap;

pub fn main() -> std::io::Result<()> {
    let input = vec!(0,20,7,16,1,18,15);
    dbg!(part_1(&input, 2020));
    dbg!(part_1(&input, 30000000)); // #YOLO: bruteforce it!

    Ok(())
}

fn part_1(input: &[u64], rounds: u64) -> u64 {
    let mut lookup: HashMap<u64,(u64,u64)> = HashMap::new();
    let mut input = input.iter();
    let mut spoken_num = 0;

    for turn in 1..(rounds + 1) {
        if let Some(&n) = input.next() {
            lookup.insert(n, (turn, turn));
            spoken_num = n;
        } else {
            match lookup.get(&spoken_num) {
                // previously spoken
                Some(&(last_previously_spoken, last_spoken)) => {
                    spoken_num = last_spoken - last_previously_spoken;
                    lookup.entry(spoken_num)
                        .and_modify(|x| *x = (x.1, turn))
                        .or_insert((turn, turn));
                },

                // never spoken before
                None => {
                    spoken_num = 0;
                    lookup.entry(spoken_num).and_modify(|x| {
                        *x = (x.1, turn);
                    });
                }
            }
        }
        // dbg!(&spoken_num);
        // dbg!(&lookup);
    }

    spoken_num
}


#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn test_input() {
        let input = vec!(0,3,6);
        assert_eq!(0, part_1(&input, 10));

        assert_eq!(1, part_1(&vec!(1,3,2), 2020));
        assert_eq!(10, part_1(&vec!(2,1,3), 2020));
        assert_eq!(27, part_1(&vec!(1,2,3), 2020));
        assert_eq!(78, part_1(&vec!(2,3,1), 2020));
        assert_eq!(438, part_1(&vec!(3,2,1), 2020));
        assert_eq!(1836, part_1(&vec!(3,1,2), 2020));
    }
}
