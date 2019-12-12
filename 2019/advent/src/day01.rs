use std::fs::File;
use std::io::{prelude::*, BufReader};

#[allow(dead_code)]
pub fn part1() -> Result<String, &'static str> {
    let file = File::open("resources/day01.txt").unwrap();

    let reader = BufReader::new(file);

    let mut total: i32 = 0;

    for line_result in reader.lines() {
        let line = line_result.expect("Error getting next line from file.");
        let num: i32 = line.parse().unwrap();
        let weight = (num /3)-2;
        total += weight;
    }

    let res = format!("{}", total);

    Ok(res)
}

#[allow(dead_code)]
pub fn part2() -> Result<String, &'static str> {
    let file = File::open("resources/day01.txt").unwrap();

    let reader = BufReader::new(file);

    let mut total: i32 = 0;

    for line_result in reader.lines() {
        let line = line_result.expect("Error getting next line from file.");
        let num: i32 = line.parse().unwrap();
        let mut delta_fuel_weight = (num / 3) - 2;
        while delta_fuel_weight > 0 {
            total += delta_fuel_weight;
            delta_fuel_weight = (delta_fuel_weight / 3) - 2;
            if delta_fuel_weight < 0 {
                delta_fuel_weight = 0
            }
        }
    }

    let res = format!("{}", total);

    Ok(res)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let res = part1().unwrap();
        assert_eq!(res, "3331523");
    }

    #[test]
    fn test_part2() {
        let res = part2().unwrap();
        assert_eq!(res, "4994396");
    }
}
