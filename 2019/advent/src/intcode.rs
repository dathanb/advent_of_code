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
            1 => Box::new(AddInstruction { opcode: full_opcode }),
            2 => Box::new(MultiplyInstruction { opcode: full_opcode }),
            3 => Box::new(InputInstruction {}),
            4 => Box::new(OutputInstruction { opcode: full_opcode }),
            5 => Box::new(JumpIfTrueInstruction { opcode: full_opcode }),
            6 => Box::new(JumpIfFalseInstruction { opcode: full_opcode }),
            7 => Box::new(LessThanInstruction { opcode: full_opcode }),
            8 => Box::new(EqualsInstruction { opcode: full_opcode }),
            99 => Box::new(TerminateInstruction {}),
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

    #[allow(dead_code)]
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

/*********************************
Structs
*********************************/
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

struct TerminateInstruction {}

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
        let operand1 = computer.get_and_advance(AddressingMode::get_by_index((self.opcode / 100) % 10));
        let operand2 = computer.get_and_advance(AddressingMode::get_by_index((self.opcode / 1000) % 10));
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
        let operand1 = computer.get_and_advance(AddressingMode::get_by_index((self.opcode / 100) % 10));
        let operand2 = computer.get_and_advance(AddressingMode::get_by_index((self.opcode / 1000) % 10));
        let result_pointer = computer.get_and_advance(AddressingMode::Immediate) as usize;

        let result = operand1 * operand2;
        computer.memory[result_pointer] = result;

        Ok(true)
    }
}

struct InputInstruction {}

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
        computer.output = computer.get_and_advance(AddressingMode::get_by_index((self.opcode / 100) % 10));

        Ok(true)
    }
}

struct JumpIfTrueInstruction {
    opcode: i32,
}

impl Instruction for JumpIfTrueInstruction {
    fn execute(&self, computer: &mut Computer) -> Result<bool, &'static str> {
        let operand = computer.get_and_advance(AddressingMode::get_by_index((self.opcode / 100) % 10));
        let destination = computer.get_and_advance(AddressingMode::get_by_index((self.opcode / 1000) % 10));

        if operand != 0 {
            computer.ip = destination as usize;
        }

        Ok(true)
    }
}

struct JumpIfFalseInstruction {
    opcode: i32,
}

impl Instruction for JumpIfFalseInstruction {
    fn execute(&self, computer: &mut Computer) -> Result<bool, &'static str> {
        let operand = computer.get_and_advance(AddressingMode::get_by_index((self.opcode / 100) % 10));
        let destination = computer.get_and_advance(AddressingMode::get_by_index((self.opcode / 1000) % 10));

        if operand == 0 {
            computer.ip = destination as usize;
        }

        Ok(true)
    }
}

struct LessThanInstruction {
    opcode: i32,
}

impl Instruction for LessThanInstruction {
    fn execute(&self, computer: &mut Computer) -> Result<bool, &'static str> {
        let mut operand_modes = self.opcode / 100;
        let operand1 = computer.get_and_advance(AddressingMode::get_by_index(operand_modes % 10));
        operand_modes /= 10;
        let operand2 = computer.get_and_advance(AddressingMode::get_by_index(operand_modes % 10));
        let destination = computer.get_and_advance(AddressingMode::Immediate);

        if operand1 < operand2 {
            computer.memory[destination as usize] = 1;
        } else {
            computer.memory[destination as usize] = 0;
        }

        Ok(true)
    }
}

struct EqualsInstruction {
    opcode: i32,
}

impl Instruction for EqualsInstruction {
    fn execute(&self, computer: &mut Computer) -> Result<bool, &'static str> {
        let mut operand_modes = self.opcode / 100;
        let operand1 = computer.get_and_advance(AddressingMode::get_by_index(operand_modes % 10));
        operand_modes /= 10;
        let operand2 = computer.get_and_advance(AddressingMode::get_by_index(operand_modes % 10));
        let destination = computer.get_and_advance(AddressingMode::Immediate);

        if operand1 == operand2 {
            computer.memory[destination as usize] = 1;
        } else {
            computer.memory[destination as usize] = 0;
        }

        Ok(true)
    }
}


