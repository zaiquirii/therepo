mod day_01;
mod day_02;
mod common;
mod day_03;
mod day_04;

fn main() {
    run_day(4, 1);
}


fn run_day(day: u8, part: u8) {
    match (day, part) {
        (1, 1) => { day_01::day_01_part1() }
        (1, 2) => { day_01::day_01_part2() }
        (2, 1) => { day_02::part_01() }
        (2, 2) => { day_02::part_01() }
        (3, 1) => { day_03::part_01() }
        (3, 2) => { day_03::part_02() }
        (4, 1) => { day_04::part_01() }
        (4, 2) => { day_04::part_02() }
        _ => { panic!("NOT IMPLEMENTED") }
    }
}
