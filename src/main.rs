use std::{
    env,
    time::{Duration, Instant},
};

use rayon::prelude::{IntoParallelRefIterator, ParallelIterator};

mod day1;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day17;
mod day18;
mod day19;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;

mod extra;

const PRINT_WIDTH: usize = 80;
const TIME_PAD_WIDTH: usize = 15;

fn main() {
    let enumerated_inputs: Vec<(usize, &str)> = vec![
        include_str!("../input/day1.txt"),
        include_str!("../input/day1.txt"),
        include_str!("../input/day2.txt"),
        include_str!("../input/day2.txt"),
        include_str!("../input/day3.txt"),
        include_str!("../input/day3.txt"),
        include_str!("../input/day4.txt"),
        include_str!("../input/day4.txt"),
        include_str!("../input/day5.txt"),
        include_str!("../input/day5.txt"),
        include_str!("../input/day6.txt"),
        include_str!("../input/day6.txt"),
        include_str!("../input/day7.txt"),
        include_str!("../input/day7.txt"),
        include_str!("../input/day8.txt"),
        include_str!("../input/day8.txt"),
        include_str!("../input/day9.txt"),
        include_str!("../input/day9.txt"),
        include_str!("../input/day10.txt"),
        include_str!("../input/day10.txt"),
        include_str!("../input/day11.txt"),
        include_str!("../input/day11.txt"),
        include_str!("../input/day12.txt"),
        include_str!("../input/day12.txt"),
        include_str!("../input/day13.txt"),
        include_str!("../input/day13.txt"),
        include_str!("../input/day14.txt"),
        include_str!("../input/day14.txt"),
        include_str!("../input/day15.txt"),
        include_str!("../input/day15.txt"),
        include_str!("../input/day16.txt"),
        include_str!("../input/day16.txt"),
        include_str!("../input/day17.txt"),
        include_str!("../input/day17.txt"),
        include_str!("../input/day18.txt"),
        include_str!("../input/day18.txt"),
        include_str!("../input/day19.txt"),
        include_str!("../input/day19.txt"),
    ]
    .into_iter()
    .enumerate()
    .collect();

    let start = Instant::now();

    let mut results: Vec<(usize, Duration, String)> = enumerated_inputs
        .par_iter()
        .map(|(index, input)| {
            let day = (index) / 2 + 1;
            let part = (index) % 2 + 1;

            if let Ok(day_str) = env::var("DAY") {
                if day_str.parse::<usize>().unwrap() != day {
                    return (*index, Duration::ZERO, "SKIPPED".to_string());
                }

                if let Ok(part_str) = env::var("PART") {
                    if part_str.parse::<usize>().unwrap() != part {
                        return (*index, Duration::ZERO, "SKIPPED".to_string());
                    }
                }
            }

            let start = Instant::now();

            let result = match (day, part) {
                (1, 1) => day1::solve_1(input),
                (1, 2) => day1::solve_2(input),
                (2, 1) => day2::solve_1(input),
                (2, 2) => day2::solve_2(input),
                (3, 1) => day3::solve_1(input),
                (3, 2) => day3::solve_2(input),
                (4, 1) => day4::solve_1(input),
                (4, 2) => day4::solve_2(input),
                (5, 1) => day5::solve_1(input),
                (5, 2) => day5::solve_2(input),
                (6, 1) => day6::solve_1(input),
                (6, 2) => day6::solve_2(input),
                (7, 1) => day7::solve_1(input),
                (7, 2) => day7::solve_2(input),
                (8, 1) => day8::solve_1(input),
                (8, 2) => day8::solve_2(input),
                (9, 1) => day9::solve_1(input),
                (9, 2) => day9::solve_2(input),
                (10, 1) => day10::solve_1(input),
                (10, 2) => day10::solve_2(input),
                (11, 1) => day11::solve_1(input),
                (11, 2) => day11::solve_2(input),
                (12, 1) => day12::solve_1(input),
                (12, 2) => day12::solve_2(input),
                (13, 1) => day13::solve_1(input),
                (13, 2) => day13::solve_2(input),
                (14, 1) => day14::solve_1(input),
                (14, 2) => day14::solve_2(input),
                (15, 1) => day15::solve_1(input),
                (15, 2) => day15::solve_2(input),
                (16, 1) => day16::solve_1(input),
                (16, 2) => day16::solve_2(input),
                (17, 1) => day17::solve_1(input),
                (17, 2) => day17::solve_2(input),
                (18, 1) => day18::solve_1(input),
                (18, 2) => day18::solve_2(input),
                (19, 1) => day19::solve_1(input),
                (19, 2) => day19::solve_2(input),
                _ => unreachable!(),
            };

            let duration = start.elapsed();

            (*index, duration, result)
        })
        .collect();

    let duration = start.elapsed();

    results.sort_by(|(a, _, _), (b, _, _)| a.cmp(b));

    let mut sum_duration = Duration::ZERO;

    for (index, duration, result) in results {
        sum_duration += duration;

        let day = (index) / 2 + 1;
        let part = (index % 2) + 1;
        let multiline = day == 10 && part == 2;

        let report = display_with_duration(
            80,
            15,
            multiline,
            format!("day{}pt{}: ", day, part).as_str(),
            "-",
            result.as_str(),
            duration,
        );

        println!("{}", report);
    }

    println!();

    println!(
        "{}",
        display_with_duration(
            PRINT_WIDTH,
            TIME_PAD_WIDTH,
            false,
            "> ",
            "-",
            "total thread time",
            sum_duration,
        )
    );

    println!(
        "{}",
        display_with_duration(
            PRINT_WIDTH,
            TIME_PAD_WIDTH,
            false,
            "> ",
            "-",
            "real time",
            duration,
        )
    );
}

