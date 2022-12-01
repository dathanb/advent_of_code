use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use anyhow::Result;

pub fn part1() -> Result<()> {
    let lines = read_lines("data/day01.txt")?;

    let mut max = 0;
    let mut current = 0;
    for line in lines {
        let line = line?;
        if line == "" {
            max = std::cmp::max(current, max);
            current = 0;
        } else {
            current += line.parse::<i32>()?;
        }
    }

    println!("{}", max);

    Ok(())
}

pub fn part2() -> Result<()> {

}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}


#[cfg(test)]
mod tests {
    use crate::day01::part1;
    use anyhow::Result;

    #[test]
    pub fn test() -> Result<()>{
        part1()
    }

    #[test]
    pub fn test_part2() -> Result<()> {
        part2()
    }
}