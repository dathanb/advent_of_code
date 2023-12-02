use std::env;

mod day01;
mod day02;
mod shared;
mod day03;
mod day05;
mod day04;
mod day06;
mod day07;
mod day08;
mod day09;
mod coordinate;
mod day10;

fn main() {
    let args: Vec<String> = env::args().collect();
    match args[1].as_str() {
        "1.1" => {day01::part1().unwrap();}
        "1.2" => {day01::part2().unwrap();}
        "2.1" => {day02::part1().unwrap();}
        "2.2" => {day02::part2().unwrap();}
        "3.1" => {day03::day3_part1().unwrap();}
        "3.2" => {day03::day3_part2().unwrap();}
        "4.1" => {day04::day04_part1("data/day04.txt").unwrap();}
        "4.1.test" => {day04::day04_part1("data/day04_test.txt").unwrap();}
        "4.2" => {day04::day04_part2("data/day04.txt").unwrap();}
        "4.2.test" => {day04::day04_part2("data/day04_test.txt").unwrap();}
        "5.1" => {day05::day05_part1("data/day05.txt").unwrap();}
        "5.1.test" => {day05::day05_part1("data/day05_test.txt").unwrap();}
        "5.2" => {day05::day05_part2("data/day05.txt").unwrap();}
        "5.2.test" => {day05::day05_part2("data/day05_test.txt").unwrap();}
        "6.1" => {day06::day06_part1("data/day06.txt").unwrap();}
        "6.1.test" => {day06::day06_part1("data/day06_test.txt").unwrap();}
        "6.2" => {day06::day06_part2("data/day06.txt").unwrap();}
        "6.2.test" => {day06::day06_part2("data/day06_test.txt").unwrap();}
        "7.1" => {day07::day07_part1("data/day07.txt").unwrap();}
        "7.1.test" => {day07::day07_part1("data/day07_test.txt").unwrap();}
        "7.2" => {day07::day07_part2("data/day07.txt").unwrap();}
        "7.2.test" => {day07::day07_part2("data/day07_test.txt").unwrap();}
        "8.1" => {day08::day08_part1("data/day08.txt").unwrap();}
        "8.1.test" => {day08::day08_part1("data/day08_test.txt").unwrap();}
        "8.2" => {day08::day08_part2("data/day08.txt").unwrap();}
        "8.2.test" => {day08::day08_part2("data/day08_test.txt").unwrap();}
        "9.1" => {day09::day09_part1("data/day09.txt").unwrap();}
        "9.1.test" => {day09::day09_part1("data/day09_test.txt").unwrap();}
        "9.2" => {day09::day09_part2("data/day09.txt").unwrap();}
        "9.2.test" => {day09::day09_part2("data/day09_test.txt").unwrap();}
        _ => println!("Didn't understand the argument")

    };


}


