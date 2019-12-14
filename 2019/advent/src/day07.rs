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

#[allow(dead_code)]
fn part2() -> Result<String, String> {
    /*
    For part 2, we need to update how we think about the computers.
    Instead of us providing static input, running the computer to completion, and collecting the output,
    we now need the ability to treat the computers like coroutines, where we provide some input and run the computer
    only until it produces output, and we can resume the computer at some later point.

    So we'll need to update the `run` method to return Result<bool, String>, just like `step` does, where the `bool`
    indicates whether the computer is capable of still running.

    We should replace the `bool` with an enum, though, because it's not immediately clear whether true means
    "can still run" or "has terminated".

    So let's do that first -- replace the bool with an enum.
    */
    unimplemented!()
}

fn compute_part1(computer: &Computer) -> Result<String, String> {
    // let's do this the super naive way:
    let mut max = -1;

    for a_input in 0..=4 {
        let mut a_computer = computer.clone();
        a_computer.enqueue_input(a_input);
        a_computer.enqueue_input(0);
        a_computer.run()?;
        let a_output = a_computer.output;

        for b_input in 0..=4 {
            if b_input == a_input {
                continue;
            }
            let mut b_computer = computer.clone();
            b_computer.enqueue_input(b_input);
            b_computer.enqueue_input(a_output);
            b_computer.run()?;
            let b_output = b_computer.output;

            for c_input in 0..=4 {
                if c_input == a_input || c_input == b_input {
                    continue;
                }
                let mut c_computer = computer.clone();
                c_computer.enqueue_input(c_input);
                c_computer.enqueue_input(b_output);
                c_computer.run()?;
                let c_output = c_computer.output;

                for d_input in 0..=4 {
                    if d_input == a_input || d_input == b_input || d_input == c_input {
                        continue;
                    }
                    let mut d_computer = computer.clone();
                    d_computer.enqueue_input(d_input);
                    d_computer.enqueue_input(c_output);
                    d_computer.run()?;
                    let d_output = d_computer.output;

                    for e_input in 0..=4 {
                        if e_input == a_input || e_input == b_input || e_input == c_input || e_input == d_input {
                            continue;
                        }
                        let mut e_computer = computer.clone();
                        e_computer.enqueue_input(e_input);
                        e_computer.enqueue_input(d_output);
                        e_computer.run()?;
                        let e_output = e_computer.output;

                        if e_output > max {
                            max = e_output;
                        }

                    }
                }
            }
        }
    }
    Ok(format!("{}", max))
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

    #[test]
    fn test_part1_real_input() {
        let actual_output = part1().unwrap();
        assert_eq!(actual_output, "65464");
    }
}