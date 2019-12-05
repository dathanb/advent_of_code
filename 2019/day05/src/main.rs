use std::fs;
use std::io::{self, prelude::*};


fn main() -> io::Result<()> {
    let contents = fs::read_to_string("resources/input.txt").unwrap();
    let nums: Vec<i32> = contents.split(",")
        .map(|n| n.trim())
        .map(|n| n.parse::<i32>())
        .filter(|n| n.is_ok())
        .map(|n| n.unwrap())
        .collect();

    part1(&nums).unwrap();

    part2(&nums).unwrap();

    Ok(())
}

fn part1(nums: &Vec<i32>) -> io::Result<()> {
    let mut nums = nums.to_vec();

    nums[1] = 12;
    nums[2] = 2;

    println!("{}", compute(&nums));

    Ok(())
}

fn part2(nums: &Vec<i32>) -> io::Result<()> {
    for noun in 0..nums.len() {
        for verb in 0..nums.len() {
            let mut nums = nums.to_vec();
            nums[1] = noun as i32;
            nums[2] = verb as i32;
            let result= compute(&nums);
            if result == 19690720 {
                println!("{}", 100 * noun + verb);
            }
        }
    }

    Ok(())
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
//        println!("IP: {}\topcode: {}\top1idx: {}\top1: {}\top2idx: {}\top2: {}\tridx: {}\tresult: {}", i, opcode,
//                 operand1_index, operand1, operand2_index, operand2, result_index, result);
        memory[result_index as usize] = result;

        i += 4;
        opcode = memory[i];
    }

    memory[0]
}

fn print_vec(vec: &Vec<i32>) {
    for i in 0..vec.len() {
        print!("{},", vec[i]);
    }
    println!("");
}

