use std::io;
use std::fs;

use crate::intcode::{Computer};
use crate::permute;

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
    computer.print();

    let num_computers = 5;
    let initial_input = 0;

    let possible_phase_settings = vec![0,1,2,3,4];

    for settings in permute::permute(possible_phase_settings) {
        let mut computers:Vec<Computer> = Vec::with_capacity(num_computers);
        for _ in 0..num_computers {
            computers.push(computer.clone());
        }

        let mut input = initial_input;

        for i in 0..num_computers {
            computers[i].enqueue_input(settings[i]);
            computers[i].enqueue_input(input);
            computers[i].run_no_suspend()?;
            input = computers[i].output;
        }

        // at this point input has the output from the final computer execution
        if input > max {
            max = input;
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

//    #[test]
//    fn test_part2_input1() {
//        let input = "3,26,1001,26,-4,26,3,27,1002,27,2,27,1,27,26,27,4,27,1001,28,-1,28,1005,28,6,99,0,0,5";
//        let expected_output = "139629729";
//        let computer = Computer::parse(input);
//        let actual_output = compute_part2(&computer).unwrap();
//        assert_eq!(actual_output, expected_output);
//    }
//
//    #[test]
//    fn test_part2_input2() {
//        let input = "3,52,1001,52,-5,52,3,53,1,52,56,54,1007,54,5,55,1005,55,26,1001,54,-5,54,1105,1,12,1,53,54,53,1008,54,0,55,1001,55,1,55,2,53,55,53,4,53,1001,56,-1,56,1005,56,6,99,0,0,0,0,10";
//        let expected_output = "18216";
//        let computer = Computer::parse(input);
//        let actual_output = compute_part2(&computer).unwrap();
//        assert_eq!(actual_output, expected_output);
//    }
//
//    #[test]
//    fn test_part2() {
//        let expected_output = String::from("foo");
//        let actual_output = part2().unwrap();
//        assert_eq!(actual_output, expected_output);
//    }
}