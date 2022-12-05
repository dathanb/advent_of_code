use anyhow::Result;
use crate::shared::read_lines;

pub fn part1() -> Result<i32> {
    let mut elves = get_elves(get_ints(read_lines("data/day01.txt")));

    elves.sort();
    Ok(elves.iter().rev().take(1).sum::<i32>())
}

pub fn part2() -> Result<i32> {
    let mut elves = get_elves(get_ints(read_lines("data/day01.txt")));

    elves.sort();
    Ok(elves.iter().rev().take(3).sum::<i32>())
}

fn get_elves(calories: Vec<Option<i32>>) -> Vec<i32> {
    let mut vec = Vec::new();
    let mut accumulator = 0;
    for i in calories {
        match i {
            Some(n) => accumulator += n,
            None => { vec.push(accumulator); accumulator = 0; }
        }
    }

    vec
}

fn get_ints(lines: Vec<String>) -> Vec<Option<i32>> {
    lines.iter().map(|l| l.parse::<i32>().map_or(None, |i| Some(i))).collect()
}

#[cfg(test)]
mod tests {
    use crate::day01::{get_elves, get_ints, part1, part2, read_lines};
    use anyhow::Result;

    #[test]
    pub fn test_part1() -> Result<()>{
        assert_eq!(73211, part1()?);
        Ok(())
    }

    #[test]
    pub fn test_part2() -> Result<()>{
        assert_eq!(213958, part2()?);
        Ok(())
    }

    #[test]
    pub fn test_get_ints() -> Result<()> {
        println!("{:?}", get_ints(read_lines("data/day01.txt")));
        Ok(())
    }

    #[test]
    pub fn test_get_elves() -> Result<()> {
        println!("{:?}", get_elves(get_ints(read_lines("data/day01.txt"))));
        Ok(())
    }
}