use std::num;

#[macro_export]
macro_rules! hashset(
    ($($value:expr),+) => {
        {
            let mut m = ::std::collections::HashSet::new();
            $(
                m.insert($value);
            )+
                m
        }
    };
);

#[macro_export]
macro_rules! time_it {
    ($context:literal, $($s:stmt);+) => {
        let timer = std::time::Instant::now();
        $(
            $s
        )*
            println!("{}: {:?}", $context, timer.elapsed());
    }
}

#[derive(Debug)]
pub enum ParseError {
    EOF,
    InvalidSyntax(String),
    Parse(num::ParseIntError, String)
}
