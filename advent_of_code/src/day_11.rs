use crate::common::Point;

pub fn part_01() {
    let total = solve(2);
    println!("Day 11 : Part 1 : {}", total)
}

pub fn part_02() {
    let total = solve(1_000_000);
    println!("Day 11 : Part 2 : {}", total)
}

fn solve(expansion_size: i64) -> u64 {
    let input = include_str!("../inputs/input_11");
    let galaxies = parse_input(input, expansion_size);
    let mut total = 0;
    for lhs_i in 0..galaxies.len() {
        let lhs = &galaxies[lhs_i];
        for rhs_i in lhs_i + 1..galaxies.len() {
            let rhs = &galaxies[rhs_i];
            let distance = lhs.x.abs_diff(rhs.x) + lhs.y.abs_diff(rhs.y);
            total += distance;
        }
    }
    total
}

fn parse_input(input: &str, expansion_size: i64) -> Vec<Point<i64>> {
    let width = input.lines().take(1).last().unwrap().len();
    let mut cols_mask = vec![false; width];
    let mut galaxies = Vec::new();
    let mut y = 0;
    input.lines().for_each(|l| {
        let mut found_galaxy = false;
        l.char_indices().for_each(|(x, c)| {
            if c == '#' {
                found_galaxy = true;
                cols_mask[x] = true;
                galaxies.push(Point::new(x as i64, y as i64));
            }
        });
        if found_galaxy {
            y += 1
        } else {
            y += expansion_size;
        }
    });
    let mut offsets = Vec::new();
    let mut count = 0;
    for b in cols_mask.iter() {
        if !b { count += expansion_size - 1 }
        offsets.push(count)
    }
    galaxies.iter_mut().for_each(|g| g.x += offsets[g.x as usize]);
    galaxies
}
