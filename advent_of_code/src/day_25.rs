use std::collections::{HashMap, HashSet, VecDeque};
use rand::prelude::*;

#[derive(Default, Debug)]
struct Graph<'a> {
    edges: HashMap<&'a str, Vec<&'a str>>,
    // rev_edges: HashMap<&'a str, Vec<&'a str>>,
    //
    // end_sets: HashMap<&'a str, HashSet<&'a str>>,
}

#[derive(Debug)]
struct Edge {
    val: String,
    og: [String; 2],
}

impl<'a> Graph<'a> {
    fn size(&self, start: &str) -> usize {
        let mut visited = HashSet::new();
        let mut queue = Vec::new();
        queue.push(start);
        let mut count = 0;
        while let Some(curr) = queue.pop() {
            if !visited.insert(curr) {
                continue;
            }

            count += 1;
            for n in self.edges.get(curr).unwrap() {
                queue.push(n);
            }
        }
        count
    }

    fn remove_edge(&mut self, from: &str, to: &str) {
        let froms = self.edges.get_mut(from).unwrap();
        if let Some(i) = froms.iter().position(|x| *x == to) {
            froms.remove(i);
        }
    }

    fn kargers(&self) -> HashMap<&'a str, Vec<Edge>> {
        let mut graph = self.edges.iter()
            .map(|(l, e)| {
                (
                    *l,
                    e.iter()
                        .map(|x| Edge { val: x.to_string(), og: [l.to_string(), x.to_string()] })
                        .collect::<Vec<_>>()
                )
            })
            .collect::<HashMap<_, _>>();

        while graph.len() > 2 {
            let (keep, merge) = select_edge(&graph);
            if keep == merge {
                continue;
            }
            let merges = graph.remove(merge.as_str()).unwrap();
            let keeps = graph.get_mut(keep.as_str()).unwrap();

            for m in merges {
                keeps.push(m);
            }
            let mut i = 0;
            while i < keeps.len() {
                let e = &keeps[i].val;
                if *e == merge || *e == keep {
                    keeps.swap_remove(i);
                } else {
                    i += 1;
                }
            }

            for n in graph.values_mut() {
                for e in n {
                    if e.val == merge {
                        e.val = keep.clone();
                    }
                }
            }
        }
        graph
    }
}

fn select_edge<'a>(graph: &HashMap<&str, Vec<Edge>>) -> (String, String) {
    let i: usize = random::<usize>() % graph.len();
    let node = graph.iter().skip(i).next().unwrap();
    let i = random::<usize>() % node.1.len();
    (node.0.to_string(), node.1[i].val.clone())
}

fn parse_input(input: &str) -> Graph {
    let mut graph: Graph = Graph::default();
    for l in input.lines() {
        let (label, connections) = l.split_once(": ").unwrap();
        for c in connections.split_ascii_whitespace() {
            graph.edges.entry(label)
                .and_modify(|v| v.push(c))
                .or_insert(vec![c]);
            graph.edges.entry(c)
                .and_modify(|v| v.push(label))
                .or_insert(vec![label]);
        }
    }
    graph
}

pub fn part_01() {
    let input = include_str!("../inputs/input_25");
    let mut graph = parse_input(input);
    let mut g = graph.kargers();
    let mut iter_count = 1;
    while g.values().next().unwrap().len() > 3 {
        iter_count += 1;
        g = graph.kargers();
    }

    let edges = g.values().next().unwrap().iter().map(|x| x.og.clone()).collect::<Vec<_>>();
    for e in &edges {
        graph.remove_edge(&e[0], &e[1]);
        graph.remove_edge(&e[1], &e[0]);
    }
    println!("Iterations: {} : Edges: {:?}", iter_count, edges);
    let [a, b] = &edges[0];
    let size_a = graph.size(a);
    let size_b = graph.size(b);
    println!("Day 25 : Part : 1 : {} X {} = {}", size_a, size_b, size_a * size_b);
}

pub fn part_02() {}