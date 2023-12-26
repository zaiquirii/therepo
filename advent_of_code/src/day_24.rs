use std::collections::HashSet;

// const MIN: f64 = 7.0;
// const MAX: f64 = 27.0;
const MIN: f64 = 200000000000000.0;
const MAX: f64 = 400000000000000.0;

pub fn part_01() {
    let input = include_str!("../inputs/input_24");
    let mut stones = parse_input(input);
    stones.sort_by(|a, b| a.vel.x.partial_cmp(&b.vel.x).unwrap());
    for s in &stones {
        println!("{:?}", s);
    }
    let lines = stones.iter()
        .map(|s| s.to_line().unwrap())
        .collect::<Vec<_>>();

    let mut count = 0;
    for left_i in 0..lines.len() - 1 {
        for right_i in left_i + 1..lines.len() {
            let left = &lines[left_i];
            let right = &lines[right_i];

            if let Some(p) = left.intersection(right) {
                if p.x >= MIN && p.x <= MAX && p.y >= MIN && p.y <= MAX {
                    count += 1;
                }
            }
        }
    }
    println!("Day 24 : Part 1 : {}", count);
}

#[derive(Debug, Copy, Clone)]
struct Vec3 {
    x: f64,
    y: f64,
    z: f64,
}

impl Vec3 {
    fn new(input: &str) -> Self {
        let t = input.replace(" ", "");
        let mut s = t.split(",");
        Self {
            x: s.next().unwrap().parse().unwrap(),
            y: s.next().unwrap().parse().unwrap(),
            z: s.next().unwrap().parse().unwrap(),
        }
    }
}

#[derive(Debug)]
struct HailStone {
    pos: Vec3,
    vel: Vec3,
}

impl HailStone {
    fn to_line(&self) -> Option<Line2d> {
        Line2d::new(self.pos.x, self.pos.y, self.vel.x, self.vel.y)
    }
}

#[derive(Debug)]
struct Line2d {
    m: f64,
    b: f64,
    dx: f64,
    x_naught: f64,
}

impl Line2d {
    fn new(pos_x: f64, pos_y: f64, vel_x: f64, vel_y: f64) -> Option<Self> {
        // if vel_x == 0.0 {
        //     return None;
        //     // panic!("bad slope")
        // }

        let m = vel_y / vel_x;
        Some(Line2d {
            m,
            b: pos_y - m * pos_x,
            dx: vel_x,
            x_naught: pos_x,
        })
    }

    fn intersection(&self, other: &Line2d) -> Option<Vec3> {
        if self.m == other.m {
            return None;
        }

        let x = (other.b - self.b) / (self.m - other.m);
        let y = self.m * x + self.b;

        if self.time(x) >= 0.0 && other.time(x) >= 0.0 {
            return Some(Vec3 {
                x,
                y,
                z: 0.0,
            });
        }
        None
    }

    fn time(&self, x: f64) -> f64 {
        (x - self.x_naught) / self.dx
    }
}

fn parse_input(input: &str) -> Vec<HailStone> {
    input.lines()
        .map(|l| {
            let (pos, vel) = l.split_once(" @ ").unwrap();
            HailStone {
                pos: Vec3::new(pos),
                vel: Vec3::new(vel),
            }
        })
        .collect()
}

pub fn part_02() {
    let input = include_str!("../inputs/input_24");
    let mut stones = parse_input(input);

    let range = 260;
    let mut offsets = Vec::new();
    for y in -range..=range {
        'outer: for x in -range..=range {
            let off_x = x as f64;
            let off_y = y as f64;
            let mut lines = Vec::new();
            for s in &stones {
                if let Some(l) = Line2d::new(s.pos.x, s.pos.y, s.vel.x - off_x, s.vel.y - off_y) {
                    lines.push(l);
                } else {
                    continue 'outer;
                }
            }
            if let Some(p) = all_intersect(&mut lines) {
                offsets.push((off_x, off_y, p));
            }
        }
    }

    // let offsets = vec![(41.0, 255.0, Vec3 { x: 267365104480541.06, y: 139405790744695.0, z: 0.0 })];
    let mut z_offsets = Vec::new();
    for offset in &offsets {
        'outer: for z in -2 * range..=2 * range {
            let off_z = z as f64;
            let off_y = offset.1;
            let mut lines = Vec::new();
            for s in &stones {
                if let Some(l) = Line2d::new(s.pos.z, s.pos.y, s.vel.z - off_z, s.vel.y - off_y) {
                    lines.push(l);
                } else {
                    continue 'outer;
                }
            }
            if let Some(p) = all_intersect(&mut lines) {
                let pos = Vec3 {
                    x: offset.2.x,
                    y: offset.2.y,
                    z: p.x,
                };
                z_offsets.push((offset.0, off_z, off_y, pos, pos.x + pos.y + pos.z));
            }
        }
    }

    println!("OFFSETS {} {:?}", offsets.len(), offsets);
    println!("Z OFFSETS {} {:?}", z_offsets.len(), z_offsets);
}

fn all_intersect(lines: &Vec<Line2d>) -> Option<Vec3> {
    let a = &lines[0];
    let b = &lines[1];
    let check = if let Some(p) = a.intersection(b) {
        p
    } else {
        return None;
    };

    let range = 200.0;
    if lines[2..]
        .iter()
        .all(|b| {
            let c = &check;
            if let Some(p) = a.intersection(b) {
                (p.x - check.x).abs() < range && (p.y - check.y).abs() < range
                // p.x == check.x && p.y == check.y
            } else {
                false
            }
        }) {
        lines[2..]
            .iter()
            .map(|b| {
                let p = a.intersection(b).unwrap();
                ((p.x - check.x).abs(), (p.y - check.y).abs())
                // (p.x - check.x).abs() < range && (p.y - check.y).abs() < range
                // (p.x - check.x).abs() < range && (p.y - check.y).abs() < range
            })
            .for_each(|x| println!("{:?}", x));

        Some(Vec3 {
            x: check.x,
            y: check.y,
            z: 0.0,
        })
    } else {
        None
    }
}