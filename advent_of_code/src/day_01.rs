use std::fs::File;
use std::io::Read;

pub fn day_01_part1() {
    let mut file = File::open("inputs/input_01").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let res : i32 = contents
        .lines()
        .map(|l| {
            let mut n = String::new();
            n.push(l.chars().find(|x| x.is_numeric()).unwrap());
            n.push(l.chars().rfind(|x| x.is_numeric()).unwrap());
            n.parse::<i32>().unwrap()
        })
        .sum();
    println!("Day 1 : Part 1 : {}", res)
}

fn tokenize(s: &str) -> Vec<i32> {
    let tokens = [
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
        ("1", 1),
        ("2", 2),
        ("3", 3),
        ("4", 4),
        ("5", 5),
        ("6", 6),
        ("7", 7),
        ("8", 8),
        ("9", 9),
    ];
    let mut res = Vec::new();
    for i in 0..s.len() {
        let substr = s.chars().skip(i).collect::<String>();
        let a = tokens.iter().find(|(t, _)| substr.starts_with(t));
        match a {
            Some(x) => {
                res.push(x.1);
            }
            _ => ()
        }
    }
    res
}

pub fn day_01_part2() {
    let mut file = File::open("inputs/input_01").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let result : i32 = contents
        .lines()
        .map(tokenize)
        .map(|x| (x[0].to_string() + &x[x.len() - 1].to_string()).parse::<i32>().unwrap())
        .sum();
    println!("Day 1 : Part 2 : {}", result)
}
