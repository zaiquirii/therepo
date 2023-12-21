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

fn reachable_spots_2(map: &Map, coords: HashSet<Coord>, steps: usize) -> usize {
    if steps == 0 {
        return coords.len();
    }

    let mut curr_coords = coords.clone();
    let mut next_coords = HashSet::new();
    for _ in 0..steps {
        for c in &curr_coords {
            for n in c.neighbors_2(map) {
                next_coords.insert(n);
            }
        }
        swap(&mut curr_coords, &mut next_coords);
        next_coords.clear();
    }
    curr_coords.len()
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
    let total = reachable_spots(&map, s, 64);
    println!("Day 21 : Part 1 : {}", total);
}

pub fn part_02() {
    let input = include_str!("../inputs/input_21_test");
    let (start, map) = parse_input(input);
    let mut s = HashSet::new();
    s.insert(start);
    let total = reachable_spots_2(&map, s, 100);
    println!("Day 21 : Part 2 : {}", total);
}