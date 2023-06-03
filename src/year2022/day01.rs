use crate::solver::{Solver, Solvers};

pub fn part_one(input: String) -> u64 {
    println!("{}", input);
    0
}

pub fn part_two(input: String) -> u64 {
    println!("part two");
    0
}

pub fn get_solvers() -> Solvers {
    Solvers::new(
        Solver::new("inputs/2022/day01.txt", part_one),
        Solver::new("inputs/2022/day01.txt", part_two),
    )
}
