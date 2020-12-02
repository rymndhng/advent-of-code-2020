use std::io::{self, BufReader};
use std::io::prelude::*;
use std::fs::File;

use std::collections::VecDeque;

fn main () -> io::Result<()> {
    let file = File::open("src/001.input")?;
    let reader = BufReader::new(file);

    let mut vector: VecDeque<i32> = VecDeque::new();

    for line in reader.lines() {
        match line {
            Ok(s) => {
                match s.parse::<i32>() {
                    Ok(n) => vector.push_back(n),
                    Err(err) => println!("skiping unparsable number: {}, Reason: {}", s, err)
                }
            },
            Err(err) => println!("skipping unexpected input {}", err),
        }
    }

    let mut result:Vec<i32> = Vec::new();

    'outer for &a in vector.iter() {
        for &b in vector.iter() {
            for &c in vector.iter() {
                if a+b+c == 2020 {
                    result = vec!(a,b,c);
                    break 'outer;
                }
            }
        }
    }

    // // produce lazy tuples, is that possible?

    // 'outer: loop {
    //     match vector.pop_front() {
    //         Some(a) => {
    //             for b in vector.iter() {
    //                 println!("sum of {} and {} = {}", a, b, a + *b);
    //                 if 2020 == (a + *b) {
    //                     result = vec!(a, *b);
    //                     break 'outer;
    //                 }
    //             }
    //         },
    //         None => break
    //     }
    // }

    println!("{:?}", result[0] * result [1] * result[2]);

    Ok(())
}
