use std::collections::HashSet;
use anyhow::{anyhow, Result};
use crate::shared::read_lines;

pub fn day06_part1(path: &str) -> Result<i32> {
    let lines = read_lines(path);
    let line = lines.into_iter().next().unwrap();

    for start in 0..line.len() {
        let chars:HashSet<char> = line.chars().skip(start).take(4).collect();
        if chars.len() == 4 {
            return Ok(4 + start as i32);
        }
    }

    Err(anyhow!("Couldn't find a marker"))
}


pub fn day06_part2(path: &str) -> Result<i32> {
    let lines = read_lines(path);
    let line = lines.into_iter().next().unwrap();

    for start in 0..line.len() {
        let chars:HashSet<char> = line.chars().skip(start).take(14).collect();
        if chars.len() == 14 {
            return Ok(14 + start as i32);
        }
    }

    Err(anyhow!("Couldn't find a marker"))
}

#[cfg(test)]
mod tests {
    use anyhow::Result;
    use crate::day06::{day06_part1, day06_part2};

    #[test]
    pub fn test_part1() -> Result<()> {
        let score = day06_part1("data/day06_test.txt")?;
        println!("{}", score);
        assert_eq!(7, score);
        Ok(())
    }

    #[test]
    pub fn run_part1() -> Result<()> {
        let score = day06_part1("data/day06.txt")?;
        println!("{}", score);
        assert_eq!(0, score);
        Ok(())
    }

    #[test]
    pub fn test_part2() -> Result<()> {
        let score = day06_part2("data/day06_test.txt")?;
        println!("{}", score);
        assert_eq!(19, score);
        Ok(())
    }

    #[test]
    pub fn run_part2() -> Result<()> {
        let score = day06_part2("data/day06.txt")?;
        println!("{}", score);
        assert_eq!(0, score);
        Ok(())
    }
}
