use anyhow::Result;
use crate::shared::read_lines;

pub fn dayn_part1(path: &str) -> Result<i32> {
    let lines = read_lines(path);
    todo!()
}


pub fn dayn_part2(path: &str) -> Result<i32> {
    let lines = read_lines(path);
    todo!()
}

#[cfg(test)]
mod tests {
    use anyhow::Result;
    use crate::dayn::{dayn_part1, dayn_part2};

    #[test]
    pub fn test_part1() -> Result<()> {
        let score = dayn_part1("data/dayn_test.txt")?;
        println!("{}", score);
        assert_eq!(0, score);
        Ok(())
    }

    #[test]
    pub fn run_part1() -> Result<()> {
        let score = dayn_part1("data/dayn.txt")?;
        println!("{}", score);
        assert_eq!(0, score);
        Ok(())
    }

    #[test]
    pub fn test_part2() -> Result<()> {
        let score = dayn_part2("data/dayn_test.txt")?;
        println!("{}", score);
        assert_eq!(0, score);
        Ok(())
    }

    #[test]
    pub fn run_part2() -> Result<()> {
        let score = dayn_part2("data/dayn.txt")?;
        println!("{}", score);
        assert_eq!(0, score);
        Ok(())
    }
}
