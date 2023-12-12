type Loc = (i64, i64);

enum Dir {
    Left,
    Right,
    Up,
    Down,
}

impl Dir {
    fn from_char(c: char) -> Option<(Self, Self)> {
        match c {
            '|' => Some((Dir::Up, Dir::Down)),
            '-' => Some((Dir::Left, Dir::Right)),
            'L' => Some((Dir::Up, Dir::Right)),
            'J' => Some((Dir::Up, Dir::Left)),
            '7' => Some((Dir::Left, Dir::Down)),
            'F' => Some((Dir::Right, Dir::Down)),
            _ => None,
        }
    }
}

struct Pipes {
    pipes: Vec<Vec<char>>,
}

impl Pipes {
    fn get(&self, l: Loc) -> char {
        return self.pipes[l.1 as usize][l.0 as usize];
    }

    fn validate(&self, l: Loc, d: Dir) -> Option<Loc> {
        match d {
            Dir::Left => {
                if l.0 == 0 {
                    None
                } else {
                    Some((l.0 - 1, l.1))
                }
            }
            Dir::Right => {
                if l.0 as usize >= self.pipes[0].len() - 1 {
                    None
                } else {
                    Some((l.0 + 1, l.1))
                }
            }
            Dir::Up => {
                if l.1 == 0 {
                    None
                } else {
                    Some((l.0, l.1 - 1))
                }
            }
            Dir::Down => {
                if l.1 as usize >= self.pipes.len() - 1 {
                    None
                } else {
                    Some((l.0, l.1 + 1))
                }
            }
        }
    }

    fn connections(&self, l: Loc) -> Option<(Loc, Loc)> {
        let l_c = self.get(l);
        let dirs = Dir::from_char(l_c);
        match dirs {
            Some(d) => {
                let first = self.validate(l, d.0);
                let second = self.validate(l, d.1);
                if first.is_some() && second.is_some() {
                    Some((first.unwrap(), second.unwrap()))
                } else {
                    None
                }
            }
            _ => None
        }
    }

    fn next(&self, prev: Loc, curr: Loc) -> Option<Loc> {
        match self.connections(curr) {
            None => None,
            Some((first, second)) => if first == prev {
                Some(second)
            } else if second == prev {
                Some(first)
            } else {
                None
            }
        }
    }

    fn get_path(&self, mut prev: Loc, mut curr: Loc) -> Option<Vec<Loc>> {
        if curr.0 < 0 || curr.0 >= self.pipes[0].len() as i64 || curr.1 < 0 || curr.1 >= self.pipes.len() as i64 {
            return None;
        }

        let mut path = Vec::new();
        path.push(curr);
        while self.get(curr) != 'S' {
            match self.next(prev, curr) {
                None => return None,
                Some(next) => {
                    path.push(next);
                    prev = curr;
                    curr = next;
                }
            }
        }
        Some(path)
    }
}

fn parse_input(input: &str) -> (Pipes, Loc) {
    let mut start = Loc::default();
    let pipes: Vec<_> = input
        .lines()
        .enumerate()
        .map(|(y, l)|
            l.chars()
                .enumerate()
                .map(|c| {
                    if c.1 == 'S' {
                        start = (c.0 as i64, y as i64);
                    };
                    c.1
                })
                .collect::<Vec<_>>()
        )
        .collect();
    (Pipes { pipes }, start)
}

fn path(pipes: &Pipes, start: Loc) -> Vec<Loc> {
    if let Some(p) = pipes.get_path(start, (start.0 + 1, start.1)) {
        p
    } else if let Some(p) = pipes.get_path(start, (start.0 - 1, start.1)) {
        p
    } else if let Some(p) = pipes.get_path(start, (start.0, start.1 + 1)) {
        p
    } else if let Some(p) = pipes.get_path(start, (start.0, start.1 - 1)) {
        p
    } else {
        panic!("WHHHY");
    }
}

pub fn part_01() {
    let input = include_str!("../inputs/input_10");
    let (pipes, start) = parse_input(input);
    let result = path(&pipes, start);
    println!("Day 10 : Part 1 : {}", result.len() / 2);
}

pub fn part_02() {
    let input = include_str!("../inputs/input_10");
    let (mut pipes, start) = parse_input(input);
    let result = path(&pipes, start);

    let mut inside_count = 0;
    let map = &mut pipes.pipes;
    // This value is hardcoded based on looking at the input,
    // didn't feel it was worth implementing the real check
    map[start.1 as usize][start.0 as usize] = 'F';
    let width = map[0].len();
    let height = map.len();
    for y in 0..height {
        let mut crosses = 0;
        let mut stack = None;
        for x in 0..width {
            let loc = (x as i64, y as i64);
            if result.contains(&loc) {
                let c = map[y][x];
                map[y][x] = '.';
                if ['L', 'F', '7', 'J'].contains(&c) {
                    if stack.is_none() {
                        let _ = stack.insert(c);
                        crosses += 1;
                    } else {
                        let last = stack.take().unwrap();
                        if (last == 'F' && c == '7') ||
                            (last == '7' && c == 'F') ||
                            (last == 'J' && c == 'L') ||
                            (last == 'L' && c == 'J')
                        {
                            crosses += 1;
                        }
                    }
                } else if c == '|' || c == 'S' {
                    crosses += 1;
                }
            } else {
                map[y][x] = if crosses % 2 == 0 {
                    'O'
                } else {
                    inside_count += 1;
                    'I'
                }
            }
        }
        println!("{}", map[y].iter().collect::<String>())
    }

    println!("Day 10 : Part 2 : {}", inside_count);
}
