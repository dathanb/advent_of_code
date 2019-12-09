use std::fs;
use std::io;

mod intcode;
use intcode::Computer;


fn main() -> io::Result<()> {
    let contents = fs::read_to_string("resources/input.txt").unwrap();

    let mut computer = Computer::parse(&contents);

    part1(&computer);

    part2(&computer);

    Ok(())
}

fn part1(computer_arg: &Computer) -> Result<(), ()> {
    let mut computer: Computer = computer_arg.clone();
    computer.input = 2;
    let result = computer.run();

    computer.print();

    if result.is_err() {
        return Err(());
    }

    println!("{}", computer.output);

    Ok(())
}

fn part2(computer_arg: &Computer) -> Result<(), ()> {
    let mut computer: Computer = computer_arg.clone();
    computer.input = 5;
    let result = computer.run();

    computer.print();

    if result.is_err() {
        return Err(());
    }

    println!("{}", computer.output);

    Ok(())
}


fn print_vec(vec: &Vec<i32>) {
    for i in 0..vec.len() {
        print!("{},", vec[i]);
    }
    println!("");
}

