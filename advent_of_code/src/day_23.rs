use std::cmp::Ordering;
use std::collections::{HashMap, HashSet, VecDeque};

#[derive(Copy, Clone, PartialEq)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
}

#[derive(PartialEq)]
enum GridType {
    Path,
    Forest,
    Slope { dir: Dir },
}

impl GridType {
    fn char(&self) -> char {
        match self {
            GridType::Path => '.',
            GridType::Forest => '#',
            GridType::Slope { dir } => {
                match dir {
                    Dir::Up => '^',
                    Dir::Down => 'v',
                    Dir::Left => '<',
                    Dir::Right => '>',
                }
            }
        }
    }
}

type Grid = Vec<Vec<GridType>>;

#[derive(Default, Eq, PartialEq, Hash, Clone, Copy, Debug)]
struct Coord {
    x: usize,
    y: usize,
}

impl Coord {
    fn adjacent(&self, dir: Dir, width: usize, height: usize) -> Option<Coord> {
        let c = match dir {
            Dir::Up if self.y > 0 => Coord { x: self.x, y: self.y - 1 },
            Dir::Down if self.y + 1 < height => Coord { x: self.x, y: self.y + 1 },
            Dir::Left if self.x > 0 => Coord { x: self.x - 1, y: self.y },
            Dir::Right if self.x + 1 < width => Coord { x: self.x + 1, y: self.y },
            _ => return None,
        };
        Some(c)
    }

    fn neighbors(&self, grid: &Grid) -> Vec<Coord> {
        let width = grid[0].len();
        let height = grid.len();

        let (dir_count, dirs) = match &grid[self.y][self.x] {
            GridType::Path => (4, [Dir::Up, Dir::Down, Dir::Left, Dir::Right]),
            GridType::Forest => (0, [Dir::Up; 4]),
            GridType::Slope { dir } => {
                match dir {
                    Dir::Up => (1, [Dir::Up; 4]),
                    Dir::Down => (1, [Dir::Down; 4]),
                    Dir::Left => (1, [Dir::Left; 4]),
                    Dir::Right => (1, [Dir::Right; 4]),
                }
            }
        };

        let mut neighbors = Vec::new();
        for d_i in 0..dir_count {
            if let Some(n) = self.adjacent(dirs[d_i], width, height) {
                if grid[n.y][n.x] != GridType::Forest {
                    neighbors.push(n);
                }
            }
        }
        neighbors
    }
}

fn longest_path_recur(grid: &Grid, start: Coord, end: Coord, visited: &mut HashSet<Coord>) -> Option<usize> {
    if start == end {
        return Some(0);
    }

    let mut max: Option<usize> = None;
    for n in start.neighbors(grid) {
        if visited.insert(n) {
            if let Some(length) = longest_path_recur(grid, n, end, visited) {
                max = Some(max.unwrap_or(0).max(length));
            }
            visited.remove(&n);
        }
    }
    max.map(|x| x + 1)
}

#[derive(Copy, Clone, Eq, PartialEq)]
struct SearchNode {
    coord: Coord,
    weight: usize,
}

impl PartialOrd for SearchNode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for SearchNode {
    fn cmp(&self, other: &Self) -> Ordering {
        self.weight.cmp(&other.weight)
    }
}

fn parse_input(input: &str, respect_slopes: bool) -> Grid {
    input
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| {
                    match c {
                        '#' => GridType::Forest,
                        _ if !respect_slopes => GridType::Path,
                        '.' => GridType::Path,
                        '<' => GridType::Slope { dir: Dir::Left },
                        '>' => GridType::Slope { dir: Dir::Right },
                        '^' => GridType::Slope { dir: Dir::Up },
                        'v' => GridType::Slope { dir: Dir::Down },
                        _ => panic!("bad input"),
                    }
                })
                .collect()
        })
        .collect()
}

