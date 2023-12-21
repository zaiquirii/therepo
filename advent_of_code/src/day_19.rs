use std::collections::HashMap;
use std::ops::Range;
use std::str::Lines;

#[derive(Debug)]
struct Part {
    x: u32,
    m: u32,
    a: u32,
    s: u32,
}

impl Part {
    fn total(&self) -> u32 { self.x + self.m + self.a + self.s }
}

#[derive(Debug)]
struct Rule {
    field: char,
    op: char,
    value: u32,
    next: String,
}

impl Rule {
    fn new_ranges(&self, part: &PartRange) -> (Option<PartRange>, Option<PartRange>) {
        if self.op == 'T' {
            return (Some(part.clone()), None);
        }
        let mut a_part = None;
        let mut r_part = None;
        let field_i = match self.field {
            'x' => 0,
            'm' => 1,
            'a' => 2,
            's' => 3,
            _ => panic!("asdf")
        };
        let (a_range, r_range) = match self.op {
            '<' => (1..self.value - 1, self.value..4000),
            '>' => (self.value+1..4000, 1..self.value),
            _ => panic!("bad")
        };
        if let Some(new_a_range) = intersection(a_range, part.ranges[field_i].clone()) {
            let mut n = part.clone();
            n.ranges[field_i] = new_a_range;
            a_part = Some(n);
        }

        if let Some(new_r_range) = intersection(r_range, part.ranges[field_i].clone()) {
            let mut n = part.clone();
            n.ranges[field_i] = new_r_range;
            r_part = Some(n);
        }
        (a_part, r_part)
    }
}

fn intersection(a: Range<u32>, b: Range<u32>) -> Option<Range<u32>> {
    if b.start > a.end || a.start > b.end {
        None
    } else {
        Some(a.start.max(b.start)..a.end.min(b.end))
    }
}

impl Rule {
    fn process(&self, p: &Part) -> Option<&str> {
        let field = match self.field {
            'x' => p.x,
            'm' => p.m,
            'a' => p.a,
            's' => p.s,
            _ => 0
        };

        let accept = match self.op {
            'T' => true,
            '<' => field < self.value,
            '>' => field > self.value,
            _ => panic!("bad")
        };
        if accept {
            Some(&self.next)
        } else {
            None
        }
    }
}

pub fn part_01() {
    let input = include_str!("../inputs/input_19");
    let mut lines = input.lines();
    let workflows = parse_workflows(&mut lines);
    let total = lines
        .map(parse_part)
        .filter(|p| {
            let r = should_accept(p, &workflows);
            println!("{}", if r { "A" } else { "R" });
            r
        })
        .map(|p| p.total())
        .sum::<u32>();
    println!("Day 19 : Part 1 : {}", total);
}

fn should_accept(part: &Part, workflows: &HashMap<String, Vec<Rule>>) -> bool {
    print!("{:?}", part);
    let mut curr_label = "in";
    loop {
        print!(" {} -> ", curr_label);
        let workflow = workflows.get(curr_label).unwrap();
        for r in workflow {
            if let Some(new_label) = r.process(&part) {
                match new_label {
                    "A" => return true,
                    "R" => return false,
                    other => {
                        curr_label = other;
                        break;
                    }
                }
            }
        }
    }
}

fn parse_workflows(lines: &mut Lines) -> HashMap<String, Vec<Rule>> {
    lines
        .take_while(|l| l.len() > 0)
        .map(|l| {
            let label_end = l.find("{").unwrap();
            let label = String::from(&l[0..label_end]);
            let rules = l[label_end + 1..l.len() - 1]
                .split(",")
                .map(|s| {
                    if let Some((c, w)) = s.split_once(":") {
                        let (f, v) = if let Some(x) = c.split_once("<") {
                            x
                        } else {
                            c.split_once(">").unwrap()
                        };
                        Rule {
                            field: c.chars().next().unwrap(),
                            op: c[f.len()..f.len() + 1].chars().next().unwrap(),
                            value: v.parse::<u32>().unwrap(),
                            next: String::from(w),
                        }
                    } else {
                        Rule {
                            field: 'T',
                            op: 'T',
                            value: 0,
                            next: String::from(s),
                        }
                    }
                })
                .collect::<Vec<_>>();
            (label, rules)
        })
        .collect()
}

fn parse_part(input: &str) -> Part {
    let mut values = input[1..input.len() - 1].split(",");
    Part {
        x: values.next().unwrap()[2..].parse().unwrap(),
        m: values.next().unwrap()[2..].parse().unwrap(),
        a: values.next().unwrap()[2..].parse().unwrap(),
        s: values.next().unwrap()[2..].parse().unwrap(),
    }
}

#[derive(Clone, Debug)]
struct PartRange {
    ranges: [Range<u32>; 4],
}

pub fn part_02() {
    let input = include_str!("../inputs/input_19");
    let mut lines = input.lines();
    let workflows = parse_workflows(&mut lines);
    let total = distinct_accepted(&workflows, "in", PartRange {
        ranges: [1..4000, 1..4000, 1..4000, 1..4000],
    });
    println!("Day 19 : Part 2 : {}", total)
}

fn distinct_accepted(workflows: &HashMap<String, Vec<Rule>>, workflow: &str, part: PartRange) -> u64 {
    if workflow == "A" {
        println!("Accepted: {:?}", part);
        return part.ranges.iter()
            .fold(1u64, |acc, r| acc * (r.len() + 1) as u64);
    }
    if workflow == "R" {
        return 0;
    }

    let workflow = workflows.get(workflow).unwrap();
    let mut sum = 0;
    let mut curr_part = part.clone();
    for rule in workflow {
        let (accepted, rejected) = rule.new_ranges(&curr_part);
        if let Some(p) = accepted {
            sum += distinct_accepted(workflows, &rule.next, p);
        }
        if let Some(p) = rejected {
            curr_part = p;
        } else {
            break;
        }
    }
    sum
}