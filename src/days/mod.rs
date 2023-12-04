macro_rules! days {
    ($($day_mod:ident),*) => {
        $(pub mod $day_mod;)*

        pub fn get_solver(day: u8) -> Option<fn(&[String]) -> crate::common::Solution> {
            match format!("day{:02}", day).as_str() {
                $(stringify!($day_mod) => Some($day_mod::solve),)*
                _ => None,
            }
        }
    };
}

pub fn all_numbers() -> Vec<u8> {
    (1..=25).filter(|&day| get_solver(day).is_some()).collect()
}

days!(day01, day02, day03, day04);
