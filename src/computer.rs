use std::collections::VecDeque;

#[derive(PartialEq, Eq)]
enum ParamMode {
    Read,
    Write,
}

enum Param {
    Position(usize),
    Value(i64),
}

impl Param {
    fn evaluate(&self, program: &Vec<i64>, mode: ParamMode) -> i64 {
        match self {
            Param::Position(pos) if mode == ParamMode::Read => program[*pos],
            Param::Position(pos) if mode == ParamMode::Write => *pos as i64,
            Param::Value(val) => *val,
            _ => unreachable!(),
        }
    }

    fn evaluate_r(&self, program: &Vec<i64>) -> i64 {
        self.evaluate(program, ParamMode::Read)
    }

    fn evaluate_w(&self, program: &Vec<i64>) -> i64 {
        self.evaluate(program, ParamMode::Write)
    }

    fn from(value: i64, param_type: i64) -> Self {
        match param_type {
            0 => Param::Position(value as usize),
            1 => Param::Value(value),
            _ => unreachable!(),
        }
    }
}

enum Instruction {
    Add(Param, Param, Param),
    Mul(Param, Param, Param),
    Inp(Param),
    Out(Param),
    JumpTrue(Param, Param),
    JumpFalse(Param, Param),
    LessThan(Param, Param, Param),
    Equals(Param, Param, Param),
    Halt,
}

impl Instruction {
    fn instruction_size(&self) -> usize {
        match self {
            Instruction::Add(_, _, _) => 4,
            Instruction::Mul(_, _, _) => 4,
            Instruction::Inp(_) => 2,
            Instruction::Out(_) => 2,
            Instruction::LessThan(_, _, _) => 4,
            Instruction::Equals(_, _, _) => 4,
            Instruction::JumpTrue(_, _) => 3,
            Instruction::JumpFalse(_, _) => 3,
            Instruction::Halt => 0,
        }
    }
}

pub struct Interpreter {
    input: VecDeque<i64>,
    pub output: VecDeque<i64>,
    program: Vec<i64>,
    pointer: usize,
    halt: bool,
    halt_on_output: bool,
}

impl Interpreter {
    pub fn new(program: Vec<i64>) -> Self {
        Self {
            input: VecDeque::new(),
            output: VecDeque::new(),
            program: program,
            pointer: 0,
            halt: false,
            halt_on_output: false,
        }
    }

    pub fn execute_program(&mut self) {
        while !self.halt {
            self.execute_step();
        }

        self.halt = false;
    }

    pub fn add_input(&mut self, input: i64) {
        self.input.push_back(input);
    }

    pub fn get_output(&mut self) -> Vec<i64> {
        self.output.iter().cloned().collect()
    }

    pub fn halt_on_output(mut self) -> Self {
        self.halt_on_output = true;
        self
    }

    fn execute_step(&mut self) {
        if !self.halt {
            let current_instruction = self.parse_current_instruction();
            self.pointer = self.execute_instruction(&current_instruction);
        }
    }

