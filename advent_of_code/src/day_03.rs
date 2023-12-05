use regex::Regex;
use crate::common;

pub fn part_01() {
    let re = Regex::new(r"\d+").unwrap();
    let input = common::read_file("inputs/input_03").unwrap();
    let lines: Vec<_> = input
        .lines()
        .map(|l| l.chars().collect::<Vec<_>>())
        .collect();

    let total: u32 = input
        .lines()
        .enumerate()
        .flat_map(|(i, s)| {
            s.match_indices(&re)
                .map(move |m| (m.0, i, m.1))
        })
        .filter(|(x, y, m)| touches_symbol(*x, *y, m.len(), &lines))
        .map(|x| x.2.parse::<u32>().unwrap())
        .sum();

    println!("day 2 : part 1 : {}", total)
}

pub fn part_02() {
    let re = Regex::new(r"\d+").unwrap();
    let input = common::read_file("inputs/input_03").unwrap();
    let lines: Vec<_> = input
        .lines()
        .map(|l| l.chars().collect::<Vec<_>>())
        .collect();

    let number_squares: Vec<_> = input
        .lines()
        .enumerate()
        .flat_map(|(i, s)| {
            s.match_indices(&re)
                .map(move |m| (m.0, i, m.1))
        })
        .filter(|(x, y, m)| touches_symbol(*x, *y, m.len(), &lines))
        .map(|(x, y, m)| Square::new(x as i32, y as i32, m))
        .collect();

    let total: u32 = input
        .lines()
        .enumerate()
        .flat_map(|(i, s)| {
            s.match_indices("*")
                .map(move |m| (m.0, i))
        })
        .map(|(x, y)| {
            number_squares
                .iter()
                .filter(|s| s.contains(x as i32, y as i32))
                .collect::<Vec<_>>()
        })
        .filter(|i| i.len() == 2)
        .map(|i| i[0].value * i[1].value)
        .sum();

    println!("day 2 : part 2 : {}", total)
}

fn touches_symbol(mut x: usize, y: usize, length: usize, lines: &Vec<Vec<char>>) -> bool {
    let mut range = length + 2;
    if x == 0 {
        range -= 1
    } else {
        x -= 1
    }

    if x + range > lines[y].len() {
        range -= 1
    }

    for i in ((y as i32) - 1)..=((y as i32) + 1) {
        if i < 0 || i >= lines.len() as i32 {
            continue;
        }

        let l = &lines[i as usize];
        if chunk_has_symbol(&l[x..x + range]) {
            return true;
        }
    }
    false
}

fn chunk_has_symbol(chunk: &[char]) -> bool {
    chunk.iter().any(|c| !c.is_numeric() && *c != '.')
}

#[derive(Debug)]
struct Square {
    value: u32,
    l: i32,
    t: i32,
    b: i32,
    r: i32,
}

impl Square {
    fn new(x: i32, y: i32, value: &str) -> Self {
        Square {
            value: value.parse().unwrap(),
            l: x - 1,
            r: x + value.len() as i32,
            t: y - 1,
            b: y + 1,
        }
    }

    fn contains(&self, x: i32, y: i32) -> bool {
        (self.l <= x && self.r >= x) &&
            (self.t <= y && self.b >= y)
    }
}