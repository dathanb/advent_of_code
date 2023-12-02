use anyhow::{anyhow, Result};
use crate::shared::read_lines;

pub fn day10_part1(path: &str) -> Result<i32> {
    let lines = read_lines(path);
    let mut vm = VirtualMachine { clock_cycle: 1, register_x: 1, active_instruction: None };
    let mut signal_sum = 0;
    let mut iter = lines.into_iter();
    loop {
        signal_sum += sample_signal(&vm).unwrap_or(0);
        if vm.active_instruction.is_some() {
            vm.step();
        }
        if vm.active_instruction.is_some() {
            continue;
        }

        if let Some(line) = iter.next() {
            let parts = line.split(" ").collect::<Vec<&str>>();
            match &parts[..] {
                ["noop"] => vm.active_instruction = Some(Instruction::Noop{completion_cycle: vm.clock_cycle+1}),
                ["addx", n] => {
                    let operand = n.parse::<i32>()?;
                    vm.active_instruction = Some(Instruction::Add{operand, completion_cycle: vm.clock_cycle+2});
                }
                _ => {
                    return Err(anyhow!("Unrecognized instruction {}", line));
                }
            }
        } else {
            break;
        }
    }

    Ok(signal_sum)
}

pub fn day10_part2(path: &str) -> Result<i32> {
    let lines = read_lines(path);
    todo!()
}

struct VirtualMachine {
    pub clock_cycle: i32,
    pub register_x: i32,
    pub active_instruction: Option<Instruction>
}

impl VirtualMachine {
    pub fn step(&mut self) {
        self.clock_cycle += 1;
        match self.active_instruction {
            Some(Instruction::Add{operand, completion_cycle}) => {
                if self.clock_cycle == completion_cycle {
                    self.register_x += operand;
                    self.active_instruction = None;
                }
            }
            Some(Instruction::Noop{completion_cycle}) => {
                if self.clock_cycle == completion_cycle {
                    self.active_instruction = None;
                }
            }
            None => {}
        };
    }
}

enum Instruction {
    /// A noop that completes on the given clock cycle
    Noop {
        completion_cycle: i32
    },
    /// An add instruction that adds the operand to the X register and completes on the given clock cycle
    Add {
        operand: i32,
        completion_cycle: i32,
    }
}



/// If we should sample, returns Some(sample_value)
/// else returns None
fn sample_signal(vm: &VirtualMachine) -> Option<i32> {
    if (vm.clock_cycle -20) % 40 == 0 {
        Some(vm.register_x * vm.clock_cycle)
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use anyhow::Result;
    use crate::day10::{day10_part1, day10_part2};

    #[test]
    pub fn test_part1() -> Result<()> {
        let score = day10_part1("data/day10_test.txt")?;
        println!("{}", score);
        assert_eq!(13140, score);
        Ok(())
    }

    #[test]
    pub fn run_part1() -> Result<()> {
        let score = day10_part1("data/day10.txt")?;
        println!("{}", score);
        assert_eq!(15140, score);
        Ok(())
    }

    #[test]
    pub fn test_part2() -> Result<()> {
        let score = day10_part2("data/day10_test.txt")?;
        println!("{}", score);
        assert_eq!(0, score);
        Ok(())
    }

    #[test]
    pub fn run_part2() -> Result<()> {
        let score = day10_part2("data/day10.txt")?;
        println!("{}", score);
        assert_eq!(0, score);
        Ok(())
    }
}
