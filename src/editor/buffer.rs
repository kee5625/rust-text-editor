use std::io::Error;

pub struct Buffer {
    pub vec: Vec<String>,
}

impl Buffer {
    pub fn init() -> Result<(), Error> {
        let mut vec = Buffer::vec;
        vec.push("Hello, World!");
    }
}