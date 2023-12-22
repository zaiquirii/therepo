use crate::common::Point;

pub fn part_01() {
    let input = include_str!("../inputs/input_22");
    let (footprint, mut blocks) = parse_input(input);
    compress_blocks(footprint, &mut blocks);
    for b in &blocks {
        println!("Block: {:?}", b);
    }

    let total = (0..blocks.len())
        .filter(|b| can_disintegrate(&blocks, *b))
        .count();
    println!("Day 22 : Part 1 : {}", total)
}

#[derive(Debug)]
struct Coord {
    x: u32,
    y: u32,
    z: u32,
}

impl Coord {
    fn new(input: &str) -> Self {
        let mut s = input.split(",");
        Self {
            x: s.next().unwrap().parse().unwrap(),
            y: s.next().unwrap().parse().unwrap(),
            z: s.next().unwrap().parse().unwrap(),
        }
    }
}

#[derive(Debug)]
struct Block {
    start: Coord,
    end: Coord,
}

fn parse_input(input: &str) -> (Point<u32>, Vec<Block>) {
    let mut footprint = Point::new(0, 0);
    let mut blocks = input.lines()
        .map(|l| {
            let mut parts = l.split("~");
            let start = Coord::new(parts.next().unwrap());
            let end = Coord::new(parts.next().unwrap());
            footprint.x = footprint.x.max(start.x).max(end.x);
            footprint.y = footprint.y.max(start.y).max(end.y);
            Block {
                start,
                end,
            }
        })
        .collect();
    (footprint, blocks)
}

fn compress_blocks(footprint: Point<u32>, blocks: &mut Vec<Block>) {
    blocks.sort_by_key(|b| b.start.z);
    let mut z_levels = vec![vec![0; footprint.x as usize + 1]; footprint.y as usize + 1];
    for b in blocks.iter_mut() {
        let mut z_max = 0;
        for y in b.start.y..=b.end.y {
            for x in b.start.x..=b.end.x {
                z_max = z_max.max(z_levels[y as usize][x as usize]);
            }
        }
        let diff = b.start.z - z_max - 1;
        b.start.z -= diff;
        b.end.z -= diff;
        for y in b.start.y..=b.end.y {
            for x in b.start.x..=b.end.x {
                z_levels[y as usize][x as usize] = b.end.z;
            }
        }
    }
}

fn can_disintegrate(blocks: &Vec<Block>, index: usize) -> bool {
    let children = get_children(blocks, index);
    if children.is_empty() {
        return true;
    }
    children.iter().all(|c| get_parents(blocks, *c).len() >= 2)
}

fn get_children(blocks: &Vec<Block>, index: usize) -> Vec<usize> {
    (0..blocks.len())
        .filter(|i| supports(blocks, index, *i))
        .collect()
}

fn get_parents(blocks: &Vec<Block>, index: usize) -> Vec<usize> {
    (0..blocks.len())
        .filter(|i| supports(blocks, *i, index))
        .collect()
}

fn supports(blocks: &Vec<Block>, under_i: usize, over_i: usize) -> bool {
    let under = &blocks[under_i];
    let over = &blocks[over_i];

    if under.end.z >= over.start.z || over.start.z - under.end.z != 1 {
        return false;
    }
    intersects(under.start.x, under.end.x, over.start.x, over.end.x) &&
        intersects(under.start.y, under.end.y, over.start.y, over.end.y)
}

fn intersects(s0: u32, e0: u32, s1: u32, e1: u32) -> bool {
    if e0 < s1 || e1 < s0 {
        return false;
    }
    return true;
}

pub fn part_02() {}