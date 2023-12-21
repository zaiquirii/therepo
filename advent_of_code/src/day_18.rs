use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};
use crate::common::Point;

#[derive(Clone, Copy, Debug, PartialEq)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
}

impl Dir {
    fn offset(&self) -> Point<isize> {
        match self {
            Dir::Up => Point::new(0, -1),
            Dir::Down => Point::new(0, 1),
            Dir::Left => Point::new(-1, 0),
            Dir::Right => Point::new(1, 0),
        }
    }
}

#[derive(Debug)]
struct Operation {
    dir: Dir,
    steps: isize,
}

struct Lagoon {
    trench: HashMap<isize, Vec<isize>>,
}

impl Lagoon {
    fn run_plan(&mut self, plan: &Vec<Operation>) {
        let mut curr_loc = Point::new(0isize, 0);
        self.add_trench(curr_loc);

        for op in plan {
            let offset = op.dir.offset();
            for _ in 0..op.steps {
                curr_loc = curr_loc.add(offset);
                self.add_trench(curr_loc);
            }
        }

        self.trench.values_mut().for_each(|r| r.sort());
    }

    fn add_trench(&mut self, loc: Point<isize>) {
        let r = self.trench.entry(loc.y).or_insert_with(|| Vec::new());
        if !r.contains(&loc.x) {
            r.push(loc.x)
        }
    }

    fn full_size(self) -> usize {
        let (min_height, max_width) = self.trench.iter()
            .map(|v| (*v.0, *v.1.last().unwrap()))
            .fold((0, 0), |acc, v| {
                (acc.0.min(v.0), acc.1.max(v.1))
            });

        let mut grid = vec![vec!['.'; (max_width + 1) as usize]; self.trench.len()];
        let mut display_grid = vec![vec!['.'; (max_width + 1) as usize]; self.trench.len()];

        self.trench.iter()
            .for_each(|(y, xs)| {
                for x in xs {
                    grid[(*y - min_height) as usize][*x as usize] = '#';
                }
            });

        let mut count = 0;
        for (y, row) in grid.iter().enumerate() {
            let mut x = 0;
            let mut inside = false;
            while x < row.len() {
                let c = row[x];
                if c == '#' {
                    let seg_start = x;
                    let seg_came_down = y != 0 && grid[y - 1][x] == '#';

                    display_grid[y][x] = '#';
                    while x < row.len() && row[x] == '#' {
                        display_grid[y][x] = '#';
                        x += 1;
                    }

                    let seg_len = x - seg_start;
                    count += seg_len;

                    if x != row.len() {
                        let seg_goes_up = y != 0 && grid[y - 1][x - 1] == '#';
                        if seg_len == 1 || seg_came_down != seg_goes_up {
                            inside = !inside;
                        }
                    }
                } else if c == '.' {
                    if inside {
                        display_grid[y][x] = 'O';
                        count += 1;
                    }
                    x += 1;
                }
            }
        }
        display_grid.iter()
            .for_each(|l| {
                println!("{}", l.iter().collect::<String>());
            });
        count
    }
}

#[derive(Debug)]
struct Line {
    start: Point<isize>,
    end: Point<isize>,
    dir: Dir,
}

impl Line {
    fn on_y(&self, y: isize) -> bool {
        let min = self.start.y.min(self.end.y);
        let max = self.start.y.max(self.end.y);
        y >= min && y <= max
    }
}

#[derive(Default)]
struct BigLagoon {
    vertices: Vec<Point<isize>>,
    perimeter: isize,
    trench: Vec<Line>,
    origin: Point<isize>,
    dims: Point<isize>,
}

impl BigLagoon {
    fn run_plan(&mut self, plan: &Vec<Operation>) {
        let mut top_left = Point::new(0, 0);
        let mut bottom_right = Point::new(0, 0);
        let mut curr_loc = Point::new(0isize, 0);
        for op in plan {
            let offset = op.dir.offset();
            let end_loc = curr_loc.offset(offset.x * op.steps, offset.y * op.steps);
            let line = Line {
                start: curr_loc,
                end: end_loc,
                dir: op.dir,
            };
            self.trench.push(line);
            top_left.x = top_left.x.min(end_loc.x);
            top_left.y = top_left.y.min(end_loc.y);
            bottom_right.x = bottom_right.x.max(end_loc.x);
            bottom_right.y = bottom_right.y.max(end_loc.y);
            curr_loc = end_loc;
            self.vertices.push(end_loc);
            self.perimeter += op.steps;
        }
        self.origin = top_left;
        self.dims = bottom_right.sub(top_left).offset(1, 1);
    }

