use std::io::{self, BufRead};
use std::path::Path;
use anyhow::Result;
use crate::shared::read_lines;

pub fn dayn_part1() -> Result<i32> {
    let lines = read_lines("data/dayn.txt")?;
    todo!()
}


pub fn dayn_part2() -> Result<i32> {
    let lines = read_lines("data/dayn.txt")?;
    todo!()
}

#[cfg(test)]
mod tests {
    use anyhow::Result;
    use crate::dayn::{dayn_part1, dayn_part2};

    #[test]
    pub fn test_part1() -> Result<()> {
        let score = dayn_part1()?;
        println!("{}", score);
        assert_eq!(0, score);
        Ok(())
    }

    #[test]
    pub fn test_part2() -> Result<()> {
        let score = dayn_part2()?;
        println!("{}", score);
        assert_eq!(0, score);
        Ok(())
    }
}
