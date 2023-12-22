use std::collections::HashSet;
use std::mem::swap;

enum Dir {
    North,
    South,
    East,
    West,
}

type Map = Vec<Vec<PlotType>>;

enum PlotType {
    Rock,
    Garden,
}

#[derive(Default, Eq, PartialEq, Hash, Clone, Copy)]
struct Coord {
    x: isize,
    y: isize,
}

impl Coord {
    fn adjacent(&self, dir: Dir, width: isize, height: isize) -> Option<Coord> {
        let c = match dir {
            Dir::North if self.y > 0 => Coord { x: self.x, y: self.y - 1 },
            Dir::South if self.y + 1 < height => Coord { x: self.x, y: self.y + 1 },
            Dir::West if self.x > 0 => Coord { x: self.x - 1, y: self.y },
            Dir::East if self.x + 1 < width => Coord { x: self.x + 1, y: self.y },
            _ => return None,
        };
        Some(c)
    }

    fn neighbors(&self, map: &Map) -> Vec<Coord> {
        let width = map[0].len() as isize;
        let height = map.len() as isize;

        let mut neighbors = Vec::new();
        for d in [Dir::North, Dir::South, Dir::East, Dir::West] {
            if let Some(n) = self.adjacent(d, width, height) {
                if let PlotType::Garden = map[n.y as usize][n.x as usize] {
                    neighbors.push(n);
                }
            }
        }
        neighbors
    }

    fn adjacent_2(&self, dir: Dir, width: isize, height: isize) -> Coord {
        match dir {
            Dir::North => Coord { x: self.x, y: self.y - 1 },
            Dir::South => Coord { x: self.x, y: self.y + 1 },
            Dir::West => Coord { x: self.x - 1, y: self.y },
            Dir::East => Coord { x: self.x + 1, y: self.y },
        }
    }

    fn neighbors_2(&self, map: &Map) -> Vec<Coord> {
        let width = map[0].len() as isize;
        let height = map.len() as isize;

        let mut neighbors = Vec::new();
        for d in [Dir::North, Dir::South, Dir::East, Dir::West] {
            let n = self.adjacent_2(d, width, height);
            let mut y = n.y % height;
            if y < 0 {
                y = height + y;
            }
            let mut x = n.x % width;
            if x < 0 {
                x = width + x;
            }
            if let PlotType::Garden = map[y as usize][x as usize] {
                neighbors.push(n);
            }
        }
        neighbors
    }
}

fn reachable_spots(map: &Map, coords: HashSet<Coord>, steps: usize) -> usize {
    if steps == 0 {
        return coords.len();
    }

    let mut next_coords = HashSet::new();
    for c in coords {
        for n in c.neighbors(map) {
            next_coords.insert(n);
        }
    }
    reachable_spots(map, next_coords, steps - 1)
}

fn single_board(map: &Map, start: Coord, steps: usize) -> usize {
    let mut s = HashSet::new();
    s.insert(start);
    reachable_spots_3(map, s, steps)[(steps + 1) % 2]
}

fn reachable_spots_3(map: &Map, coords: HashSet<Coord>, steps: usize) -> [usize; 2] {
    let mut curr_coords = coords.clone();
    let mut next_coords = HashSet::new();
    let mut seen_coords = HashSet::new();
    let mut counts = [0, 0];
    for i in 0..steps {
        for c in &curr_coords {
            for n in c.neighbors(map) {
                if seen_coords.insert(n) {
                    next_coords.insert(n);
                    counts[i % 2] += 1;
                }
            }
        }
        swap(&mut curr_coords, &mut next_coords);
        next_coords.clear();
    }
    counts
}

fn reachable_spots_2(map: &Map, coords: HashSet<Coord>, steps: usize) -> usize {
    let mut curr_coords = coords.clone();
    let mut next_coords = HashSet::new();
    let mut seen_coords = HashSet::new();
    let mut counts = [0, 0];
    for i in 0..steps {
        for c in &curr_coords {
            for n in c.neighbors_2(map) {
                if seen_coords.insert(n) {
                    next_coords.insert(n);
                    counts[i % 2] += 1;
                }
            }
        }
        swap(&mut curr_coords, &mut next_coords);
        next_coords.clear();
    }
    counts[(steps + 1) % 2]
}

