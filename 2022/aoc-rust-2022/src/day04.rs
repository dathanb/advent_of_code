use anyhow::Result;
use crate::shared::read_lines;

pub fn day04_part1(path: &str) -> Result<i32> {
    let lines = read_lines(path);
    let mut count = 0;
    for line in lines {
        let assignments = line.split(",").collect::<Vec<&str>>();
        let elf1 = ElfRange::new_from_str(assignments[0])?;
        let elf2 = ElfRange::new_from_str(assignments[1])?;
        if elf1.contains(&elf2) || elf2.contains(&elf1) {
            count += 1;
        }
    }
    Ok(count)
}


pub fn day04_part2(path: &str) -> Result<i32> {
    let lines = read_lines(path);
    let mut count = 0;
    for line in lines {
        let assignments = line.split(",").collect::<Vec<&str>>();
        let elf1 = ElfRange::new_from_str(assignments[0])?;
        let elf2 = ElfRange::new_from_str(assignments[1])?;
        if elf1.overlaps(&elf2) {
            count += 1;
        }
    }
    Ok(count)
}

struct ElfRange {
    pub low: i32,
    pub high: i32
}

impl ElfRange {
    pub fn new_from_str(s: &str) -> Result<ElfRange> {
        let parts =s.split("-").collect::<Vec<&str>>();
        Ok(ElfRange {
            low: parts[0].parse::<i32>()?,
            high: parts[1].parse::<i32>()?
        })
    }


    pub fn contains(&self, other: &ElfRange) -> bool {
        self.low <= other.low && self.high >= other.high
    }

    pub fn overlaps(&self, other: &ElfRange) -> bool {
        (self.low <= other.low && self.high >= other.low)
            || (self.low <= other.high && self.high >= other.high)
            || self.contains(other)
            || other.contains(self)
    }
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
        assert_eq!(4, score);
        Ok(())
    }

    #[test]
    pub fn run_part2() -> Result<()> {
        let score = day04_part2("data/day04.txt")?;
        println!("{}", score);
        assert_eq!(857, score);
        Ok(())
    }
}
