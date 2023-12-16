use std::fs::File;
use std::io::Read;
use std::ops::Add;

pub fn read_file(path: &str) -> Result<String, std::io::Error> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
pub struct Point<T: Copy + Add<Output=T>> {
    pub x: T,
    pub y: T,
}

impl<T: Copy + Add<Output=T>> Point<T> {
    pub fn new(x: T, y: T) -> Self {
        Point { x, y }
    }

    pub fn add(&self, other: Self) -> Self {
        Point::new(self.x + other.x, self.y + other.y)
    }

    pub fn offset(&self, x: T, y: T) -> Self {
        Point::new(self.x + x, self.y + y)
    }
}