pub fn part_01() {
    let input = include_str!("../inputs/input_09");
    let total: i64 = input.lines()
        .map(|x| x
            .split_ascii_whitespace()
            .map(|t| t.parse::<i64>().unwrap())
            .collect::<Vec<_>>())
        .map(|d| extrapolate(d))
        .sum();
    println!("Day 09 : Part 1 : {}", total)
}

fn extrapolate(values: Vec<i64>) -> i64 {
    let mut diffs = vec![values];
    let mut all_zeroes = false;
    while !all_zeroes {
        all_zeroes = true;
        let mut new_diff = Vec::new();
        let working = diffs.last_mut().unwrap();
        for i in 0..working.len() - 1 {
            let x = working[i + 1] - working[i];
            new_diff.push(x);
            if x != 0 {
                all_zeroes = false;
            }
        }
        diffs.push(new_diff);
    }

    diffs.iter()
        .map(|d| d.last().unwrap())
        .sum()
}

pub fn part_02() {
    let input = include_str!("../inputs/input_09");
    let total: i64 = input.lines()
        .map(|x| x
            .split_ascii_whitespace()
            .map(|t| t.parse::<i64>().unwrap())
            .rev()
            .collect::<Vec<_>>())
        .map(|d| extrapolate_first(d))
        .sum();
    println!("Day 09 : Part 1 : {}", total)
}

fn extrapolate_first(values: Vec<i64>) -> i64 {
    let mut diffs = vec![values];
    let mut all_zeroes = false;
    while !all_zeroes {
        all_zeroes = true;
        let mut new_diff = Vec::new();
        let working = diffs.last_mut().unwrap();
        for i in 0..working.len() - 1 {
            let x = working[i] - working[i + 1];
            new_diff.push(x);
            if x != 0 {
                all_zeroes = false;
            }
        }
        diffs.push(new_diff);
    }

    diffs.iter()
        .rev()
        .map(|d| d.last().unwrap())
        .fold(0, |acc, x| x - acc)
}
