pub mod day01;
pub mod day02;
pub mod day03;
pub mod day04;
pub mod day05;
pub mod day06;
pub mod day07;
pub mod day08;
pub mod day09;
pub mod day10;
pub mod day11;
pub mod day12;
pub mod day13;
pub mod day14;
pub mod day15;
pub mod day16;

macro_rules! days {
    ($($day_mod:ident),*) => {
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

days!(
    day01, day02, day03, day04, day05, day06, day07, day08, day09, day10, day11, day12, day13,
    day14, day15, day16
);
