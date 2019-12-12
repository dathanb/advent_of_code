use std::fs;

fn get_input() -> Vec<i32> {
    let contents = fs::read_to_string("resources/day04.txt").unwrap();
    let parts: Vec<i32> = contents.split("-")
        .map(|s| s.parse::<i32>().unwrap())
        .collect();

    parts
}

#[allow(dead_code)]
fn part1() -> String {
    let parts = get_input();
    let lower_bound = parts[0];
    let upper_bound = parts[1];

    let mut count = 0;

    for n in lower_bound..upper_bound + 1 {
        if has_six_digits(n) && has_repeat(n) && is_non_decreasing(n) {
            count += 1;
        }
    }

    format!("{}", count)
}

#[allow(dead_code)]
fn part2() -> String {
    let parts = get_input();
    let lower_bound = parts[0];
    let upper_bound = parts[1];

    let mut count = 0;
    for n in lower_bound..upper_bound + 1 {
        if has_six_digits(n) && has_two_digit_only_repeat(n) && is_non_decreasing(n) {
            count += 1;
        }
    }

    format!("{}", count)
}


fn has_six_digits(n: i32) -> bool {
    n >= 100000 && n <= 999999
}

fn has_repeat(n: i32) -> bool {
    let digits: Vec<i32> = get_digits(n);
    let mut repeat = false;
    for i in 0..digits.len() - 1 {
        if digits[i] == digits[i + 1] {
            repeat = true;
        }
    }
    repeat
}

fn has_two_digit_only_repeat(n: i32) -> bool {
    let digits = get_digits(n);
    let mut repeat = 1;
    let mut has_two_digit_repeat = false;
    for i in 0..digits.len() - 1 {
        if digits[i] == digits[i + 1] {
            repeat += 1;
        } else {
            if repeat == 2 {
                has_two_digit_repeat = true;
            }
            repeat = 1;
        }
    }
    if repeat == 2 {
        has_two_digit_repeat = true;
    }

    has_two_digit_repeat
}

fn is_non_decreasing(n: i32) -> bool {
    let digits: Vec<i32> = get_digits(n);
    for i in 0..digits.len() - 1 {
        if digits[i + 1] < digits[i] {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1(), "1610");
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(), "1104");
    }
}