fn solution(grid: &Grid) -> usize {
    let start = Coord {
        x: grid[0].iter().enumerate().find(|c| *c.1 == GridType::Path).map(|x| x.0).unwrap(),
        y: 0,
    };
    let end = Coord {
        x: grid[grid.len() - 1].iter().enumerate().find(|c| *c.1 == GridType::Path).map(|x| x.0).unwrap(),
        y: grid.len() - 1,
    };
    let mut visited: HashSet<Coord> = HashSet::default();
    visited.insert(start);
    longest_path_recur(&grid, start, end, &mut visited).unwrap()
}

struct Graph {
    edges: HashMap<Coord, Vec<(Coord, usize)>>,
}

fn compress(grid: &Grid, respect_slopes: bool) -> Graph {
    let mut edges: HashMap<Coord, Vec<(Coord, usize)>> = HashMap::new();
    let start = Coord { x: 1, y: 0 };
    let end = Coord { x: grid.len() - 2, y: grid.len() - 1 };
    let (real_start, dist) = next_intersection(grid, start, end).unwrap();
    edges.insert(start, vec![(real_start, dist - 2)]);

    let mut visited = HashSet::new();
    visited.insert(real_start);

    let mut queue = VecDeque::new();
    queue.push_back(real_start);

    while let Some(curr) = queue.pop_front() {
        for neighbor in curr.neighbors(grid) {
            let ns = neighbor.neighbors(grid);
            if ns[0] == curr {
                continue;
            }

            if let Some((next, dist)) = next_intersection(&grid, neighbor, end) {
                edges.entry(curr)
                    .and_modify(|f| {
                        f.push((next, dist));
                    })
                    .or_insert_with(|| {
                        vec![(next, dist)]
                    });
                if !respect_slopes {
                    edges.entry(next)
                        .and_modify(|f| {
                            f.push((curr, dist));
                        })
                        .or_insert_with(|| {
                            vec![(curr, dist)]
                        });
                }
                if !visited.insert(next) {
                    continue;
                }
                queue.push_back(next);
            }
        }
    }

    Graph {
        edges,
    }
}

fn next_intersection(grid: &Grid, start: Coord, end: Coord) -> Option<(Coord, usize)> {
    let mut dist = 0;
    let mut curr = start;
    let mut visited = HashSet::new();

    'outer: loop {
        visited.insert(curr);
        let neighbors = curr.neighbors(grid);
        if neighbors.is_empty() {
            return None;
        }

        if dist > 0 {
            if curr == end {
                return Some((end, dist + 2));
            }
            if let GridType::Slope { .. } = &grid[curr.y][curr.x] {
                if neighbors.len() != 1 {
                    panic!("shouldn't happen");
                }

                return Some((neighbors[0], dist + 2));
            }
        }

        for n in neighbors {
            if !visited.contains(&n) {
                dist += 1;
                curr = n;
                continue 'outer;
            }
        }
        return None;
    }
}

fn longest_path_compressed(graph: &Graph, start: Coord, end: Coord, visited: &mut HashSet<Coord>) -> Option<usize> {
    if start == end {
        return Some(0);
    }

    let mut max: Option<usize> = None;
    for (n, n_dist) in graph.edges.get(&start).unwrap_or(&Vec::new()) {
        if visited.insert(*n) {
            if let Some(length) = longest_path_compressed(graph, *n, end, visited) {
                max = Some(max.unwrap_or(0).max(length + n_dist));
            }
            visited.remove(&n);
        }
    }
    max
}

pub fn part_01() {
    let input = include_str!("../inputs/input_23");
    let grid = parse_input(input, true);
    let length = solution(&grid);
    println!("Day 23 : Part 1 : {}", length);
}

pub fn part_02() {
    let input = include_str!("../inputs/input_23");
    let grid = parse_input(input, true);
    let graph = compress(&grid, false);
    let total = longest_path_compressed(
        &graph,
        Coord { x: 1, y: 0 },
        Coord { x: grid[0].len() - 2, y: grid.len() - 1 },
        &mut HashSet::new(),
    ).unwrap();
    println!("Day 23 : Part 2 : {}", total);
}
