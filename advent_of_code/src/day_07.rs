use std::cmp::Ordering;
use crate::common;

pub fn part_01() {
    let input = common::read_file("inputs/input_07").unwrap();
    let mut hands: Vec<_> = input.lines()
        .map(|l| {
            let parts: Vec<_> = l.split_ascii_whitespace().collect();
            Hand {
                cards: parts[0].chars().collect(),
                bid: parts[1].parse::<u64>().unwrap(),
            }
        })
        .collect();
    hands.sort_by(|lhs, rhs| {
        match rhs.ordinal().cmp(&lhs.ordinal()) {
            Ordering::Equal => {
                for i in 0..5 {
                    let lhs_rank = char_to_rank(lhs.cards[i]);
                    let rhs_rank = char_to_rank(rhs.cards[i]);
                    match rhs_rank.cmp(&lhs_rank) {
                        Ordering::Equal => continue,
                        other => return other
                    }
                }
                Ordering::Equal
            }
            other => other
        }
    });
    let total: u64 = hands.iter()
        .enumerate()
        .map(|(i, h)| (i + 1) as u64 * h.bid as u64)
        .sum();
    println!("Day 07 : Part 1 : {}", total);
}

pub fn part_02() {
    let input = common::read_file("inputs/input_07").unwrap();
    let mut hands: Vec<_> = input.lines()
        .map(|l| {
            let parts: Vec<_> = l.split_ascii_whitespace().collect();
            let h = Hand {
                cards: parts[0].chars().collect(),
                bid: parts[1].parse::<u64>().unwrap(),
            };
            h.ordinal_j();
            h
        })
        .collect();
    hands.sort_by(|lhs, rhs| {
        match rhs.ordinal_j().cmp(&lhs.ordinal_j()) {
            Ordering::Equal => {
                for i in 0..5 {
                    let lhs_rank = char_to_rank_j(lhs.cards[i]);
                    let rhs_rank = char_to_rank_j(rhs.cards[i]);
                    match rhs_rank.cmp(&lhs_rank) {
                        Ordering::Equal => continue,
                        other => return other
                    }
                }
                Ordering::Equal
            }
            other => other
        }
    });
    let total: u64 = hands.iter()
        .enumerate()
        .map(|(i, h)| (i + 1) as u64 * h.bid as u64)
        .sum();
    println!("Day 07 : Part 2 : {}", total);
}

#[derive(Debug)]
struct Hand {
    cards: Vec<char>,
    bid: u64,
}

impl Hand {
    fn ordinal(&self) -> u64 {
        let mut counts: Vec<[u64; 2]> = Vec::new();
        'outer: for c in &self.cards {
            let c_rank = char_to_rank(*c);
            for x in &mut counts {
                if c_rank == x[0] {
                    x[1] += 1;
                    continue 'outer;
                }
            }
            counts.push([c_rank, 1])
        }
        counts.sort_by_key(|x| x[1]);
        match (counts.len(), counts[counts.len() - 1][1]) {
            (1, 5) => 0,
            (2, 4) => 1,
            (2, 3) => 2,
            (3, 3) => 3,
            (3, 2) => 4,
            (4, 2) => 5,
            (5, 1) => 6,
            e => {
                println!("NOOOO : {:?}", e);
                panic!("shouldn't get here");
            }
        }
    }
    fn ordinal_j(&self) -> u64 {
        let mut counts: Vec<[u64; 2]> = Vec::new();
        'outer: for c in &self.cards {
            let c_rank = char_to_rank_j(*c);
            for x in &mut counts {
                if c_rank == x[0] {
                    x[1] += 1;
                    continue 'outer;
                }
            }
            counts.push([c_rank, 1])
        }

        let j_index = counts.iter()
            .enumerate()
            .find(|(_, x)| x[0] == 12)
            .map(|(i, _)| i);

        let j_count = match j_index {
            None => 0,
            Some(i) => counts.remove(i)[1]
        };
        if j_count == 5 {
            return 0
        }

        counts.sort_by_key(|x| x[1]);

        match (counts.len(), counts[counts.len() - 1][1] + j_count) {
            (1, 5) => 0,
            (2, 4) => 1,
            (2, 3) => 2,
            (3, 3) => 3,
            (3, 2) => 4,
            (4, 2) => 5,
            (5, 1) => 6,
            e => {
                println!("NOOOO : {:?}", e);
                panic!("shouldn't get here");
            }
        }
    }
}

const RANKS: [char; 13] = [
    'A', 'K', 'Q', 'J', 'T', '9', '8', '7', '6', '5', '4', '3', '2'
];

fn char_to_rank(c: char) -> u64 {
    RANKS.iter()
        .enumerate()
        .find(|x| *x.1 == c)
        .map(|x| x.0 as u64)
        .unwrap()
}

const RANKS_J: [char; 13] = [
    'A', 'K', 'Q', 'T', '9', '8', '7', '6', '5', '4', '3', '2', 'J'
];

fn char_to_rank_j(c: char) -> u64 {
    RANKS_J.iter()
        .enumerate()
        .find(|x| *x.1 == c)
        .map(|x| x.0 as u64)
        .unwrap()
}
