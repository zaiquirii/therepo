use std::collections::HashMap;
use regex::Regex;

#[derive(Debug)]
struct Node {
    left: (String, bool),
    right: (String, bool),
}

pub fn part_01() {
    let re = Regex::new(r"(\w+) = \(([A-Z0-9]+)+, ([A-Z0-9]+)\)").unwrap();
    let input = include_str!("../inputs/input_08");
    let mut lines = input.lines();

    let directions = lines.next().unwrap();
    let _ = lines.next();

    let map: HashMap<_, _> = lines
        .map(|l| re.captures(l).unwrap().extract())
        .map(|(_, [n, l, r])|
            (n, Node { left: (l.into(), l.ends_with("Z")), right: (r.into(), r.ends_with("Z")) })
        )
        .collect();

    let moves = count_moves("AAA", directions, &map);
    println!();
    println!("Day 08 : Part 1 : {}", moves)
}

pub fn part_02_better() {
    println!("{} {} {}", lcm(2, 4), lcm(100, 25), lcm(4, 6));
    let re = Regex::new(r"(\w+) = \(([A-Z0-9]+)+, ([A-Z0-9]+)\)").unwrap();
    let input = include_str!("../inputs/input_08");
    let mut lines = input.lines();

    let directions = lines.next().unwrap();
    let _ = lines.next();

    let map: HashMap<_, _> = lines
        .map(|l| re.captures(l).unwrap().extract())
        .map(|(_, [n, l, r])|
            (n, Node { left: (l.into(), l.ends_with("Z")), right: (r.into(), r.ends_with("Z")) })
        )
        .collect();

    let moves = map.keys()
        .filter(|x| x.ends_with("A"))
        .map(|x| count_moves(x, directions, &map))
        .reduce(|acc, x| lcm(acc, x))
        .unwrap();

    println!("Day 08 : Part 1 : {}", moves)
}

fn count_moves(start: &str, directions: &str, map: &HashMap<&str, Node>) -> u64 {
    let mut moves = 0;
    let mut current = start;
    'outer: loop {
        for c in directions.chars() {
            let node = map.get(current).unwrap();
            let (next, terminal) = match c {
                'L' => &node.left,
                'R' => &node.right,
                _ => panic!("SHOULDN'T GET HERE")
            };
            current = next;
            moves += 1;
            if *terminal {
                break 'outer;
            }
        }
    }
    moves
}

fn lcm(l: u64, r: u64) -> u64 {
    let pf_l = prime_factors(l);
    let mut pf_r = prime_factors(r);

    let mut result = 1;
    pf_l.iter().for_each(|x| {
        result *= x;

        for i in 0..pf_r.len() {
            if pf_r[i] == *x {
                pf_r.remove(i);
                break;
            }
        }
    });
    pf_r.iter().for_each(|x| result *= x);
    result
}

fn prime_factors(mut x: u64) -> Vec<u64> {
    if x == 1 {
        return vec![1]
    }

    let mut result = Vec::new();
    let mut div = 2;
    while div <= x {
        if x % div == 0 {
            x /= div;
            result.push(div);
        } else {
            div += 1;
        }
    }
    result
}

// type NodeID = usize;
// type DirID = usize;
// type LeftRight = [Option<(NodeID, DirID, usize)>; 2];
//
// struct Cache {
//     lefts: Vec<(usize, bool)>,
//     rights: Vec<(usize, bool)>,
//     directions: Vec<char>,
//
//     saved: HashMap<NodeID, HashMap<DirID, LeftRight>>,
// }
//
// impl Cache {
//     fn next(&mut self, node: NodeID, dir: DirID) -> (NodeID, DirID, usize) {
//         let node_map = self.saved.entry(node).or_insert_with(|| HashMap::new());
//         let dir_map = node_map.entry(dir).or_insert_with(|| [None, None]);
//         let dir_index = if self.directions[dir] == 'L' { 0 } else { 1 };
//         let turn = &mut dir_map[dir_index];
//         if let Some(x) = turn {
//             return x.clone();
//         }
//
//         let mut current = node;
//         let mut dir_index = dir;
//         let mut moves = 0;
//
//         loop {
//             let next_turn = self.directions[dir_index];
//             let (next, terminate) = if next_turn == 'L' {
//                 self.lefts[current]
//             } else {
//                 self.rights[current]
//             };
//             moves += 1;
//             current = next;
//             dir_index = (dir_index + 1) % self.directions.len();
//
//             if terminate {
//                 *turn = Some((current, dir_index, moves));
//                 return (current, dir_index, moves);
//             }
//         }
//     }
// }
//
// pub fn _part_02_this_one_worked_for_all_cycle_types() {
//     let re = Regex::new(r"(\w+) = \(([A-Z0-9]+)+, ([A-Z0-9]+)\)").unwrap();
//     let input = include_str!("../inputs/input_08");
//     let mut lines = input.lines();
//
//     let directions = lines.next().unwrap().chars().collect();
//     let _ = lines.next();
//
//     let mut lefts = Vec::new();
//     let mut rights = Vec::new();
//
//     let mut keys = Vec::new();
//     let mut positions = Vec::new();
//
//     let lines: Vec<_> = lines.collect();
//
//     lines.iter()
//         .map(|l| re.captures(l).unwrap().extract::<3>())
//         .for_each(|x| keys.push(x.1[0]));
//
//     lines
//         .iter()
//         .map(|l| re.captures(l).unwrap().extract())
//         .for_each(|(_, [n, l, r])| {
//             let n_id = id(&keys, n);
//             if n.ends_with("A") {
//                 positions.push(n_id);
//             }
//             let l_id = id(&keys, l);
//             let r_id = id(&keys, r);
//
//             lefts.push((l_id, l.ends_with("Z")));
//             rights.push((r_id, r.ends_with("Z")));
//         });
//     let mut cache = Cache {
//         lefts,
//         rights,
//         directions,
//         saved: HashMap::new(),
//     };
//
//     let mut currents: Vec<_> = positions.iter()
//         .map(|x| (*x, 0, 0))
//         .collect();
//
//     println!("KEYS: {:?}", keys);
//     println!("STEP: {:?}", currents.iter().map(|x| (keys[x.0], cache.directions[x.1], x.2)).collect::<Vec<_>>());
//     let mut final_moves = 0;
//     loop {
//         let (index, current) = currents.iter().enumerate().min_by_key(|x| x.1.2).unwrap();
//         let (new_current, new_dir, move_count) = cache.next(current.0, current.1);
//         let new_moves = move_count + current.2;
//         currents[index] = (new_current, new_dir, new_moves);
//         let all_match = currents.iter().all(|x| x.2 == new_moves);
//         // println!("STEP: {:?} ::: {:?}", currents.iter().map(|x| (keys[x.0], x.1, x.2)).collect::<Vec<_>>(), currents);
//         if all_match {
//             final_moves = new_moves;
//             break;
//         }
//     }
//     println!("Day 08 : Part 2 : {}", final_moves)
// }
//
// fn id<'a>(keys: &Vec<&'a str>, n: &'a str) -> usize {
//     match keys.iter()
//         .enumerate()
//         .find(|(_, x)| **x == n)
//         .map(|x| x.0) {
//         None => {
//             panic!("SHOULDNT HAPPEN");
//         }
//         Some(e) => e
//     }
// }
