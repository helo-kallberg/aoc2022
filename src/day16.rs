use std::{cmp::min, collections::HashMap, hash::Hash};

type SolutionCache = HashMap<SolverInput, usize>;

#[derive(Clone)]
pub struct Valve {
    pub id: usize,
    pub name: String,
    pub flow_rate: usize,
}

#[derive(Clone)]
pub struct ValveSystem {
    pub size: usize,
    pub names: Vec<String>,
    pub flow_rates: Vec<usize>,
    pub distances: Vec<Vec<usize>>,
}

#[derive(PartialEq, Eq, Hash, Clone)]
pub struct SolverInput {
    pub position: usize,
    pub time_left: usize,
    pub visited_mask: u64,
}

pub struct ValveSystemSolver {
    pub system: ValveSystem,
    pub cache: SolutionCache,
}

impl ValveSystemSolver {
    fn solve_by_input(&mut self, input: SolverInput) -> usize {
        if let Some(cached) = self.cache.get(&input) {
            return *cached;
        }

        let mut best_score = 0;

        for index in 0..self.system.size {
            let visit_bit = 1u64 << index;

            if input.visited_mask & visit_bit != 0 {
                continue;
            }

            let flow_rate = self.system.flow_rates[index];

            if flow_rate == 0 {
                continue;
            }

            let distance = self.system.distances[input.position][index];
            let time_spent = distance + 1;

            if time_spent >= input.time_left {
                continue;
            }

            let time_left = input.time_left - time_spent;

            let mut score = flow_rate * time_left;

            if time_left > 2 {
                let mut solver_next = input.clone();

                solver_next.visited_mask |= visit_bit;
                solver_next.time_left = time_left;
                solver_next.position = index;

                score += self.solve_by_input(solver_next);
            }

            if score > best_score {
                best_score = score;
            }
        }

        self.cache.insert(input, best_score);

        best_score
    }

    pub fn solve(&mut self) -> usize {
        self.system = self.system.as_simplified();

        self.solve_by_input(SolverInput {
            position: self.system.start(),
            time_left: 30,
            visited_mask: 0,
        })
    }

    fn elephant_solve_by_input(&mut self, input: SolverInput) -> usize {
        let mask = (1u64 << self.system.size) - 1;

        let halfway = (mask + 1) / 2;

        let mut best_score = 0;

        for index in 0..halfway {
            let mut regular = input.clone();
            let mut elephant = input.clone();

            elephant.visited_mask = mask ^ index;
            regular.visited_mask = index;

            let score = self.solve_by_input(regular) + self.solve_by_input(elephant);

            if score > best_score {
                best_score = score;
            }
        }

        best_score
    }

    pub fn elephant_solve(&mut self) -> usize {
        self.system = self.system.as_simplified();

        self.elephant_solve_by_input(SolverInput {
            position: self.system.start(),
            time_left: 26,
            visited_mask: 0,
        })
    }
}

impl ValveSystem {
    fn as_simplified(&self) -> Self {
        let start = self.start();

        let mut output = Self {
            distances: vec![],
            names: vec![],
            size: 0,
            flow_rates: vec![],
        };

        for index in 0..self.size {
            if self.flow_rates[index] == 0 && index != start {
                continue;
            }

            output.names.push(self.names[index].clone());
            output.flow_rates.push(self.flow_rates[index]);
            output.size += 1;

            let mut distance_row = vec![];

            for to in 0..self.size {
                if self.flow_rates[to] == 0 && to != start {
                    continue;
                }

                distance_row.push(self.distances[index][to])
            }

            output.distances.push(distance_row);
        }

        output
    }

    fn start(&self) -> usize {
        let mut start = 0;

        let start_name = "AA".to_string();

        for name in &self.names {
            if start_name.eq(name) {
                break;
            }

            start += 1;
        }

        start
    }
}

impl From<&str> for ValveSystem {
    fn from(value: &str) -> Self {
        let mut output = Self {
            size: 0,
            names: vec![],
            distances: vec![],
            flow_rates: vec![],
        };

        let mut neighbours = vec![];

        value.lines().for_each(|line| {
            let line = line.strip_prefix("Valve ").unwrap();
            let (valve_name, line) = line.split_once(' ').unwrap();
            let (flow_rate_str, line) = line
                .strip_prefix("has flow rate=")
                .unwrap()
                .split_once(';')
                .unwrap();
            let (_, line) = line.split_once("valve").unwrap().1.split_once(' ').unwrap();

            let mut connections: Vec<String> =
                line.split(", ").map(|str| str.to_string()).collect();
            if connections.is_empty() {
                connections = vec![line.to_string()];
            }
            let valve_name = valve_name.to_string();

            let flow_rate = flow_rate_str.parse().unwrap();

            output.names.push(valve_name);
            output.flow_rates.push(flow_rate);
            output.size += 1;

            neighbours.push(connections);
        });

        let mut distances = vec![vec![None; output.size]; output.size];

        for (index, neighbours) in neighbours.iter().cloned().enumerate() {
            for neighbour in neighbours {
                let Some((valve_id, _)) = output
                    .names
                    .clone()
                    .into_iter()
                    .enumerate()
                    .find(|(_, name)| neighbour.eq(name)) else {
                        continue;
                    };

                distances[index][valve_id] = Some(1usize);
            }
        }

        for identity in 0..distances.len() {
            distances[identity][identity] = Some(0);
        }

        let distances = floyd_warshall(&distances);
        let distances: Vec<Vec<usize>> = distances
            .iter()
            .map(|row| {
                row.iter()
                    .map(|dist| dist.expect("distance to all nodes known"))
                    .collect()
            })
            .collect();

        output.distances = distances;

        output
    }
}

fn floyd_warshall(graph: &Vec<Vec<Option<usize>>>) -> Vec<Vec<Option<usize>>> {
    let n = graph.len();
    let mut distance = graph.clone();

    for k in 0..n {
        for i in 0..n {
            for j in 0..n {
                if let (Some(x), Some(y)) = (distance[i][k], distance[k][j]) {
                    if let Some(z) = distance[i][j] {
                        distance[i][j] = Some(min(z, x + y));
                    } else {
                        distance[i][j] = Some(x + y);
                    }
                }
            }
        }
    }

    distance
}

pub fn solve_1(input: &str) -> String {
    let mut solver = ValveSystemSolver {
        cache: HashMap::new(),
        system: ValveSystem::from(input),
    };

    solver.solve().to_string()
}
pub fn solve_2(input: &str) -> String {
    let mut solver = ValveSystemSolver {
        cache: HashMap::new(),
        system: ValveSystem::from(input),
    };

    solver.elephant_solve().to_string()
}
