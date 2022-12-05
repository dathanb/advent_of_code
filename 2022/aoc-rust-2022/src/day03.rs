use std::collections::HashSet;
use std::io::{self};
use anyhow::{anyhow, Result};
use crate::shared::read_lines;

pub fn day3_part1() -> Result<i32> {
    let lines = read_lines("data/day03.txt")?;
    let mut sum = 0;
    for line in lines {
        if line.is_err() {
            return Err(anyhow!("One or more lines couldn't be parsed"));
        }
        let line = line.unwrap();
        let compartment1 = &line[0..line.len()/2];
        let compartment2 = &line[line.len()/2..line.len()];

        let mut set1: HashSet<char> = HashSet::new();
        set1.extend(compartment1.chars().into_iter());
        let mut set2: HashSet<char> = HashSet::new();
        set2.extend(compartment2.chars().into_iter());

        let shared = set1.intersection(&set2);
        let shared_char = shared.take(1).next().unwrap();



        sum += priority(shared_char)?;
    }

    Ok(sum)
}


pub fn day3_part2() -> Result<i32> {
    let lines = read_lines("data/day03.txt")?;
    let mut sum = 0;
    let lines: Vec<Result<String, io::Error>> = lines.collect();
    for group in lines.chunks(3) {
        let bag1 = group[0].as_ref().unwrap();
        let bag2 = group[1].as_ref().unwrap();
        let bag3 = group[2].as_ref().unwrap();

        let shared = HashSet::<char>::from_iter(bag1.chars());
        let set2 = HashSet::<char>::from_iter(bag2.chars());
        let set3 = HashSet::<char>::from_iter(bag3.chars());

        let shared: HashSet<char> = shared.intersection(&set2).map(|c| c.clone()).collect();
        let shared: HashSet<char> = shared.intersection(&set3).map(|c| c.clone()).collect();

        sum += shared.iter().map(priority).map(|p| p.unwrap()).sum::<i32>();
    }

    Ok(sum)
}

fn priority(ch: &char) -> Result<i32> {
    Ok(match ch {
        'a' => 1,
        'b' => 2,
        'c' => 3,
        'd' => 4,
        'e' => 5,
        'f' => 6,
        'g' => 7,
        'h' => 8,
        'i' => 9,
        'j' => 10,
        'k' => 11,
        'l' => 12,
        'm' => 13,
        'n' => 14,
        'o' => 15,
        'p' => 16,
        'q' => 17,
        'r' => 18,
        's' => 19,
        't' => 20,
        'u' => 21,
        'v' => 22,
        'w' => 23,
        'x' => 24,
        'y' => 25,
        'z' => 26,
        'A' => 27,
        'B' => 28,
        'C' => 29,
        'D' => 30,
        'E' => 31,
        'F' => 32,
        'G' => 33,
        'H' => 34,
        'I' => 35,
        'J' => 36,
        'K' => 37,
        'L' => 38,
        'M' => 39,
        'N' => 40,
        'O' => 41,
        'P' => 42,
        'Q' => 43,
        'R' => 44,
        'S' => 45,
        'T' => 46,
        'U' => 47,
        'V' => 48,
        'W' => 49,
        'X' => 50,
        'Y' => 51,
        'Z' => 52,
        _ => return Err(anyhow!("unrecognized character {}", ch)),
    })
}


#[cfg(test)]
mod tests {
    use anyhow::Result;
    use crate::day03::{day3_part1, day3_part2};

    #[test]
    pub fn test_part1() -> Result<()> {
        let score = day3_part1()?;
        println!("{}", score);
        assert_eq!(8085, score);
        Ok(())
    }

    #[test]
    pub fn test_part2() -> Result<()> {
        let score = day3_part2()?;
        println!("{}", score);
        assert_eq!(2515, score);
        Ok(())
    }
}
