mod edition;
mod year2022;

pub mod solver;

use crate::edition::Edition;
use crate::solver::Solvers;
use std::env;

fn get_edition(year: usize) -> Option<Edition> {
    match year {
        2022 => Some(year2022::get_edition()),
        _ => None,
    }
}

fn get_day(edition: Edition, day: u8) -> Option<Solvers> {
    match day {
        1 => edition.day01,
        2 => edition.day02,
        3 => edition.day03,
        4 => edition.day04,
        5 => edition.day05,
        6 => edition.day06,
        7 => edition.day07,
        8 => edition.day08,
        9 => edition.day09,
        10 => edition.day10,
        11 => edition.day11,
        12 => edition.day12,
        13 => edition.day13,
        14 => edition.day14,
        15 => edition.day15,
        16 => edition.day16,
        17 => edition.day17,
        18 => edition.day18,
        19 => edition.day19,
        20 => edition.day20,
        21 => edition.day21,
        22 => edition.day22,
        23 => edition.day23,
        24 => edition.day24,
        25 => edition.day25,
        26 => edition.day26,
        27 => edition.day27,
        28 => edition.day28,
        29 => edition.day29,
        30 => edition.day30,
        31 => edition.day31,
        _ => None,
    }
}

fn main() {
    let year = env::args()
        .nth(1)
        .expect("no year specified")
        .parse::<usize>()
        .expect("unable to parse the year value");
    let day = env::args()
        .nth(2)
        .expect("no day specified")
        .parse::<u8>()
        .expect("unable to parse the day value");
    let part = env::args()
        .nth(3)
        .expect("no part specified")
        .parse::<u8>()
        .expect("unable te parse the part value");

    let edition = get_edition(year).expect("nothing found for the edition");
    let day = get_day(edition, day).expect("day not implemented");

    let result = match part {
        1 => day.part_one.call(),
        2 => day.part_two.call(),
        _ => panic!("invalid part"),
    };

    println!("{}", result.expect("unable to run func"));
}
