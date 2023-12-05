use std::ops::Range;
use std::time::Instant;
use rayon::prelude::*;
use crate::common;

pub fn part_01() {
    let input = common::read_file("inputs/input_05").unwrap();
    let almanac = parse_input(input, false);
    let closest = almanac.closest_location();
    println!("Day 5 : Part 1 : {}", closest);
}

pub fn part_02() {
    let now = Instant::now();
    let input = common::read_file("inputs/input_05").unwrap();
    let almanac = parse_input(input, true);
    let closest = almanac.closest_para();
    let elapsed = now.elapsed();
    println!("TIME: {:?}", elapsed);
    println!("Day 5 : Part 1 : {}", closest);
}
// 78µs


fn parse_input(input: String, seeds_as_range: bool) -> Almanac {
    let mut lines = input.lines();
    let seed_parts = lines.next().unwrap()
        .split_ascii_whitespace()
        .skip(1)
        .map(|x| x.parse::<i64>().unwrap());
    let seeds: Vec<_> = if seeds_as_range {
        seed_parts
            .array_chunks()
            .map(|[start, range]| {
                start..start + range
            })
            .collect()
    } else {
        seed_parts
            .map(|x| x..x + 1)
            .collect()
    };
    lines.next();

    let mut maps = Vec::new();
    for l in lines {
        if l.contains("map") {
            maps.push(Vec::new());
            continue;
        }

        let parts: Vec<_> = l
            .split_ascii_whitespace()
            .map(|x| x.parse::<i64>().unwrap())
            .collect();

        if let [dst_start, src_start, range] = parts.as_slice() {
            let map = maps.last_mut().unwrap();
            map.push((*src_start..*src_start + *range, dst_start - src_start));
        }
    }

    Almanac {
        seeds,
        maps,
    }
}

struct Almanac {
    seeds: Vec<Range<i64>>,
    maps: Vec<Vec<(Range<i64>, i64)>>,
}

impl Almanac {
    fn closest_location(&self) -> i64 {
        self.seeds
            .iter()
            .flat_map(|r| {
                r.clone().map(|x| {
                    let mut target = x;
                    for m in &self.maps {
                        if let Some(x) = m.iter().find(|x| x.0.contains(&target)) {
                            target += x.1
                        }
                    }
                    target
                })
            })
            .min()
            .unwrap()
    }
    fn closest_para(&self) -> i64 {
        self.seeds
            .par_iter()
            .map(|r| {
                r.clone()
                    .into_par_iter()
                    .map(|x| {
                        let mut target = x;
                        for m in &self.maps {
                            if let Some(x) = m.iter().find(|x| x.0.contains(&target)) {
                                target += x.1
                            }
                        }
                        target
                    })
                    .min()
                    .unwrap()
            })
            .min()
            .unwrap()
    }
}
