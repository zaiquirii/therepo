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
    println!("PARSE TIME: {:?}", now.elapsed());
    let closest = almanac.closest_smart();
    let elapsed = now.elapsed();
    println!("TIME: {:?}", elapsed);
    println!("Day 5 : Part 2 : {}", closest);
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
        maps: maps.iter_mut().map(|x| fill_gaps(x)).collect(),
    }
}

fn fill_gaps(x: &mut Vec<(Range<i64>, i64)>) -> Vec<(Range<i64>, i64)> {
    let sorted = x;
    sorted.sort_by_key(|x| x.0.start);

    let mut output = Vec::new();
    let f = sorted[0].clone();
    if f.0.start != 0 {
        output.push((0..f.0.start, 0));
    }
    output.push(f);

    for i in 1..sorted.len() {
        let t = &sorted[i];
        let last = output.last().unwrap();
        if last.0.end != t.0.start {
            output.push((last.0.end..t.0.start, 0));
        }
        output.push(t.clone());
    }
    let l = output.last().unwrap().0.end;
    output.push((l..i64::MAX, 0));

    output
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

    fn closest_smart(&self) -> i64 {
        self.seeds
            .iter()
            .map(|r| {
                let mut current_seed = r.start;
                let mut min_location = i64::MAX;
                while r.contains(&current_seed) {
                    let mut target = current_seed;
                    let mut smallest_offset = i64::MAX;
                    for m in &self.maps {
                        if let Some(x) = m.iter().find(|x| x.0.contains(&target)) {
                            smallest_offset = smallest_offset.min(x.0.end - target);
                            target += x.1;
                        } else {
                            smallest_offset = 1
                        }
                    }
                    min_location = min_location.min(target);
                    current_seed += smallest_offset;
                }
                min_location
            })
            .min()
            .unwrap()
    }

    fn _closest_para(&self) -> i64 {
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
