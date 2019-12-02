use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn main() -> io::Result<()> {
    let file = File::open("resources/input.txt").unwrap();

    let reader = BufReader::new(file);

    let mut total: i32 = 0;

    for line_result in reader.lines() {
        let line = line_result.expect("Error getting next line from file.");
        let num: i32 = line.parse().unwrap();
        let weight = (num /3)-2;
        total += weight;
    }

    println!("{}", total);

    Ok(())
}
