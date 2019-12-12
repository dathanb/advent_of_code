use std::io;
use std::fs;

use crate::intcode::{Computer};

#[allow(dead_code)]
fn part1() -> Result<String, String> {
    let computer = match get_input() {
        Ok(c) => c,
        Err(n) => return Err(format!("{}", n)),
    };

    return compute_part1(&computer);
}

fn compute_part1(computer: &Computer) -> Result<String, String> {

    Err(String::from("Not yet implemented"))
}

fn get_input() -> io::Result<Computer> {
    let contents = fs::read_to_string("resources/day07.txt")?;
    Ok(Computer::parse(&contents))
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_input1() {
        let input = "3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0";
        let expected_output = "43210";
        let computer= Computer::parse(input);
        let actual_output = compute_part1(&computer).unwrap();
        assert_eq!(actual_output, expected_output);
    }

    #[test]
    fn test_part1_input2() {
        let input = "3,23,3,24,1002,24,10,24,1002,23,-1,23,101,5,23,23,1,24,23,23,4,23,99,0,0";
        let expected_output = "54321";
        let computer= Computer::parse(input);
        let actual_output = compute_part1(&computer).unwrap();
        assert_eq!(actual_output, expected_output);
    }

    #[test]
    fn test_part1_input3() {
        let input = "3,31,3,32,1002,32,10,32,1001,31,-2,31,1007,31,0,33,1002,33,7,33,1,33,31,31,1,32,31,31,4,31,99,0,0,0";
        let expected_output = "65210";
        let computer= Computer::parse(input);
        let actual_output = compute_part1(&computer).unwrap();
        assert_eq!(actual_output, expected_output);
    }
}