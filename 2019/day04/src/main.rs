use std::fs;

fn main() {
    let contents = fs::read_to_string("resources/input.txt").unwrap();

    part1(&contents);

    println!("Hello, world!");
}

fn part1(input: &str) {
    let parts: Vec<i32> = input.split("-")
        .map(|s| s.parse::<i32>().unwrap())
        .collect();

    let lower_bound = parts[0];
    let upper_bound = parts[1];

    let mut count = 0;

    for n in lower_bound .. upper_bound+1 {
        if has_six_digits(n) && has_repeat(n) && is_non_decreasing(n) {
            count += 1;
        }
    }

    println!("{}", count);
}

fn has_six_digits(n: i32) -> bool {
    n >= 100000 && n <= 999999
}

fn has_repeat(n: i32) -> bool {
    let digits: Vec<i32> = get_digits(n);
    let mut repeat = false;
    for i in 0..digits.len()-1 {
        if digits[i] == digits[i+1] {
            repeat = true;
        }
    }
    repeat
}

fn is_non_decreasing(n: i32) -> bool {
    let digits: Vec<i32> = get_digits(n);
    for i in 0..digits.len()-1 {
        if digits[i+1] < digits[i] {
            return false;
        }
    }

    true
}

fn get_digits(n: i32) -> Vec<i32> {
    let mut digits: Vec<i32> = Vec::new();
    let mut current = n;
    while current > 0 {
        digits.push(current % 10);
        current /= 10;
    }
    digits.reverse();
    digits
}

