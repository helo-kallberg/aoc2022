mod day1pt1;
mod day1pt2;
mod day2pt1;
mod day2pt2;
mod day3pt1;
mod day3pt2;

fn main() {
    println!("{}", day1pt1::solve(include_str!("../input/day1pt1.txt")));
    println!("{}", day1pt2::solve(include_str!("../input/day1pt1.txt")));
    println!("{:?}", day2pt1::solve(include_str!("../input/day2pt1.txt")));
    println!("{:?}", day2pt2::solve(include_str!("../input/day2pt1.txt")));
    println!("{:?}", day3pt1::solve(include_str!("../input/day3pt1.txt")));
    println!("{}", day3pt2::solve(include_str!("../input/day3pt1.txt")),);
}

#[cfg(test)]
mod tests {
    use crate::{day1pt1, day1pt2, day2pt1, day2pt2, day3pt1, day3pt2};

    #[test]
    fn day1pt1_eq_example() {
        let input = include_str!("../example_input/day1pt1.txt");
        assert_eq!(day1pt1::solve(input), 24000);
    }

    #[test]
    fn day1pt2_eq_example() {
        let input = include_str!("../example_input/day1pt1.txt");
        assert_eq!(day1pt2::solve(input), 45000);
    }

    #[test]
    fn day2pt1_eq_example() {
        let input = include_str!("../example_input/day2pt1.txt");
        assert_eq!(day2pt1::solve(input), 15)
    }

    #[test]
    fn day2pt2_eq_example() {
        let input = include_str!("../example_input/day2pt1.txt");
        assert_eq!(day2pt2::solve(input), 12)
    }

    #[test]
    fn day3pt1_eq_example() {
        let input = include_str!("../example_input/day3pt1.txt");
        assert_eq!(day3pt1::solve(input), 157)
    }

    #[test]
    fn day3pt2_eq_example() {
        let input = include_str!("../example_input/day3pt1.txt");
        assert_eq!(day3pt2::solve(input), 70)
    }
}
