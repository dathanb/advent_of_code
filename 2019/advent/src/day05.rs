use std::fs;

use crate::intcode::Computer;

fn get_input() -> Computer {
    let contents = fs::read_to_string("resources/day05.txt").unwrap();
    Computer::parse(&contents)
}

#[allow(dead_code)]
fn part1() -> String {
    let mut computer = get_input();
    computer.input.push(2);
    computer.run().unwrap();

    format!("{:?}", computer.output.last().unwrap())
}

#[allow(dead_code)]
fn part2() -> String {
    let mut computer = get_input();
    computer.input.push(5);
    computer.run().unwrap();
    format!("{:?}", computer.output.last().unwrap())
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1(), "5821753");
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(), "11956381");
    }
}
