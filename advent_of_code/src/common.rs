use std::fs::File;
use std::io::Read;
use std::ops::{Add, Sub};

pub fn read_file(path: &str) -> Result<String, std::io::Error> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq, Default)]
pub struct Point<T: Copy + Add<Output=T>> {
    pub x: T,
    pub y: T,
}

impl<T: Copy + Add<Output=T> + Sub<Output=T>> Point<T> {
    pub fn new(x: T, y: T) -> Self {
        Point { x, y }
    }

    pub fn add(&self, other: Self) -> Self {
        Point::new(self.x + other.x, self.y + other.y)
    }

    pub fn sub(&self, other: Self) -> Self {
        Point::new(self.x - other.x, self.y - other.y)
    }

    pub fn offset(&self, x: T, y: T) -> Self {
        Point::new(self.x + x, self.y + y)
    }
}

pub fn lcm(l: u64, r: u64) -> u64 {
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

pub fn prime_factors(mut x: u64) -> Vec<u64> {
    if x == 1 {
        return vec![1]
    }

    let mut result = vec![1];
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
