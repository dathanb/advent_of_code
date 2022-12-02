use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use anyhow::Result;

pub fn part1() -> Result<i32> {
    let lines = read_lines("data/day02.txt")?;
    let mut score = 0;
    for line in lines {
        let line = line.expect("Couldn't get one or more lines");

        let parts: Vec<&str> = line.split(" ").collect();
        if parts[0] == "A" {
            match parts[1] {
                "X" => score += 1 + 3,
                "Y" => score += 2 + 6,
                "Z" => score += 3,
                _ => ()
            };
        } else if parts[0] == "B" {
            match parts[1] {
                "X" => score += 1,
                "Y" => score += 2 + 3,
                "Z" => score += 3 + 6,
                _ => ()
            };
        } else if parts[0] == "C" {
            match parts[1] {
                "X" => score += 1 + 6,
                "Y" => score += 2,
                "Z" => score += 3 + 3,
                _ => ()
            };
        }
    }

    Ok(score)
}


pub fn part2() -> Result<i32> {
    let lines = read_lines("data/day02.txt")?;
    let mut score = 0;
    for line in lines {
        let line = line.expect("Couldn't get one or more lines");

        let parts: Vec<&str> = line.split(" ").collect();
        if parts[0] == "A" {
            match parts[1] {
                "X" => score += 3,
                "Y" => score += 1 + 3,
                "Z" => score += 2 + 6,
                _ => ()
            };
        } else if parts[0] == "B" {
            match parts[1] {
                "X" => score += 1,
                "Y" => score += 2 + 3,
                "Z" => score += 3 + 6,
                _ => ()
            };
        } else if parts[0] == "C" {
            match parts[1] {
                "X" => score += 2,
                "Y" => score += 3 + 3,
                "Z" => score += 1 + 6,
                _ => ()
            };
        }
    }

    Ok(score)
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

#[cfg(test)]
mod tests {
    use crate::day02::{part1, part2};
    use anyhow::Result;

    #[test]
    pub fn test_part1() -> Result<()> {
        let score = part1()?;
        println!("{}", score);
        Ok(())
    }

    #[test]
    pub fn test_part2() -> Result<()> {
        let score = part2()?;
        println!("{}", score);
        Ok(())
    }
}