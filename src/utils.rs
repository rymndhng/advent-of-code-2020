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
