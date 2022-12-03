use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use anyhow::Result;
use std::fmt::Debug;

pub fn read_lines<P>(filename: P) -> Result<io::Lines<io::BufReader<File>>>
    where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}


pub trait SelfPrint {
    fn print(self) -> Self;
}

impl <T: Debug> SelfPrint for T {
    fn print(self) -> Self {
        println!("{:?}", self);
        self
    }
}
