use adventofcode_2023::common::day_input_filename;
use adventofcode_2023::common::get_file_lines;
use adventofcode_2023::days;

fn test_day(day: u8, correct_a: &str, correct_b: &str) -> Result<(), std::io::Error> {
    let solve = days::get_solver(day).unwrap();
    let input_lines = get_file_lines(&day_input_filename(day))?;
    let (solution_a, solution_b) = solve(&input_lines);
    assert_eq!(
        solution_a.as_str(),
        correct_a,
        "Incorrect solution for day {}a",
        day
    );
    assert_eq!(
        solution_b.as_str(),
        correct_b,
        "Incorrect solution for day {}b",
        day
    );

    Ok(())
}

macro_rules! test_day {
    ($name: ident, $sol_a: literal, $sol_b: literal) => {
        #[test]
        fn $name() -> Result<(), std::io::Error> {
            let day_name = stringify!($name);
            let day_num: u8 = day_name[3..].parse().unwrap();
            test_day(day_num, $sol_a, $sol_b)
        }
    };
}

test_day!(day01, "54940", "54208");
test_day!(day02, "2486", "87984");
test_day!(day03, "520019", "75519888");
test_day!(day04, "15268", "6283755");
test_day!(day05, "174137457", "1493866");
test_day!(day06, "503424", "32607562");
test_day!(day07, "254024898", "254115617");
test_day!(day08, "14681", "14321394058031");
test_day!(day09, "1584748274", "1026");
test_day!(day10, "6860", "343");
test_day!(day11, "9274989", "357134560737");
