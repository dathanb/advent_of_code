use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use anyhow::Result;

pub fn part1() -> Result<()> {
    let mut elves = get_elves(get_ints(read_lines("data/day01.txt")?));

    elves.sort();
    println!("{}", elves[elves.len()-1]);

    Ok(())
}

pub fn part2() -> Result<()> {
    let mut elves = get_elves(get_ints(read_lines("data/day01.txt")?));

    elves.sort();
    let sum = elves[elves.len()-1] + elves[elves.len()-2] + elves[elves.len()-3];
    println!("{}", sum);

    Ok(())
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

fn get_ints(lines: io::Lines<io::BufReader<File>>) -> Vec<Option<i32>> {
    lines.map(|l| match l {
        Ok(i) => i.parse::<i32>().map_or(None, |i| Some(i)),
        _ => panic!("Failed to read one or more lines")
    }).collect()
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}


#[cfg(test)]
mod tests {
    use crate::day01::{get_elves, get_ints, part1, part2, read_lines};
    use anyhow::Result;

    #[test]
    pub fn test_part1() -> Result<()>{
        part1()
    }

    #[test]
    pub fn test_part2() -> Result<()>{
        part2()
    }

    #[test]
    pub fn test_get_ints() -> Result<()> {
        println!("{:?}", get_ints(read_lines("data/day01.txt")?));
        Ok(())
    }

    #[test]
    pub fn test_get_elves() -> Result<()> {
        println!("{:?}", get_elves(get_ints(read_lines("data/day01.txt")?)));
        Ok(())
    }
}