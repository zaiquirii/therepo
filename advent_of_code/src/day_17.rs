use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};
use crate::common::Point;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
enum Dir {
    Left,
    Right,
    Up,
    Down,
}

impl Dir {
    fn opposite(&self) -> Self {
        match self {
            Dir::Left => Dir::Right,
            Dir::Right => Dir::Left,
            Dir::Up => Dir::Down,
            Dir::Down => Dir::Up,
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq)]
struct Container {
    cost: u32,
    dir: Dir,
    pos: Point<usize>,
    steps: u8,
}

impl PartialOrd for Container {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Container {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl Container {
    fn successors(&self, grid: &Vec<Vec<u32>>, min_steps: u8, max_steps: u8) -> Vec<Self> {
        let height = grid.len();
        let width = grid[0].len();

        let mut successors = Vec::new();
        for dir in [Dir::Left, Dir::Right, Dir::Up, Dir::Down] {
            if self.dir == dir && self.steps == max_steps {
                continue;
            }
            if self.dir.opposite() == dir {
                continue;
            }
            if self.dir != dir && self.steps < min_steps {
                continue;
            }

            if let Some(pos) = forward(self.pos, dir, width, height) {
                let cost = self.cost + grid[self.pos.y][self.pos.x];
                let steps = if self.dir == dir {
                    self.steps + 1
                } else {
                    1
                };
                successors.push(Container {
                    cost,
                    dir,
                    pos,
                    steps,
                })
            }
        }
        successors
    }
}

fn forward(loc: Point<usize>, dir: Dir, width: usize, height: usize) -> Option<Point<usize>> {
    let result = match dir {
        Dir::Left if loc.x > 0 => Point::new(loc.x - 1, loc.y),
        Dir::Right if loc.x < width - 1 => Point::new(loc.x + 1, loc.y),
        Dir::Up if loc.y > 0 => Point::new(loc.x, loc.y - 1),
        Dir::Down if loc.y < height - 1 => Point::new(loc.x, loc.y + 1),
        _ => return None
    };
    Some(result)
}

fn min_cost(grid: &Vec<Vec<u32>>, end: Point<usize>, min_steps: u8, max_steps: u8) -> (u32, Vec<(Point<usize>, Dir, u8)>) {
    let mut queue = BinaryHeap::new();
    let mut comes_from = HashMap::new();
    let mut dist = HashMap::new();
        let right = Container {
        cost: grid[0][1],
        dir: Dir::Right,
        pos: Point::new(1, 0),
        steps: 1,
    };
    queue.push(right);
    dist.insert((right.pos, right.dir, right.steps), right.cost);
    let down = Container {
        cost: grid[1][0],
        dir: Dir::Down,
        pos: Point::new(0, 1),
        steps: 1,
    };
    queue.push(down);
    dist.insert((down.pos, down.dir, down.steps), down.cost);
    while let Some(container) = queue.pop() {
        if container.pos == end && container.steps >= min_steps {
            let mut path = Vec::new();
            let mut curr = (container.pos, container.dir, container.steps);
            path.push(curr);
            while let Some(new) = comes_from.get(&curr) {
                path.push(*new);
                curr = *new;
                if curr.0 == Point::new(1, 0) || curr.0 == Point::new(0, 1) {
                    let mut t = 0;
                    for (i, p) in path.iter().rev().enumerate() {
                        t += grid[p.0.y][p.0.x];
                        println!("t {} {} {}", i, t, dist.get(p).copied().unwrap());
                    }

                    break;
                }
            }
            return (container.cost, path);
        }
        if let Some(c) = dist.get(&(container.pos, container.dir, container.steps)) {
            if container.cost > *c {
                continue;
            }
        }
        for successor in container.successors(grid, min_steps, max_steps) {
            let successor_node = (successor.pos, successor.dir, successor.steps);
            if successor.cost < dist.get(&successor_node).copied().unwrap_or(u32::MAX) {
                queue.push(successor);
                dist.insert(successor_node, successor.cost);
                comes_from.insert(successor_node, (container.pos, container.dir, container.steps));
            }
        }
    }
    panic!("WHOOP")
}

pub fn part_01() {
    let input = include_str!("../inputs/input_17_test");
    let grid = input.lines()
        .map(|l| l.chars().map(|c| c.to_digit(10).unwrap()).collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let (min, path) = min_cost(
        &grid,
        Point::new(grid[0].len() - 1, grid.len() - 1),
        1,
        3,
    );

    let mut display = vec![vec!['.'; grid[0].len()]; grid.len()];
    for p in path {
        display[p.0.y][p.0.x] = '#';
    }
    for i in 0..grid.len() {
        println!("{}  {}",
                 grid[i].iter().map(|x| x.to_string()).collect::<String>(),
                 display[i].iter().collect::<String>()
        );
    };
    println!("Day 17 : Part 1 : {}", min);
}

pub fn part_02() {
    let input = include_str!("../inputs/input_17_test");
    let grid = input.lines()
        .map(|l| l.chars().map(|c| c.to_digit(10).unwrap()).collect::<Vec<_>>())
        .collect::<Vec<_>>();

    // println!("Day 17 : Part 2 : {}", min);
    let (min, path) = min_cost(
        &grid,
        Point::new(grid[0].len() - 1, grid.len() - 1),
        4,
        10,
    );

    let mut display = vec![vec!['.'; grid[0].len()]; grid.len()];
    let mut sum = 0;
    for p in &path {
        display[p.0.y][p.0.x] = '#';
        sum += grid[p.0.y][p.0.x]
    }

    for i in 0..grid.len() {
        println!("{}  {}",
                 grid[i].iter().map(|x| x.to_string()).collect::<String>(),
                 display[i].iter().collect::<String>()
        );
    };
    println!("Day 17 : Part 1 : {}, {}", min, sum);
}