    fn full_size(&self) -> usize {
        // for each y from top left through height
        //     find intersecting lines
        //     sort them by x
        //     collapse them

        let mut count = 0;
        for i in 0..self.dims.y {
            let y = self.origin.y + i;
            // Optimization: reuse vec
            let mut lines = self.trench.iter()
                .filter(|l| l.on_y(y))
                .collect::<Vec<_>>();
            lines.sort_by(|a, b| {
                let min_a = a.start.x.min(a.end.x);
                let min_b = b.start.x.min(b.end.x);
                let c = min_a.cmp(&min_b);
                if c != Ordering::Equal {
                    return c;
                }
                if a.dir == Dir::Up || a.dir == Dir::Down {
                    return Ordering::Less;
                }
                return Ordering::Greater;
            });

            let mut inside = false;
            let mut i = 0;
            while i < lines.len() {
                let l = lines[i];
                if inside {
                    let prev = lines[i - 1];
                    count += (l.start.x - prev.start.x - 1) as usize;
                }

                if y == l.start.y || y == l.end.y {
                    let seg_came_down = l.dir == Dir::Down;
                    let seg = lines[i + 1];
                    count += seg.start.x.abs_diff(seg.end.x) + 1;
                    let seg_goes_up = lines[i + 2].dir == Dir::Up;
                    if seg_came_down != seg_goes_up {
                        inside = !inside;
                    }
                    i += 3;
                } else {
                    count += 1;
                    inside = !inside;
                    i += 1;
                    continue;
                }
            }
        }
        count
    }

    fn shoelace(&mut self) -> usize {
        let mut x_sum = 0;
        let mut y_sum = 0;
        for i in 0..self.vertices.len() {
            let a = self.vertices[i];
            let b = self.vertices[(i + 1) % self.vertices.len()];
            x_sum += a.x * b.y;
            y_sum += a.y * b.x;
        }
        ((x_sum - y_sum).abs() / 2 + self.perimeter / 2 + 1) as usize
    }
}

fn parse_input(input: &str) -> Vec<Operation> {
    input.lines()
        .map(|l| {
            let mut parts = l.split_ascii_whitespace();
            let dir = match parts.next().unwrap() {
                "R" => Dir::Right,
                "L" => Dir::Left,
                "U" => Dir::Up,
                "D" => Dir::Down,
                _ => panic!("Bad input")
            };
            let steps = parts.next().unwrap().parse::<isize>().unwrap();
            Operation {
                dir,
                steps,
            }
        })
        .collect()
}

fn parse_input_2(input: &str) -> Vec<Operation> {
    input.lines()
        .map(|l| {
            let mut parts = l.split_ascii_whitespace();
            parts.next();
            parts.next();
            let s = parts.next().unwrap();
            let steps = isize::from_str_radix(&s[2..7], 16).unwrap();
            let dir = match &s[7..8] {
                "0" => Dir::Right,
                "1" => Dir::Down,
                "2" => Dir::Left,
                "3" => Dir::Up,
                _ => panic!("Bad input")
            };

            Operation {
                dir,
                steps,
            }
        })
        .collect()
}

pub fn part_01() {
    let input = include_str!("../inputs/input_18");
    let plan = parse_input(input);
    let mut lagoon = Lagoon { trench: HashMap::new() };
    lagoon.run_plan(&plan);
    println!("Day 18 : Part 1 : {}", lagoon.full_size())
}

pub fn part_02() {
    let input = include_str!("../inputs/input_18");
    let plan = parse_input_2(input);
    let mut lagoon = BigLagoon::default();
    lagoon.run_plan(&plan);
    println!("Day 18 : Part 2 : {}", lagoon.shoelace());
    println!("Day 18 : Part 2 : {}", lagoon.full_size())
}