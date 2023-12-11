use std::fs::File;
use std::io::Read;
use std::ops::Add;

pub fn read_file(path: &str) -> Result<String, std::io::Error> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

pub struct Point<T: Add<Output=T>> {
    pub x: T,
    pub y: T,
}

impl<T: Add<Output=T>> Point<T> {
    pub fn new(x: T, y: T) -> Self {
        Point { x, y }
    }
}