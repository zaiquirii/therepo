use crate::common;

pub fn part_01() {
    let input = common::read_file("inputs/input_02").unwrap();

    let limit = Pull {
        red: 12,
        green: 13,
        blue: 14,
    };

    let total: u32 = input.lines()
        .map(|x| parse_game(x).unwrap())
        .filter(|g| g.valid(limit))
        .map(|g| g.id)
        .sum();
    println!("Day 02 : Part 1 : {}", total);
}

pub fn part_02() {
    let input = common::read_file("inputs/input_02").unwrap();
    let total: u32 = input.lines()
        .map(|x| parse_game(x).unwrap())
        .map(|g| g.min_pull().power())
        .sum();
    println!("Day 02 : Part 2 : {}", total);
}

fn parse_game(input: &str) -> Option<Game> {
    let parts: Vec<_> = input.split(": ").collect();
    let game = parts[0].split(" ").last()?.parse::<u32>().ok()?;

    let pulls: Vec<_> = parts[1].split("; ")
        .map(parse_pull)
        .collect();
    Some(Game {
        id: game,
        pulls,
    })
}

fn parse_pull(input: &str) -> Pull {
    let mut pull = Pull { red: 0, green: 0, blue: 0 };
    input
        .split(", ")
        .map(|t| t.split(" ").collect::<Vec<_>>())
        .for_each(|p| {
            let value = p[0].parse::<u32>().unwrap();
            match p[1] {
                "red" => { pull.red = value }
                "blue" => { pull.blue = value }
                "green" => { pull.green = value }
                _ => {}
            }
        });
    pull
}


#[derive(Debug, Copy, Clone)]
struct Pull {
    red: u32,
    green: u32,
    blue: u32,
}

impl Pull {
    fn power(&self) -> u32 {
        self.red * self.green * self.blue
    }
}

#[derive(Debug)]
struct Game {
    id: u32,
    pulls: Vec<Pull>,
}

impl Game {
    fn valid(&self, limit: Pull) -> bool {
        self.pulls.iter()
            .all(|p| p.red <= limit.red &&
                p.blue <= limit.blue &&
                p.green <= limit.green
            )
    }

    fn min_pull(&self) -> Pull {
        let out = Pull {
            red: 0,
            green: 0,
            blue: 0,
        };
        self.pulls.iter()
            .fold(out, |mut acc, p| {
                acc.red = acc.red.max(p.red);
                acc.blue = acc.blue.max(p.blue);
                acc.green = acc.green.max(p.green);
                acc
            })
    }
}
