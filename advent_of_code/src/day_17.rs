use std::collections::{HashMap, HashSet};
use crate::common::Point;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
enum Dir {
    Left,
    Right,
    Up,
    Down,
}

pub fn part_01() {
    let input = include_str!("../inputs/input_17_test");
    let grid = input.lines()
        .map(|l| l.chars().map(|c| c.to_digit(10).unwrap()).collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let path = minimize_heat(&grid,
                              Point::new(11, 9),
                              Dir::Down,
                              Point::new((grid[0].len() - 1) as i32, (grid.len() - 1) as i32));
    let mut display = vec![vec!['.'; grid[0].len()]; grid.len()];
    for p in &path {
        println!("{:?}", p);
        display[p.y as usize][p.x as usize] = '#';
    }

    for i in 0..grid.len() {
        println!("{}   {}", grid[i].iter().map(|x| x.to_string()).collect::<String>(), display[i].iter().collect::<String>());
    }

    let total = path.iter()
        .map(|p| grid[p.y as usize][p.x as usize])
        .sum::<u32>();
    println!("Day 17 : Part 1 : {}", total);
}

const MAX_STEPS: i32 = 3;

fn dist(lhs: Point<i32>, rhs: Point<i32>) -> i32 {
    (rhs.x - lhs.x).abs() + (rhs.y - lhs.y).abs()
}

fn minimize_heat(grid: &Vec<Vec<u32>>, start: Point<i32>, start_dir: Dir, end: Point<i32>) -> Vec<Point<i32>> {
    let mut open_set: HashSet<(Point<i32>, Dir)> = HashSet::new();
    open_set.insert((start, start_dir));
    let mut came_from: HashMap<Point<i32>, Point<i32>> = HashMap::new();
    let mut g_scores: HashMap<(Point<i32>, Dir, i32), i32> = HashMap::new();
    g_scores.insert((start, start_dir, MAX_STEPS), 0);

    let mut f_scores = HashMap::new();
    f_scores.insert((start, start_dir, MAX_STEPS), dist(end, start));

    while !open_set.is_empty() {
        let curr_node = open_set.iter()
            .min_by_key(|n| f_scores.get(n).copied().unwrap_or(i32::MAX))
            .unwrap()
            .clone();
        let (cur_loc, cur_dir, cur_step) = curr_node;

        if cur_loc == end {
            let mut path = Vec::new();
            path.push(cur_loc);

            let mut curr = &cur_loc;
            while let Some(next) = came_from.get(&curr) {
                curr = next;
                path.push(*next)
            }
            return path;
        }

        open_set.remove(&curr_node);
        // FIGURING OUT HOW TO GENERATE LIST OF NEIGHBORS
        let opposite_dir = match cur_dir {
            Dir::Left => Dir::Right,
            Dir::Right => Dir::Left,
            Dir::Up => Dir::Down,
            Dir::Down => Dir::Up,
        };

        for (dir, off_x, off_y) in [(Dir::Left, -1i32, 0i32), (Dir::Right, 1, 0), (Dir::Up, 0, -1), (Dir::Down, 0, 1)] {
            if dir == cur_dir {
                if cur_step > 0 {
                    // ISSUE?: gScores doesn't account for dir or steps
                    let neighbor = cur_loc.offset(off_x, off_y);
                    // let tentative_g_score = g_scores.get(&cur_loc).copied().unwrap_or(i32::MAX) + score(&grid, neighbor);
                    let tentative_g_score = match g_scores.get(&curr_node).copied() {
                        None => continue,
                        Some(g) => match score(&grid, neighbor) {
                            None => continue,
                            Some(s) => g + s
                        }
                    };
                    let neighbor_node = (neighbor, dir, cur_step - 1);
                    if tentative_g_score < g_scores.get(&neighbor_node).copied().unwrap_or(i32::MAX) {
                        came_from.insert(neighbor, cur_loc);
                        g_scores.insert(neighbor_node, tentative_g_score);
                        f_scores.insert(neighbor_node, tentative_g_score + dist(neighbor, end));
                        open_set.insert(neighbor_node);
                    }
                }
            } else if dir != opposite_dir {
                let neighbor = cur_loc.offset(off_x, off_y);
                let tentative_g_score = match g_scores.get(&curr_node).copied() {
                    None => continue,
                    Some(g) => match score(&grid, neighbor) {
                        None => continue,
                        Some(s) => g + s
                    }
                };
                let neighbor_node = (neighbor, dir, MAX_STEPS);
                if tentative_g_score < g_scores.get(&neighbor_node).copied().unwrap_or(i32::MAX) {
                    came_from.insert(neighbor, cur_loc);
                    g_scores.insert(neighbor_node, tentative_g_score);
                    f_scores.insert(neighbor_node, tentative_g_score + dist(neighbor, end));
                    open_set.insert(neighbor_node);
                }
            }
        }
    }
    panic!("WHOOPS");
}

fn score(grid: &&Vec<Vec<u32>>, rhs: Point<i32>) -> Option<i32> {
    if rhs.x < 0 || rhs.x >= grid[0].len() as i32 || rhs.y < 0 || rhs.y >= grid.len() as i32 {
        return None
    }
    Some(grid[rhs.y as usize][rhs.x as usize] as i32)
}

pub fn part_02() {}
