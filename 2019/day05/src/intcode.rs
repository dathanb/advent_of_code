#[derive(Clone)]
struct Computer {
    memory: Vec<i32>,
    ip: usize,
    input: i32,
    output: i32,
}

impl Computer {
    fn parse(input: &str) -> Computer {
        let nums: Vec<i32> = input.split(",")
            .map(|n| n.trim())
            .map(|n| n.parse::<i32>())
            .filter(|n| n.is_ok())
            .map(|n| n.unwrap())
            .collect();

        Computer { memory: nums, ip: 0, input: 0, output: 0 }
    }

    /**
    Process a single instruction.

    Returns Ok(bool) if the program should continue. The boolean value indicates whether the program should continue to
    run: true if program execution can continue, false if the program has run to completion.
    Returns Err with a string value on error
    */
    fn step(&mut self) -> Result<bool, &'static str> {
        let full_opcode = self.get_and_advance(AddressingMode::Immediate);
        let opcode = full_opcode % 100;
        if opcode == 1 {
            return AddInstruction{opcode: full_opcode}.execute(self);
        }
        Err("Didn't match an opcode")
    }

    /**
    Gets the value at the current instruction pointer.
    If mode is AddressingMode::Immediate, return that value.
    If mode is AddressingMode::Indirect, return the value at the index given by that value.
    In either case, postincrement the instruction pointer by 1.
    */
    fn get_and_advance(&mut self, mode: AddressingMode) -> i32 {
        let n = self.memory[self.ip];
        self.ip += 1;

        match mode {
            AddressingMode::Immediate => n,
            AddressingMode::Indirect => self.memory[n as usize],
        }
    }
}

/*
I think the Computer should expose a `run` method that runs to completion.
It also needs an input -- for now it's a constant, so we'll just make it an i32
It also needs an output -- for now let's just make it a single i32 value.

run() just calls step() repeatedly.
step() resolves an opcode from the value under the current instruction pointer, and calls opcode.execute(Computer)

So how do we resolve that opcode?

Just pattern match on the lower two digits of the opcode, and instantiate the correct opcode type.

We may need to define an Opcode trait for each of these so we can just call execute()?
*/

enum AddressingMode {
    Immediate,
    Indirect,
}


impl AddressingMode {
    fn get_by_index(index: i32) -> AddressingMode {
        match index {
            0 => AddressingMode::Indirect,
            1 => AddressingMode::Immediate,
            _ => panic!("Unrecognized index: {}", index),
        }
    }
}

trait Instruction {
    fn execute(&self, computer: &mut Computer) -> Result<bool, &'static str>;
}

struct TerminateInstruction {
    opcode: i32,
}

impl Instruction for TerminateInstruction {
    fn execute(&self, computer: &mut Computer) -> Result<bool, &'static str> {
        Ok(false)
    }
}

struct AddInstruction {
    opcode: i32,
}

impl Instruction for AddInstruction {
    fn execute(&self, computer: &mut Computer) -> Result<bool, &'static str> {
        let operand1 = computer.get_and_advance(AddressingMode::get_by_index((self.opcode/100)%10));
        let operand2 = computer.get_and_advance(AddressingMode::get_by_index((self.opcode/1000)%10));
        let result_pointer = computer.get_and_advance(AddressingMode::Immediate) as usize;

        let result = operand1 + operand2;
        computer.memory[result_pointer] = result;

        Ok(true)
    }
}

struct MultiplyInstruction {
    opcode: i32,
}

impl Instruction for MultiplyInstruction {
    fn execute(&self, computer: &mut Computer) -> Result<bool, &'static str> {
        let operand1 = computer.get_and_advance(AddressingMode::get_by_index((self.opcode/100)%10));
        let operand2 = computer.get_and_advance(AddressingMode::get_by_index((self.opcode/1000)%10));
        let result_pointer = computer.get_and_advance(AddressingMode::Immediate) as usize;

        let result = operand1 * operand2;
        computer.memory[result_pointer] = result;

        Ok(true)
    }
}

struct InputInstruction {
    opcode: i32,
}

impl Instruction for InputInstruction {
    fn execute(&self, computer: &mut Computer) -> Result<bool, &'static str> {
        let operand1 = computer.get_and_advance(AddressingMode::Immediate);
        computer.memory[operand1 as usize] = computer.input;

        Ok(true)
    }
}

struct OutputInstruction {
    opcode: i32,
}

impl Instruction for OutputInstruction {
    fn execute(&self, computer: &mut Computer) -> Result<bool, &'static str> {
        computer.output = computer.get_and_advance(AddressingMode::Indirect);

        Ok(true)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_opcode_immediate() {
        let mut computer = Computer::parse("0001,4,3,4,33");
        computer.step();
        assert_eq!(computer.memory[4], 36);
    }

}