fn display_with_duration(
    target_len: usize,
    time_len: usize,
    multiline: bool,
    prefix: &str,
    suffix: &str,
    content: &str,
    duration: Duration,
) -> String {
    let mut output = String::new();
    output += prefix;
    if multiline {
        output += "\n";
    }
    let duration_str = format!("{:?}", duration);
    let content_len = content.lines().map(|line| line.len()).max().unwrap();
    let pad_len =
        target_len - if !multiline { prefix.len() } else { 0 } - content_len - suffix.len();

    let main_pad = " ".repeat(pad_len - time_len);
    let time_pad = " ".repeat(time_len - duration_str.len());
    output += format!(
        "{}{}{}{}{}",
        content, main_pad, suffix, time_pad, duration_str
    )
    .as_str();

    output
}

#[cfg(test)]
mod tests {
    use crate::{
        day1, day10, day11, day12, day13, day14, day15, day16, day17, day2, day3, day4, day5, day6,
        day7, day8, day8::TreeGrid, day9,
    };

    #[test]
    fn day1_eq_example() {
        let input = include_str!("../example_input/day1.txt");
        assert_eq!(day1::solve_1(input), "24000");
        assert_eq!(day1::solve_2(input), "45000");
    }

    #[test]
    fn day2_eq_example() {
        let input = include_str!("../example_input/day2.txt");
        assert_eq!(day2::solve_1(input), "15");
        assert_eq!(day2::solve_2(input), "12");
    }

    #[test]
    fn day3_eq_example() {
        let input = include_str!("../example_input/day3.txt");
        assert_eq!(day3::solve_1(input), "157");
        assert_eq!(day3::solve_2(input), "70");
    }

    #[test]
    fn day4_eq_example() {
        let input = include_str!("../example_input/day4.txt");
        assert_eq!(day4::solve_1(input), "2");
        assert_eq!(day4::solve_2(input), "4");
    }

    #[test]
    fn day5_eq_example() {
        let input = include_str!("../example_input/day5.txt");
        assert_eq!(day5::solve_1(input), "CMZ".to_string());
        assert_eq!(day5::solve_2(input), "MCD".to_string());
    }

    #[test]
    fn day6_eq_example() {
        let input = include_str!("../example_input/day6.txt");
        assert_eq!(day6::solve_1(input), "7");
        assert_eq!(day6::solve_2(input), "19");
    }

    #[test]
    fn day7_eq_example() {
        let input = include_str!("../example_input/day7.txt");
        assert_eq!(day7::solve_1(input), "95437");
        assert_eq!(day7::solve_2(input), "24933642");
    }

    #[test]
    fn day8_eq_example() {
        let input = include_str!("../example_input/day8.txt");
        assert_eq!(day8::solve_1(input), "21");
        let scan = TreeGrid::from(input);
        assert_eq!(scan.scenic_score(&day8::Coord { x: 2, y: 1 }), 4);
        assert_eq!(scan.scenic_score(&day8::Coord { x: 0, y: 0 }), 0);
        assert_eq!(day8::solve_2(input), "8")
    }

    #[test]
    fn day9_eq_example() {
        let input = include_str!("../example_input/day9.txt");
        assert_eq!(day9::solve_1(input), "13");
        assert_eq!(day9::solve_2(input), "1");
        let input = include_str!("../example_input/day9pt2.txt");
        assert_eq!(day9::solve_2(input), "36");
    }

    #[test]
    fn day10_eq_example() {
        let input = include_str!("../example_input/day10.txt");

        assert_eq!(day10::solve_1(input), "13140");

        assert_eq!(
            day10::solve_2(input),
            r"##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######....."
        );
    }

    #[test]
    fn day11_eq_example() {
        let input = include_str!("../example_input/day11.txt");

        assert_eq!(day11::solve_1(input), "10605");
        assert_eq!(day11::solve_2(input), "2713310158");
    }

    #[test]
    fn day12_eq_example() {
        let input = include_str!("../example_input/day12.txt");

        assert_eq!(day12::solve_1(input), "31");
        assert_eq!(day12::solve_2(input), "29");
    }

    #[test]
    fn day13_eq_example() {
        let input = include_str!("../example_input/day13.txt");

        assert_eq!(day13::solve_1(input), "13");
        assert_eq!(day13::solve_2(input), "140");
    }

    #[test]
    fn day14_eq_example() {
        let input = include_str!("../example_input/day14.txt");

        assert_eq!(day14::solve_1(input), "24");
        assert_eq!(day14::solve_2(input), "93");
    }

    #[test]
    fn day15_eq_example() {
        let input = include_str!("../example_input/day15.txt");

        assert_eq!(day15::solve_1(input), "26");
        assert_eq!(day15::solve_2(input), "56000011");
    }

    #[test]
    fn day16_eq_example() {
        let input = include_str!("../example_input/day16.txt");

        assert_eq!(day16::solve_1(input), "1651");
        assert_eq!(day16::solve_2(input), "1707");
    }

    #[test]
    fn day17_eq_example() {
        let input = include_str!("../example_input/day17.txt");

        assert_eq!(day17::solve_1(input), "3068");
        assert_eq!(day17::solve_2(input), "3068");
    }

    #[test]
    fn day18_eq_example() {
        use crate::day18;

        let input = include_str!("../example_input/day18.txt");

        assert_eq!(day18::solve_1(input), "64");
        assert_eq!(day18::solve_2(input), "58");
    }

    #[test]
    fn day19_eq_example() {
        use crate::day19;

        let input = include_str!("../example_input/day19.txt");

        assert_eq!(day19::solve_1(input), "33");
        assert_eq!(day19::solve_2(input), "58");
    }
}
