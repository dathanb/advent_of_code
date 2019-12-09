use std::fs;
use std::io::{self, prelude::*};

fn get_day02_input() -> io::Result<Vec<i32>> {
    let contents = fs::read_to_string("resources/day02.txt")?;
    Ok(contents.split(",")
        .map(|n| n.trim())
        .map(|n| n.parse::<i32>())
        .filter(|n| n.is_ok())
        .map(|n| n.unwrap())
        .collect())
}

fn part1() -> Result<String, &'static str> {
    let result = get_day02_input();
    if result.is_err() {
        return Err("Error getting input");
    }
    let mut nums = result.unwrap();

    nums[1] = 12;
    nums[2] = 2;

    println!("{}", compute(&nums));

    Ok(format!("{}", compute(&nums)))
}

fn part2() -> Result<String, &'static str> {
    let mut nums = get_day02_input().unwrap();
    for noun in 0..nums.len() {
        for verb in 0..nums.len() {
            let mut nums = nums.to_vec();
            nums[1] = noun as i32;
            nums[2] = verb as i32;
            let result= compute(&nums);
            if result == 19690720 {
                return Ok(format!("{}", 100 * noun + verb));
            }
        }
    }

    Err("Answer not found")
}

fn compute(initial_memory: &Vec<i32>) -> i32 {
    let mut memory = initial_memory.to_vec();

    let mut i = 0;
    let mut opcode = memory[i];
    while opcode != 99 {
//        print_vec(&memory);

        let operand1_index = memory[i+1];
        let operand1 = memory[operand1_index as usize];
        let operand2_index = memory[i+2];
        let operand2 = memory[operand2_index as usize];
        let result_index = memory[i+3];
        let result: i32;

        if opcode == 1 {
            result = operand1 + operand2;
        }
        else if opcode == 2 {
            result = operand1 * operand2;
        } else {
            panic!("Unexpected value!");
        }
        memory[result_index as usize] = result;

        i += 4;
        opcode = memory[i];
    }

    memory[0]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let res = part1().unwrap();
        assert_eq!(res, "2842648");
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2().unwrap(), "9074");
    }
}
