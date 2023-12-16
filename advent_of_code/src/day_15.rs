pub fn part_01() {
    let input = include_str!("../inputs/input_15");
    let total = input.lines().next().unwrap()
        .split(",")
        .map(ascii_hash)
        .sum::<u32>();
    println!("Day 15 : Part 1 : {}", total);
}

fn ascii_hash(s: &str) -> u32 {
    s.as_bytes().iter()
        .fold(0, |mut acc, x| {
            acc += *x as u32;
            acc *= 17;
            acc %= 256;
            acc
        })
}

pub fn part_02() {
    let input = include_str!("../inputs/input_15");
    let mut boxes: Vec<Vec<(&str, u8)>> = vec![Vec::new(); 256];
    input.lines().next().unwrap()
        .split(",")
        .for_each(|c| {
            let pivot = c.find(|x| x == '-' || x == '=').unwrap();
            let label = &c[..pivot];
            let box_i = ascii_hash(label) as usize;
            let opt_i = boxes[box_i]
                .iter()
                .enumerate()
                .find_map(|(i, x)| if x.0 == label { Some(i) } else { None });
            if pivot + 1 == c.len() { // Remove
                if let Some(i) = opt_i {
                    boxes[box_i].remove(i);
                }
            } else { // Add
                let lens = c[pivot + 1..].parse::<u8>().unwrap();
                match opt_i {
                    None => boxes[box_i].push((label, lens)),
                    Some(i) => boxes[box_i][i] = (label, lens),
                }
            }
        });
    let total = boxes.iter().enumerate()
        .flat_map(|(i, b)| {
            b.iter().enumerate()
                .map(move |x| (1 + i) * (x.0 + 1) * x.1.1 as usize)
        })
        .sum::<usize>();
    println!("Day 15 : Part 2 : {}", total);
}