/*********************************
Tests
*********************************/
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input() {
        let mut computer = Computer::parse("3,0,99");
        computer.input = 1;
        computer.step().unwrap();
        assert_eq!(computer.memory[0], 1);
    }

    #[test]
    fn test_output() {
        let mut computer = Computer::parse("4,0,99");
        computer.step().unwrap();
        assert_eq!(computer.output, 4);
    }

    #[test]
    fn test_add_instruction_indirect() {
        let mut computer = Computer::parse("0001,4,3,4,33");
        computer.step().unwrap();
        assert_eq!(computer.memory[4], 37);
    }

    #[test]
    fn test_add_instruction_immediate() {
        let mut computer = Computer::parse("1101,4,3,4,33");
        computer.step().unwrap();
        assert_eq!(computer.memory[4], 7);
    }

    #[test]
    fn test_multiply_instruction_indirect() {
        let mut computer = Computer::parse("0002,4,3,4,33");
        computer.step().unwrap();
        assert_eq!(computer.memory[4], 132);
    }

    #[test]
    fn test_multiply_instruction_immediate() {
        let mut computer = Computer::parse("1102,4,3,4,33");
        computer.step().unwrap();
        assert_eq!(computer.memory[4], 12);
    }

    #[test]
    fn test_jump_if_true_instruction_indirect_true() {
        let mut computer = Computer::parse("05,8,9,1,1,1,1,99,1,7");
        computer.step().unwrap();
        assert_eq!(computer.ip, 7);
    }

    #[test]
    fn test_jump_if_true_instruction_indirect_false() {
        let mut computer = Computer::parse("05,8,9,1,1,1,1,99,0,7");
        computer.step().unwrap();
        assert_eq!(computer.ip, 3);
    }

    #[test]
    fn test_jump_if_true_instruction_immediate_true() {
        let mut computer = Computer::parse("1105,1,7,1,1,1,1,99,1,7");
        let result = computer.step();
        assert_eq!(result.is_err(), false);
        assert_eq!(computer.ip, 7);
    }

    #[test]
    fn test_jump_if_true_instruction_immediate_false() {
        let mut computer = Computer::parse("1105,0,7,1,1,1,1,99,1,7");
        let result = computer.step();
        assert_eq!(result.is_err(), false);
        assert_eq!(computer.ip, 3);
    }

    #[test]
    fn test_jump_if_false_instruction_indirect_true() {
        let mut computer = Computer::parse("06,8,9,1,1,1,1,99,1,7");
        let result = computer.step();
        assert_eq!(result.is_err(), false);
        assert_eq!(computer.ip, 3);
    }

    #[test]
    fn test_jump_if_false_instruction_indirect_false() {
        let mut computer = Computer::parse("06,8,9,1,1,1,1,99,0,7");
        let result = computer.step();
        assert_eq!(result.is_err(), false);
        assert_eq!(computer.ip, 7);
    }

    #[test]
    fn test_jump_if_false_instruction_immediate_true() {
        let mut computer = Computer::parse("1106,1,7,1,1,1,1,99,1,7");
        let result = computer.step();
        assert_eq!(result.is_err(), false);
        assert_eq!(computer.ip, 3);
    }

    #[test]
    fn test_jump_if_false_instruction_immediate_false() {
        let mut computer = Computer::parse("1106,0,7,1,1,1,1,99,1,7");
        let result = computer.step();
        assert_eq!(result.is_err(), false);
        assert_eq!(computer.ip, 7);
    }

    #[test]
    fn test_less_than_instruction_immediate_less() {
        let mut computer = Computer::parse("11107,1,2,5,99,0");
        let result = computer.step();
        assert_eq!(result.is_err(), false);
        assert_eq!(computer.memory[5], 1);
    }

    #[test]
    fn test_less_than_instruction_immediate_greater() {
        let mut computer = Computer::parse("11107,3,2,5,99,1");
        let result = computer.step();
        assert_eq!(result.is_err(), false);
        assert_eq!(computer.memory[5], 0);
    }

    #[test]
    fn smoke_test_computer() {
        let mut computer = Computer::parse("3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9");
        computer.input = 0;
        let result = computer.run();
        assert_eq!(false, result.is_err());
        assert_eq!(0, computer.output);

        let mut computer = Computer::parse("3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9");
        computer.input = 1;
        let result = computer.run();
        assert_eq!(false, result.is_err());
        assert_eq!(1, computer.output);

        let mut computer = Computer::parse("3,3,1105,-1,9,1101,0,0,12,4,12,99,1");
        computer.input = 0;
        let result = computer.run();
        assert_eq!(false, result.is_err());
        assert_eq!(0, computer.output);

        let mut computer = Computer::parse("3,3,1105,-1,9,1101,0,0,12,4,12,99,1");
        computer.input = 1;
        let result = computer.run();
        assert_eq!(false, result.is_err());
        assert_eq!(1, computer.output);

//         - Using position mode, consider whether the input is equal to 8; output 1 (if it is) or 0 (if it is not).
        let mut computer = Computer::parse("3,9,8,9,10,9,4,9,99,-1,8");
        computer.input = 1;
        let result = computer.run();
        assert_eq!(false, result.is_err());
        assert_eq!(0, computer.output);

        let mut computer = Computer::parse("3,9,8,9,10,9,4,9,99,-1,8");
        computer.input = 8;
        let result = computer.run();
        assert_eq!(false, result.is_err());
        assert_eq!(1, computer.output);

//                let mut computer =
//        3,9,7,9,10,9,4,9,99,-1,8 - Using position mode, consider whether the input is less than 8; output 1 (if it is) or 0 (if it is not).
//        3,3,1108,-1,8,3,4,3,99 - Using immediate mode, consider whether the input is equal to 8; output 1 (if it is) or 0 (if it is not).
//        3,3,1107,-1,8,3,4,3,99 - Using immediate mode, consider whether the input is less than 8; output 1 (if it is) or 0 (if it is not).


        let mut computer = Computer::parse("3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99");
        computer.input = 1;
        let result = computer.run();
        assert_eq!(false, result.is_err());
        assert_eq!(999, computer.output);

        let mut computer = Computer::parse("3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99");
        computer.input = 8;
        let result = computer.run();
        assert_eq!(false, result.is_err());
        assert_eq!(1000, computer.output);

        let mut computer = Computer::parse("3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99");
        computer.input = 9;
        let result = computer.run();
        assert_eq!(false, result.is_err());
        assert_eq!(1001, computer.output);
    }
}
