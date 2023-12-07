#![feature(iter_array_chunks)]

mod day_01;
mod day_02;
mod common;
mod day_03;
mod day_04;
mod day_05;
mod day_06;
mod day_07;

fn main() {
    run_day(7, 2);
}

fn run_day(day: u8, part: u8) {
    match (day, part) {
        (1, 1) => { day_01::day_01_part1() }
        (1, 2) => { day_01::day_01_part2() }
        (2, 1) => { day_02::part_01() }
        (2, 2) => { day_02::part_02() }
        (3, 1) => { day_03::part_01() }
        (3, 2) => { day_03::part_02() }
        (4, 1) => { day_04::part_01() }
        (4, 2) => { day_04::part_02() }
        (5, 1) => { day_05::part_01() }
        (5, 2) => { day_05::part_02() }
        (6, 1) => { day_06::part_01() }
        (6, 2) => { day_06::part_02() }
        (7, 1) => { day_07::part_01() }
        (7, 2) => { day_07::part_02() }
        _ => { panic!("NOT IMPLEMENTED") }
    }
}

