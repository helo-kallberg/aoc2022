use std::collections::HashSet;

pub fn solve_1(input: &str) -> String {
    let found = input
        .chars()
        .collect::<Vec<char>>()
        .windows(4)
        .map(|chars| {
            let mut set: HashSet<char> = HashSet::new();

            chars.iter().for_each(|char| {
                set.insert(*char);
            });

            set
        })
        .enumerate()
        .find(|(_, set)| set.len() == 4)
        .expect("solution");

    (found.0 + 4).to_string()
}

pub fn solve_2(input: &str) -> String {
    let found = input
        .chars()
        .collect::<Vec<char>>()
        .windows(14)
        .map(|chars| {
            let mut set = HashSet::<char>::new();

            for char in chars {
                set.insert(*char);
            }

            set
        })
        .enumerate()
        .find(|(_, set)| set.len() == 14)
        .expect("solution");

    (found.0 + 14).to_string()
}
