use std::sync::atomic::{AtomicUsize, Ordering};
use rayon::prelude::*;

pub fn part_01() {
    let input = include_str!("../inputs/input_12");
    let totals: Vec<_> = input.lines()
        .map(|l| solutions(l))
        .collect();
    let total: usize = totals.iter().map(|x| x.0).sum();
    let steps: usize = totals.iter().map(|x| x.1).sum();
    println!("Day 12 : Part 1 : {} STEPS: {}", total, steps);
}

pub fn part_02() {
    let input = include_str!("../inputs/input_12");
    let inputs: Vec<_> = input.lines()
        .map(|l| {
            let parts: Vec<_> = l.split_ascii_whitespace().collect();
            let mut new_0 = String::from(parts[0]);
            let mut new_1 = String::from(parts[1]);
            for _ in 0..4 {
                new_0.push('?');
                new_0.push_str(parts[0]);
                new_1.push(',');
                new_1.push_str(parts[1]);
            }
            new_0 + " " + new_1.as_str()
        })
        .enumerate()
        .collect();

    let totals: Vec<_> = inputs
        .iter()
        .map(|l| solutions(&l.1))
        .collect();
    let total: usize = totals.iter().map(|x| x.0).sum();
    let steps: usize = totals.iter().map(|x| x.1).sum();
    println!("Day 12 : Part 2 : {} STEPS: {}", total, steps);
}

fn solutions(input: &str) -> (usize, usize) {
    let parts: Vec<_> = input.split_ascii_whitespace().collect();
    let mut unknowns = Vec::new();
    let mut chunks: Vec<_> = parts[1]
        .split(",")
        .map(|x| x.parse::<usize>().unwrap())
        .collect::<Vec<_>>()
        .iter()
        .rev()
        .map(|x| *x)
        .collect();
    let mut row: Vec<_> = parts[0]
        .chars()
        .inspect(|c| if *c == '?' { unknowns.push(*c) })
        .collect();
    let mut steps = 0;
    let mut ucount = unknowns.len();
    let mut memos = Vec::new();
    let sols = step(0, &mut row, &mut chunks, &mut memos, &mut steps);

    (sols, steps)
}

// Used validate to identify bad solutions that were worth looking at
// Found out I had a bad edge case when the last char was a #.
fn validate(sol: &str, chunks: &Vec<usize>) -> bool {
    let mut groups = Vec::new();
    let mut last_was_broken = false;
    for s in sol.chars() {
        if s == '#' {
            if !last_was_broken {
                groups.push(0);
            }
            *groups.last_mut().unwrap() += 1;
            last_was_broken = true;
        } else {
            last_was_broken = false;
        }
    }

    if groups.len() != chunks.len() {
        println!("LENGTHS DONT MATCH {:?} {:?}", groups, chunks);
        return false;
    }
    for i in 0..groups.len() {
        if groups[i] != chunks[groups.len() - 1 - i] {
            println!("Chunks do not match {:?} {:?}", groups, chunks);
            return false;
        }
    }
    return true;
}

fn step(test: usize, row: &mut Vec<char>, chunks: &mut Vec<usize>, memo: &mut Vec<(usize, usize, usize)>, steps: &mut usize) -> usize {
    for m in memo.iter() {
        if m.0 == test && m.1 == chunks.len() {
            return m.2;
        }
    }

    let c_size = match chunks.pop() {
        None => {
            if row.iter().skip(test).all(|c| *c != '#') {
                memo.push((test, 0, 1));
                return 1;
                // solutions.push(String::from_iter(row.iter()));
                // println!("Solution: {:?} {}", row.iter().collect::<String>(), test);
            }
            memo.push((test, 0, 0));
            return 0;
        }
        Some(e) => e
    };

    let min_l = c_size + chunks.iter().sum::<usize>() + chunks.len();
    if test + min_l > row.len() {
        // print!("H");
        chunks.push(c_size);
        memo.push((test, chunks.len() + 1, 0));
        return 0;
    }

    let mut solutions = 0;
    let mut i = test;
    while i < row.len() {
        *steps += 1;
        if let Some(updates) = test_chunk(i, row, c_size) {
            let new_solutions = step(i + c_size + 1, row, chunks, memo, steps);
            solutions += new_solutions;
            updates.iter().for_each(|i| row[*i] = '?');
        }

        if row[i] == '#' {
            // chunk size has to be used here, no need to continue looping
            break;
        }
        i += 1;
    }

    chunks.push(c_size);
    memo.push((test, chunks.len(), solutions));
    solutions
}

fn test_chunk(i: usize, row: &mut Vec<char>, c_size: usize) -> Option<Vec<usize>> {
    if row[i] == '.' ||
        i + c_size > row.len() {
        return None;
    }

    let mut updates = Vec::new();

    // If there will be space after chunk
    if i + c_size < row.len() {
        let t = i + c_size;
        match row[t] {
            '.' => {}
            '#' => return None,
            '?' => {
                updates.push(t);
                row[t] = '.';
            }
            _ => panic!("why"),
        }
    }

    let mut is_bad = false;
    for test in i..i + c_size {
        let c = row[test];
        match c {
            '.' => {
                is_bad = true;
                break;
            }
            '#' => {}
            '?' => {
                updates.push(test);
                row[test] = '#';
            }
            _ => panic!("why"),
        };
    };

    if is_bad {
        for i in updates.iter() {
            row[*i] = '?';
        }
        return None;
    }
    return Some(updates);
}