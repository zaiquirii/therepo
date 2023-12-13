use std::hash::{DefaultHasher, Hash, Hasher};

pub fn part_01() {
    let input = include_str!("../inputs/input_13");
    let mut lines = input.lines();

    let mut cols = 0;
    let mut rows = 0;
    loop {
        let pattern = lines
            .by_ref()
            .take_while(|&l| !l.is_empty())
            .map(|l| l.as_bytes())
            .collect::<Vec<_>>();
        if pattern.is_empty() {
            break;
        }

        let mut c_hashers = vec![DefaultHasher::new(); pattern[0].len()];
        let r_hashes = pattern.iter()
            .map(|l| {
                let mut hasher = DefaultHasher::new();
                for (i, b) in l.iter().enumerate() {
                    b.hash(&mut hasher);
                    b.hash(&mut c_hashers[i]);
                }
                hasher.finish()
            })
            .collect::<Vec<_>>();
        let c_hashes = c_hashers.iter().map(Hasher::finish).collect::<Vec<_>>();

        let c_mirror = mirror_point(&c_hashes);
        cols += c_mirror;
        if c_mirror == 0 {
            rows += mirror_point(&r_hashes);
        }
    }

    let total = cols + rows * 100;
    println!("Day 13 : Part 1 : {}", total);
}

fn mirror_point(hashes: &Vec<u64>) -> usize {
    for i in 0..hashes.len() - 1 {
        if check_mirror(i, hashes) {
            return i + 1
        }
    }
    0
}

fn check_mirror(test: usize, hashes: &Vec<u64>) -> bool {
    let mut l = test;
    let mut r = test + 1;
    loop {
        if hashes[l] != hashes[r] {
            return false;
        }
        if l == 0 || r == hashes.len() - 1 {
            return true;
        }
        l -= 1;
        r += 1;
    }
}

pub fn part_02() {}
