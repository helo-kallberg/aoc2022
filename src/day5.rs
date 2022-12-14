use std::{collections::VecDeque, vec};

pub fn read_crate_line(line: &str) -> Vec<Option<char>> {
    let char_vector: Vec<char> = line.chars().collect();
    let chunks = char_vector.chunks(4);

    let mut values = vec![];

    for chunk in chunks {
        if chunk[0] == '[' {
            values.push(Some(chunk[1]));
        } else {
            values.push(None)
        }
    }

    values
}

pub fn read_crates(input: &str) -> Vec<VecDeque<char>> {
    let mut crate_lines: Vec<Vec<Option<char>>> = vec![];
    let mut number_of_crates = 0;

    for line in input.lines() {
        if line.starts_with(" 1") {
            number_of_crates = line
                .split(' ')
                .last()
                .expect("read last crate number")
                .parse()
                .expect("parse last crate number");

            break;
        }

        crate_lines.push(read_crate_line(line))
    }

    let mut crates: Vec<VecDeque<char>> = Vec::with_capacity(number_of_crates);

    for index in 0..number_of_crates {
        let mut crate_vector: VecDeque<char> = VecDeque::new();

        for crate_line in &crate_lines {
            if let Some(Some(value)) = crate_line.get(index) {
                crate_vector.push_back(*value);
            }
        }

        crates.push(crate_vector);
    }

    crates
}

pub fn read_moves(input: &str) -> Vec<(usize, usize, usize)> {
    let mut output = vec![];

    for line in input.lines() {
        if !line.starts_with("move") {
            continue;
        }

        let line = line.strip_prefix("move ").expect("strip prefix");
        let (move_count_str, rest) = line.split_once(' ').expect("split move count");
        let move_count: usize = move_count_str.parse().expect("parse move count");
        let line = rest.strip_prefix("from ").expect("split prefix");
        let (move_from_str, rest) = line.split_once(' ').expect("split move to");
        let move_from: usize = move_from_str.parse().expect("parse move to");
        let line = rest.strip_prefix("to ").expect("strip prefix");
        let move_to: usize = line.parse().expect("parse move to");

        output.push((move_count, move_from, move_to))
    }

    output
}

pub fn perform_move(
    crates: Vec<VecDeque<char>>,
    move_instruction: (usize, usize, usize),
) -> Vec<VecDeque<char>> {
    let (count, from, to) = move_instruction;
    let mut from_crate = crates[from - 1].clone();
    let mut to_crate = crates[to - 1].clone();

    for _ in 0..count {
        let value = from_crate.pop_front().expect("crate contents");
        to_crate.push_front(value);
    }

    let mut new = vec![];

    for (index, orig_crate) in crates.into_iter().enumerate() {
        if index != (from - 1) && index != (to - 1) {
            new.push(orig_crate);
        } else if index == (from - 1) {
            new.push(from_crate.clone())
        } else if index == (to - 1) {
            new.push(to_crate.clone())
        }
    }

    new
}

pub fn crates_front_string(crates: Vec<VecDeque<char>>) -> String {
    let mut output = String::new();

    for crate_deque in crates {
        let top: Option<&char> = crate_deque.front();

        if let Some(top) = top {
            output += top.to_string().as_str();
        }
    }

    output
}

pub fn solve_1(input: &str) -> String {
    let mut crates = read_crates(input);
    let move_instructions = read_moves(input);

    for move_instruction in move_instructions {
        crates = perform_move(crates, move_instruction);
    }

    crates_front_string(crates)
}

pub fn perform_move_2(
    crates: Vec<VecDeque<char>>,
    move_instruction: (usize, usize, usize),
) -> Vec<VecDeque<char>> {
    let (count, from, to) = move_instruction;
    let mut from_crate = crates[from - 1].clone();
    let mut to_crate = crates[to - 1].clone();

    let mut to_move: Vec<char> = vec![];

    for _ in 0..count {
        let value = from_crate.pop_front().expect("crate contents");
        to_move.push(value);
    }

    to_move.reverse();

    for item in to_move {
        to_crate.push_front(item)
    }

    let mut new = vec![];

    for (index, orig_crate) in crates.into_iter().enumerate() {
        if index != (from - 1) && index != (to - 1) {
            new.push(orig_crate);
        } else if index == (from - 1) {
            new.push(from_crate.clone())
        } else if index == (to - 1) {
            new.push(to_crate.clone())
        }
    }

    new
}

pub fn solve_2(input: &str) -> String {
    let mut crates = read_crates(input);
    let move_instructions = read_moves(input);

    for move_instruction in move_instructions {
        crates = perform_move_2(crates, move_instruction);
    }

    crates_front_string(crates)
}
