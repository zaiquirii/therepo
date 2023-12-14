use std::collections::HashMap;
use std::hash::{DefaultHasher, Hash, Hasher};
use std::mem::swap;

#[derive(Copy, Clone)]
enum Dir {
    North,
    South,
    East,
    West,
}

pub fn part_01() {
    let input = include_str!("../inputs/input_14");
    let mut grid = Vec::new();
    let height = input.lines()
        .inspect(|l| {
            l.chars().for_each(|c| grid.push(c));
        })
        .count();
    let width = grid.len() / height;

    grid.chunks(width)
        .for_each(|c| println!("{}", c.iter().collect::<String>()));
    println!();

    move_rocks(Dir::North, &mut grid, height);

    grid.chunks(width)
        .for_each(|c| println!("{}", c.iter().collect::<String>()));

    let total = grid.chunks(width)
        .enumerate()
        .map(|(y, c)| (height - y) * c.iter().filter(|&x| *x == 'O').count())
        .sum::<usize>();

    println!("Day 14 : Part 1 : {}", total);
}

fn move_rocks(dir: Dir, grid: &mut Vec<char>, size: usize) {
    match dir {
        Dir::North => {}
        Dir::South => {}
        Dir::East => {}
        Dir::West => {}
    }

    'outer: for col in 0..size {
        let mut pivot = 0;
        let mut next = 1;
        'pivot: loop {
            // pivot = match match dir {
            //     Dir::North | Dir::West => (pivot..size).find(|p| grid[p * size + x] == '.'),
            //     Dir::South | Dir::East => (0..pivot).rev().find(|p| grid[p * size + x] == '.'),
            // } {
            //     None => continue 'outer,
            //     Some(p) => p
            // };
            // pivot = match (pivot..size).find(|p| grid[p * size + x] == '.') {
            //     None => continue 'outer,
            //     Some(p) => p
            // };
            pivot = match (pivot..size).find(|p| grid[index(dir, size, *p, col)] == '.') {
                None => continue 'outer,
                Some(p) => p
            };

            // pivot is now set
            for next in pivot + 1..size {
                let c_index = index(dir, size, next, col);
                let c = grid[c_index];
                if c == 'O' {
                    grid.swap(index(dir, size, pivot, col), c_index);
                    pivot += 1;
                    continue 'pivot;
                } else if c == '#' {
                    pivot = next + 1;
                    continue 'pivot;
                }
            }
            continue 'outer;
        }
    }
}

fn index(dir: Dir, size: usize, index: usize, col: usize) -> usize {
    match dir {
        Dir::North => index * size + col,
        Dir::South => (size - index - 1) * size + col,
        Dir::East => col * size + (size - index - 1),
        Dir::West => col * size + index,
    }
}

pub fn part_02() {
    let input = include_str!("../inputs/input_14");
    let mut grid = Vec::new();
    let height = input.lines()
        .inspect(|l| {
            l.chars().for_each(|c| grid.push(c));
        })
        .count();
    let width = grid.len() / height;

    grid.chunks(width)
        .for_each(|c| println!("{}", c.iter().collect::<String>()));
    println!();

    let mut grids: HashMap<u64, Vec<char>> = HashMap::new();
    let mut curr_hash = hashed(&grid);
    grids.insert(curr_hash, grid);

    let mut hashes: HashMap<u64, u64> = HashMap::new();
    let cycles = 1_000_000_000;
    for cycle in 0..cycles {
        if let Some(h) = hashes.get(&curr_hash) {
            curr_hash = *h;
        } else {
            let mut new_grid = grids.get(&curr_hash).unwrap().clone();
            move_rocks(Dir::North, &mut new_grid, height);
            move_rocks(Dir::West, &mut new_grid, height);
            move_rocks(Dir::South, &mut new_grid, height);
            move_rocks(Dir::East, &mut new_grid, height);
            let new_hash = hashed(&new_grid);
            grids.insert(new_hash, new_grid);
            hashes.insert(curr_hash, new_hash);
            curr_hash = new_hash;
        }

        if cycle % 10_000_000 == 0 {
            println!("{} {} {}", cycle / 10_000_000, hashes.len(), curr_hash);
        }
    }

    let total = grids.get(&curr_hash).unwrap()
        .chunks(width)
        .enumerate()
        .map(|(y, c)| (height - y) * c.iter().filter(|&x| *x == 'O').count())
        .sum::<usize>();

    println!("Day 14 : Part 2 : {}", total);
}

fn hashed(vec: &Vec<char>) -> u64 {
    let mut hasher = DefaultHasher::new();
    vec.hash(&mut hasher);
    hasher.finish()
}