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

    let mut i = 0;
    let mut opcode = nums[i];
    while opcode != 99 {
        print_vec(&nums);

        let operand1_index = nums[i+1];
        let operand1 = nums[operand1_index as usize];
        let operand2_index = nums[i+2];
        let operand2 = nums[operand2_index as usize];
        let result_index = nums[i+3];
        let result: i32;

        if opcode == 1 {
            result = operand1 + operand2;
        }
        else if opcode == 2 {
            result = operand1 * operand2;
        } else {
            panic!("Unexpected value!");
        }
        println!("IP: {}\topcode: {}\top1idx: {}\top1: {}\top2idx: {}\top2: {}\tridx: {}\tresult: {}", i, opcode,
                 operand1_index, operand1, operand2_index, operand2, result_index, result);
        nums[result_index as usize] = result;

        i += 4;
        opcode = nums[i];
    }

    println!("{}", nums[0]);

    Ok(())
}

fn part2(nums: &Vec<i32>) -> io::Result<()> {
    let mut nums = nums.to_vec();
//    let file = File::open("resources/input.txt").unwrap();
//
//    let reader = BufReader::new(file);
//
//    let mut total: i32 = 0;
//
//    for line_result in reader.lines() {
//        let line = line_result.expect("Error getting next line from file.");
//        let num: i32 = line.parse().unwrap();
//        let mut delta_fuel_weight = (num / 3) - 2;
//        while delta_fuel_weight > 0 {
//            total += delta_fuel_weight;
//            delta_fuel_weight = (delta_fuel_weight / 3) - 2;
//            if delta_fuel_weight < 0 {
//                delta_fuel_weight = 0
//            }
//        }
//    }
//
//    println!("{}", total);
//
    Ok(())
}

fn print_vec(vec: &Vec<i32>) {
    for i in 0..vec.len() {
        print!("{},", vec[i]);
    }
    println!("");
}

