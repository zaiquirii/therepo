use std::mem::swap;

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

    'outer: for x in 0..width {
        let mut pivot = 0;
        'pivot: loop {
            if pivot >= height {
                continue 'outer;
            }

            while grid[pivot * width + x] != '.' {
                pivot += 1;
                if pivot >= height {
                    continue 'outer;
                }
            }

            // pivot is now set
            for next in pivot + 1..width {
                let c = grid[next * width + x];
                if c == 'O' {
                    grid.swap(pivot * width + x, next * width + x);
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

    grid.chunks(width)
        .for_each(|c| println!("{}", c.iter().collect::<String>()));

    let total = grid.chunks(width)
        .enumerate()
        .map(|(y, c)| (height - y) * c.iter().filter(|&x| *x == 'O').count())
        .sum::<usize>();
    println!("Day 14 : Part 1 : {}", total);
}

pub fn part_02() {}
