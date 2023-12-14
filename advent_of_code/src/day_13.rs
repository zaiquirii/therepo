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

        cols += mirror_point(&c_hashes);
        rows += mirror_point(&r_hashes);
    }

    let total = cols + rows * 100;
    println!("Day 13 : Part 1 : {}", total);
}

fn mirror_point(hashes: &Vec<u64>) -> usize {
    for i in 0..hashes.len() - 1 {
        if check_mirror(i, hashes) {
            return i + 1;
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

pub fn part_02() {
    let input = include_str!("../inputs/input_13");
    let mut lines = input.lines();

    let mut col_count = 0;
    let mut row_count = 0;
    loop {
        let pattern = lines
            .by_ref()
            .take_while(|&l| !l.is_empty())
            .map(|l| l.as_bytes())
            .collect::<Vec<_>>();
        if pattern.is_empty() {
            break;
        }

        let mut columns = vec![Vec::new(); pattern[0].len()];
        pattern.iter()
            .for_each(|l| {
                for (i, &b) in l.iter().enumerate() {
                    columns[i].push(b);
                }
            });
        let cols = columns.iter().map(|x| x.as_slice()).collect::<Vec<_>>();
        let rows = pattern;

        col_count += mirror_point_2(&cols);
        row_count += mirror_point_2(&rows);
    }

    let total = col_count + row_count * 100;
    println!("Day 13 : Part 2 : {}", total);
}

fn mirror_point_2(chunks: &Vec<&[u8]>) -> usize {
    for i in 0..chunks.len() - 1 {
        if check_mirror_2(i, chunks) {
            return i + 1;
        }
    }
    0
}

fn check_mirror_2(test: usize, chunks: &Vec<&[u8]>) -> bool {
    let mut l = test;
    let mut r = test + 1;
    let mut smudge_found = false;
    loop {
        if smudge_found {
            if chunks[l] != chunks[r] {
                return false;
            }
        } else {
            let d = differences(chunks[l], chunks[r]);
            match d {
                0 => {},
                1 => smudge_found = true,
                _ => return false
            }
        }

        if l == 0 || r == chunks.len() - 1 {
            return smudge_found;
        }

        l -= 1;
        r += 1;
    }
}

fn differences(lhs: &[u8], rhs: &[u8]) -> usize {
    let mut count = 0;
    for i in 0..lhs.len() {
        if lhs[i] != rhs[i] {
            count += 1;
        }
    }
    count
}
