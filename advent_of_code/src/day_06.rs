use crate::common;

pub fn part_01() {
    let input = common::read_file("inputs/input_06").unwrap();
    let total: usize = input.lines()
        .map(parse_line)
        .array_chunks()
        .flat_map(|[t, d]| t.zip(d))
        .map(|x| ways_to_win(x.0, x.1))
        .fold(1, |acc, x| acc * x);
    println!("Day 6 : Part 1 : {}", total);
}

pub fn part_02() {
    let input = common::read_file("inputs/input_06").unwrap();
    let mut lines = input.lines();
    let time = parse_line_02(lines.next().unwrap());
    let distance = parse_line_02(lines.next().unwrap());
    let ways = ways_to_win(time, distance);
    println!("Day 6 : Part 2 : {}", ways);
}

fn parse_line(line: &str) -> impl Iterator<Item=i64> + '_ {
    line.split_ascii_whitespace()
        .skip(1)
        .map(|x| x.parse::<i64>().unwrap())
}

fn parse_line_02(line: &str) -> i64 {
    line.split_ascii_whitespace()
        .skip(1)
        .collect::<String>().parse().unwrap()
}

fn ways_to_win(time: i64, distance: i64) -> usize {
    (0..time)
        .take_while(|t| t * (time - t) <= distance)
        .last()
        .map(|count| time - count * 2)
        .unwrap_or(0) as usize
}
