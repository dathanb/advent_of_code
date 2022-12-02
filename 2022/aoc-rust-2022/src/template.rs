use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use anyhow::Result;
use crate::shared::read_lines;

pub fn part1() -> Result<i32> {
    let lines = read_lines("data/day02.txt")?;
}


pub fn part2() -> Result<i32> {
    let lines = read_lines("data/day02.txt")?;
}

#[cfg(test)]
mod tests {
    use anyhow::Result;

    #[test]
    pub fn test_part1() -> Result<()> {
        let score = dayn_part1()?;
        println!("{}", score);
        assert_eq!(12679, score);
        Ok(())
    }

    #[test]
    pub fn test_part2() -> Result<()> {
        let score = dayn_part2()?;
        println!("{}", score);
        assert_eq!(14470, score);
        Ok(())
    }
}
