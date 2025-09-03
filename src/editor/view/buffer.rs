use std::fs::read_to_string;
use std::io::Error;

#[derive(Default)]
pub struct Buffer {
    pub tuples: Vec<Vec<String>>,
}

impl Buffer {
    pub fn load(file_name: &str) -> Result<Self, Error> {
        let contents = read_to_string(file_name)?;
        let mut tuples = Vec::new();
        for line in contents.lines() {
            let mut tuple = Vec::new();

            for el in line.split(',') {
                tuple.push(String::from(el));
            }

            tuples.push(tuple);
        }
        Ok(Self { tuples })
    }

    pub fn is_empty(&self) -> bool {
        self.tuples.is_empty()
    }
}
