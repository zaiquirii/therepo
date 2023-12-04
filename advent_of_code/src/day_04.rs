use crate::common;

pub fn part_01() {
    let input = common::read_file("inputs/input_04").unwrap();
    let total: u32 = input
        .lines()
        .map(|l| {
            let mut tokens = l.split_ascii_whitespace();
            let winning_numbers : Vec<_> = tokens
                .by_ref()
                .skip(2)
                .take_while(|l| *l != "|")
                .map(|l| l.parse::<u32>().unwrap())
                .collect();
            let count = tokens
                .map(|l| l.parse::<u32>().unwrap())
                .filter(|n| winning_numbers.contains(n))
                .count();
            if count == 0 {
                0
            } else {
                2u32.pow((count - 1) as u32)
            }
        })
        .sum();
    println!("Day 4 : Part 1 : {}", total)
}

pub fn part_02() {
    let input = common::read_file("inputs/input_04").unwrap();
    let winning_counts: Vec<_> = input
        .lines()
        .map(|l| {
            let mut tokens = l.split_ascii_whitespace();
            let winning_numbers: Vec<_> = tokens
                .by_ref()
                .skip(2)
                .take_while(|l| *l != "|")
                .map(|l| l.parse::<u32>().unwrap())
                .collect();
            tokens
                .map(|l| l.parse::<u32>().unwrap())
                .filter(|n| winning_numbers.contains(n))
                .count() as u32
        })
        .collect();
    let mut counts = vec!(1; winning_counts.len());
    for (card, winning_count) in winning_counts.iter().enumerate() {
        let card_count = counts[card];
        for j in card + 1..card + 1 + (*winning_count as usize) {
            counts[j] += card_count
        }
    }
    let total: u32 = counts.iter().sum();
    println!("Day 4 : Part 1 : {}", total)
}
