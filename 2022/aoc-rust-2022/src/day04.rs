use anyhow::Result;
use crate::shared::read_lines;

pub fn day04_part1(path: &str) -> Result<i32> {
    let lines = read_lines(path);
    todo!()
}


pub fn day04_part2(path: &str) -> Result<i32> {
    let lines = read_lines(path);
    todo!()
}

#[cfg(test)]
mod tests {
    use anyhow::Result;
    use crate::day04::{day04_part1, day04_part2};

    #[test]
    pub fn test_part1() -> Result<()> {
        let score = day04_part1("data/day04_test.txt")?;
        println!("{}", score);
        assert_eq!(2, score);
        Ok(())
    }

    #[test]
    pub fn run_part1() -> Result<()> {
        let score = day04_part1("data/day04.txt")?;
        println!("{}", score);
        assert_eq!(0, score);
        Ok(())
    }

    #[test]
    pub fn test_part2() -> Result<()> {
        let score = day04_part2("data/day04_test.txt")?;
        println!("{}", score);
        assert_eq!(0, score);
        Ok(())
    }

    #[test]
    pub fn run_part2() -> Result<()> {
        let score = day04_part2("data/day04.txt")?;
        println!("{}", score);
        assert_eq!(0, score);
        Ok(())
    }
}