fn get_start(x: isize, y: isize, size: usize) -> Coord {
    let half_length = (size / 2) as isize;
    let top_left = Coord {
        x: x * size as isize,
        y: y * size as isize,
    };
    if x == 0 && y == 0 {
        return Coord {
            x: top_left.x + half_length,
            y: top_left.y + half_length,
        };
    }

    let possible_coords = if x == 0 || y == 0 {
        [
            Coord { x: half_length, y: 0 },
            Coord { x: half_length, y: (size - 1) as isize },
            Coord { x: 0, y: half_length },
            Coord { x: (size - 1) as isize, y: half_length }
        ]
    } else {
        [
            Coord { x: 0, y: 0 },
            Coord { x: 0, y: (size - 1) as isize },
            Coord { x: (size - 1) as isize, y: (size - 1) as isize },
            Coord { x: (size - 1) as isize, y: 0 },
        ]
    };

    possible_coords
        .iter()
        .map(|c| Coord {
            x: top_left.x + c.x,
            y: top_left.y + c.y,
        })
        .min_by_key(|c| {
            c.x.abs() + c.y.abs()
        })
        .unwrap()
}

fn logic_way(map: &Map, steps: usize) -> usize {
    let size = map.len() as isize;

    let first_chunk = (size + 3) / 2;
    let max_offset = (steps as isize - first_chunk) / size;
    let max_full = steps as isize / size;

    let mut running_count = 0;
    let mut fulls = [0, 0];
    for y in -max_offset..=max_offset {
        for x in -max_offset..=max_offset {
            let dist = x.abs() + y.abs();
            if dist > max_offset {
                continue;
            }

            if dist < max_full {
                fulls[dist as usize % 2] += 1;
                continue;
            }

            let start = get_start(x, y, size as usize);
            let remaining_steps = (steps as isize - ((start.x.abs() + start.y.abs()) - 2 * (size / 2))) % size;
            let normalized_start = Coord {
                x: ((start.x % size) + size) % size,
                y: ((start.y % size) + size) % size,
            };
            running_count = single_board(map, normalized_start, remaining_steps as usize);
        }
    }

    let center = Coord {
        x: (size / 2 + 1) as isize,
        y: (size / 2 + 1) as isize,
    };
    let mut s = HashSet::new();
    s.insert(center);
    let full_counts = reachable_spots_3(map, s, steps);
    running_count + fulls[0] * full_counts[0] + fulls[1] * full_counts[1]
}

fn parse_input(input: &str) -> (Coord, Map) {
    let mut start = Coord::default();
    let map = input.lines()
        .enumerate()
        .map(|(y, l)| {
            l.char_indices()
                .map(|(x, c)| {
                    match c {
                        '.' => PlotType::Garden,
                        '#' => PlotType::Rock,
                        'S' => {
                            start = Coord { x: x as isize, y: y as isize };
                            PlotType::Garden
                        }
                        _ => panic!("whoops")
                    }
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    (start, map)
}

pub fn part_01() {
    let input = include_str!("../inputs/input_21");
    let (start, map) = parse_input(input);
    let mut s = HashSet::new();
    s.insert(start);
    let total = single_board(&map, start, 64);
    println!("Day 21 : Part 1 : {}", total);
}

pub fn part_02() {
    let input = include_str!("../inputs/input_21_test");
    let (start, map) = parse_input(input);
    // let total = again(&map, 10);

    for i in [
        65,
        65 + 131,
        65 + 131 + 131,
        65 + 131 + 131 + 131,
    ] {
        let mut s = HashSet::new();
        s.insert(start);
        let ui = (i - 65) / 131;
        let total = reachable_spots_2(&map, s, i);
        println!("{} {} {}", i, ui, total);
    }
    // println!("Day 21 : Part 2 : {}", total);
}