use std::fs;
use std::io;

mod intcode;
use intcode::Computer;


fn main() -> io::Result<()> {
    let contents = fs::read_to_string("resources/input.txt").unwrap();
//    let contents = "3,0,4,0,99";

    let mut computer = Computer::parse(&contents);
    computer.input = 2;

    part1(&mut computer);

//    part2(&nums).unwrap();

    Ok(())
}

fn part1(computer: &mut Computer) -> Result<(), ()> {
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