    fn execute_instruction(&mut self, instruction: &Instruction) -> usize {
        let program = &mut self.program;
        let mut pointer = self.pointer;
        match instruction {
            Instruction::Add(p1, p2, p3) => {
                let dst_addr = p3.evaluate_w(program) as usize;
                if dst_addr >= program.len() {
                    println!("Error at {:?} {:?}", pointer, program);
                }
                program[dst_addr] = p1.evaluate_r(program) + p2.evaluate_r(program);
                pointer += instruction.instruction_size();
            }
            Instruction::Mul(p1, p2, p3) => {
                let dst_addr = p3.evaluate_w(program) as usize;
                if dst_addr >= program.len() {
                    println!("Error at {:?} {:?}", pointer, program);
                }
                program[dst_addr] = p1.evaluate_r(program) * p2.evaluate_r(program);
                pointer += instruction.instruction_size();
            }
            Instruction::Inp(p1) => {
                let input = self
                    .input
                    .pop_front()
                    .expect(&format!("Expected input at {}", self.pointer));
                let dst_addr = p1.evaluate_w(program) as usize;
                if dst_addr >= program.len() {
                    println!("Error at {:?} {:?}", pointer, program);
                }
                program[dst_addr] = input;
                pointer += instruction.instruction_size();
            }
            Instruction::Out(p1) => {
                let out_val = p1.evaluate_r(program);
                self.output.push_back(out_val);
                pointer += instruction.instruction_size();
                if self.halt_on_output {
                    self.halt = true;
                }
            }
            Instruction::JumpTrue(p1, p2) => {
                if p1.evaluate_r(program) != 0 {
                    let dst = p2.evaluate_r(program) as usize;
                    pointer = dst;
                    if dst >= program.len() {
                        println!("Error at {:?} {:?}", pointer, program);
                    }
                } else {
                    pointer += instruction.instruction_size();
                }
            }
            Instruction::JumpFalse(p1, p2) => {
                if p1.evaluate_r(program) == 0 {
                    let dst = p2.evaluate_r(program) as usize;
                    pointer = dst;
                    if dst >= program.len() {
                        println!("Error at {:?} {:?}", pointer, program);
                    }
                } else {
                    pointer += instruction.instruction_size();
                }
            }
            Instruction::LessThan(p1, p2, p3) => {
                let dst_addr = p3.evaluate_w(program) as usize;
                if dst_addr >= program.len() {
                    println!("Error at {:?} {:?}", pointer, program);
                }
                if p1.evaluate_r(program) < p2.evaluate_r(program) {
                    program[dst_addr] = 1;
                } else {
                    program[dst_addr] = 0;
                }
                pointer += instruction.instruction_size();
            }
            Instruction::Equals(p1, p2, p3) => {
                let dst_addr = p3.evaluate_w(program) as usize;
                if dst_addr >= program.len() {
                    println!("Error at {:?} {:?}", pointer, program);
                }
                if p1.evaluate_r(program) == p2.evaluate_r(program) {
                    program[dst_addr] = 1;
                } else {
                    program[dst_addr] = 0;
                }
                pointer += instruction.instruction_size();
            }
            Instruction::Halt => {
                self.halt = true;
                pointer += instruction.instruction_size();
            }
        }
        pointer
    }

    fn parse_current_instruction(&self) -> Instruction {
        let raw_opcode = self.program[self.pointer];

        let (opcode, t1, t2, t3) = Self::parse_opcode(raw_opcode);
        match opcode {
            1 => Instruction::Add(
                Param::from(self.program[self.pointer + 1], t1),
                Param::from(self.program[self.pointer + 2], t2),
                Param::from(self.program[self.pointer + 3], t2),
            ),
            2 => Instruction::Mul(
                Param::from(self.program[self.pointer + 1], t1),
                Param::from(self.program[self.pointer + 2], t2),
                Param::from(self.program[self.pointer + 3], t2),
            ),
            3 => Instruction::Inp(Param::from(self.program[self.pointer + 1], t1)),
            4 => Instruction::Out(Param::from(self.program[self.pointer + 1], t1)),
            5 => Instruction::JumpTrue(
                Param::from(self.program[self.pointer + 1], t1),
                Param::from(self.program[self.pointer + 2], t2),
            ),
            6 => Instruction::JumpFalse(
                Param::from(self.program[self.pointer + 1], t1),
                Param::from(self.program[self.pointer + 2], t2),
            ),
            7 => Instruction::LessThan(
                Param::from(self.program[self.pointer + 1], t1),
                Param::from(self.program[self.pointer + 2], t2),
                Param::from(self.program[self.pointer + 3], t3),
            ),
            8 => Instruction::Equals(
                Param::from(self.program[self.pointer + 1], t1),
                Param::from(self.program[self.pointer + 2], t2),
                Param::from(self.program[self.pointer + 3], t3),
            ),
            99 => Instruction::Halt,
            _ => unreachable!(),
        }
    }

    fn parse_opcode(raw_code: i64) -> (i64, i64, i64, i64) {
        let opcode = raw_code % 100;
        let params = raw_code / 100;

        let p1 = params % 10;
        let p2 = (params / 10) % 10;
        let p3 = params / 100;

        (opcode, p1, p2, p3)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn parse_opcode() {
        assert_eq!((2, 0, 1, 0), Interpreter::parse_opcode(1002));
        assert_eq!((1, 1, 1, 0), Interpreter::parse_opcode(1101));
        assert_eq!((2, 0, 0, 0), Interpreter::parse_opcode(0002));
        assert_eq!((2, 0, 1, 1), Interpreter::parse_opcode(11002));
    }

    #[test]
    fn interpreter() {
        // This program checks if input is equal to 8
        let mut interpreter = Interpreter::new(vec![3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8]);
        interpreter.add_input(1);
        interpreter.execute_program();
        assert_eq!(
            0,
            *interpreter.get_output().last().expect("expected output")
        );
    }
}
