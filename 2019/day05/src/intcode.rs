#[derive(Clone)]
pub struct Computer {
    memory: Vec<i32>,
    ip: usize,
    pub input: i32,
    pub output: i32,
}

impl Computer {
    pub fn parse(input: &str) -> Computer {
        let nums: Vec<i32> = input.split(",")
            .map(|n| n.trim())
            .map(|n| n.parse::<i32>())
            .filter(|n| n.is_ok())
            .map(|n| n.unwrap())
            .collect();

        Computer { memory: nums, ip: 0, input: 0, output: 0 }
    }

    pub fn run(&mut self) -> Result<(), &'static str> {
        let mut keep_going = true;
        while keep_going {
            match self.step() {
                Ok(n) => keep_going = n,
                Err(str) => return Err(str)
            };
        }

        Ok(())
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
        let instruction: Box<dyn Instruction> = match opcode {
            1 => Box::new(AddInstruction{opcode: full_opcode}),
            2 => Box::new(MultiplyInstruction{opcode: full_opcode}),
            3 => Box::new(InputInstruction{opcode: full_opcode}),
            4 => Box::new(OutputInstruction{opcode: full_opcode}),
            99 => Box::new(TerminateInstruction{opcode: full_opcode}),
            _ => panic!("Unrecognized opcode: {}", full_opcode),
        };

        instruction.execute(self)
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

    pub fn print(&self) {
        println!("ip: {}", self.ip);
        println!("input: {}", self.input);
        println!("output: {}", self.output);
        print!("Memory: ");
        for i in 0..self.memory.len() {
            print!("{},", self.memory[i]);
        }
        println!();
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
    fn execute(&self, _computer: &mut Computer) -> Result<bool, &'static str> {
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
    fn test_input() {
        let mut computer = Computer::parse("3,0,99");
        computer.input = 1;
        computer.step();
        assert_eq!(computer.memory[0], 1);
    }

    #[test]
    fn test_output() {
        let mut computer = Computer::parse("4,0,99");
        computer.step();
        assert_eq!(computer.output, 4);
    }

    #[test]
    fn test_add_instruction_indirect() {
        let mut computer = Computer::parse("0001,4,3,4,33");
        computer.step();
        assert_eq!(computer.memory[4], 37);
    }

    #[test]
    fn test_add_instruction_immediate() {
        let mut computer = Computer::parse("1101,4,3,4,33");
        computer.step();
        assert_eq!(computer.memory[4], 7);
    }

    #[test]
    fn test_multiply_instruction_indirect() {
        let mut computer = Computer::parse("0002,4,3,4,33");
        computer.step();
        assert_eq!(computer.memory[4], 132);
    }

    #[test]
    fn test_multiply_instruction_immediate() {
        let mut computer = Computer::parse("1102,4,3,4,33");
        computer.step();
        assert_eq!(computer.memory[4], 12);
    }
}
