use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::fmt::Debug;

pub fn read_lines<P>(filename: P) -> Vec<String>
    where P: AsRef<Path> {
    let file = File::open(filename).unwrap();
    io::BufReader::new(file).lines().map(|s| s.unwrap()).collect()
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
