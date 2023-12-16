use std::collections::HashSet;
use crate::common::Point;

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
enum Dir {
    Left,
    Right,
    Up,
    Down,
}

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
struct Beam {
    loc: Point<i32>,
    dir: Dir,
}

impl Beam {
    fn next(&self, grid: &Vec<Vec<char>>) -> Vec<Beam> {
        let mut results = Vec::<Beam>::new();
        let curr_tile = self.loc;
        let next_tile = match self.dir {
            Dir::Left => self.loc.add(Point::new(-1, 0)),
            Dir::Right => self.loc.add(Point::new(1, 0)),
            Dir::Up => self.loc.add(Point::new(0, -1)),
            Dir::Down => self.loc.add(Point::new(0, 1)),
        };

        if curr_tile.x < 0
            || curr_tile.x as usize >= grid[0].len()
            || curr_tile.y < 0
            || curr_tile.y as usize >= grid.len() {
            return results;
        }

        match grid[curr_tile.y as usize][curr_tile.x as usize] {
            '.' => {
                results.push(Beam { loc: next_tile, dir: self.dir })
            }
            '/' => match self.dir {
                Dir::Left => results.push(Beam { loc: curr_tile.offset(0, 1), dir: Dir::Down }),
                Dir::Right => results.push(Beam { loc: curr_tile.offset(0, -1), dir: Dir::Up }),
                Dir::Up => results.push(Beam { loc: curr_tile.offset(1, 0), dir: Dir::Right }),
                Dir::Down => results.push(Beam { loc: curr_tile.offset(-1, 0), dir: Dir::Left }),
            },
            '\\' => match self.dir {
                Dir::Left => results.push(Beam { loc: curr_tile.offset(0, -1), dir: Dir::Up }),
                Dir::Right => results.push(Beam { loc: curr_tile.offset(0, 1), dir: Dir::Down }),
                Dir::Up => results.push(Beam { loc: curr_tile.offset(-1, 0), dir: Dir::Left }),
                Dir::Down => results.push(Beam { loc: curr_tile.offset(1, 0), dir: Dir::Right }),
            },
            '|' => match self.dir {
                Dir::Left | Dir::Right => {
                    results.push(Beam { loc: curr_tile.offset(0, -1), dir: Dir::Up });
                    results.push(Beam { loc: curr_tile.offset(0, 1), dir: Dir::Down });
                }
                dir => {
                    results.push(Beam { loc: next_tile, dir });
                }
            },
            '-' => match self.dir {
                Dir::Up | Dir::Down => {
                    results.push(Beam { loc: curr_tile.offset(-1, 0), dir: Dir::Left });
                    results.push(Beam { loc: curr_tile.offset(1, 0), dir: Dir::Right });
                }
                dir => {
                    results.push(Beam { loc: next_tile, dir });
                }
            },
            _ => panic!("Shouldn't happen")
        }
        results
    }
}

fn trace_beam(grid: &Vec<Vec<char>>, start_beam: Beam) -> usize {
    let mut grid_status = vec![false; grid.len() * grid[0].len()];
    let mut past_beams = HashSet::new();
    let mut beams = vec![start_beam];
    while !beams.is_empty() {
        let curr = beams.pop().unwrap();
        if past_beams.contains(&curr) {
            continue;
        }
        let new_beams = curr.next(&grid);
        if !new_beams.is_empty() {
            grid_status[curr.loc.y as usize * grid[0].len() + curr.loc.x as usize] = true;
            past_beams.insert(curr);
        }
        for beam in new_beams {
            beams.push(beam);
        }
    }
    let total = grid_status.iter()
        .filter(|x| **x)
        .count();
    total
}

pub fn part_01() {
    let input = include_str!("../inputs/input_16");
    let grid = input.lines()
        .map(|l| l.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let total = trace_beam(&grid, Beam { loc: Point::new(0, 0), dir: Dir::Right });
    println!("Day 16 : Part 1 : {}", total)
}

pub fn part_02() {
    let input = include_str!("../inputs/input_16");
    let grid = input.lines()
        .map(|l| l.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let top = (0..grid[0].len())
        .map(|i| trace_beam(&grid, Beam { loc: Point::new(i as i32, 0), dir: Dir::Down }))
        .max()
        .unwrap();
    let bottom = (0..grid[0].len())
        .map(|i| trace_beam(&grid, Beam { loc: Point::new(i as i32, (grid.len() - 1) as i32), dir: Dir::Up }))
        .max()
        .unwrap();
    let left = (0..grid[0].len())
        .map(|i| trace_beam(&grid, Beam { loc: Point::new(0, i as i32), dir: Dir::Right }))
        .max()
        .unwrap();
    let right = (0..grid[0].len())
        .map(|i| trace_beam(&grid, Beam { loc: Point::new((grid[0].len() - 1) as i32, i as i32), dir: Dir::Left }))
        .max()
        .unwrap();

    let most = top.max(bottom).max(left).max(right);
    println!("Day 16 : Part 2 : {}", most)
}