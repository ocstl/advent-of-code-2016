use std::convert::TryFrom;

#[derive(Debug, Clone)]
pub enum ComputerError {
    InvalidOpCode(String),
    InvalidValue(String),
    InvalidRegister(String),
    MissingArgument(String),
    InvalidInstructionPointer(i32),
}

impl std::fmt::Display for ComputerError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        write!(f, "{:?}", self)
    }
}

impl std::error::Error for ComputerError {}

type ComputerResult<T> = std::result::Result<T, ComputerError>;

#[derive(Debug, Clone, Copy)]
struct Value(i32);

impl TryFrom<&str> for Value {
    type Error = ComputerError;

    fn try_from(value: &str) -> ComputerResult<Value> {
        match value.parse::<i32>() {
            Ok(v) => Ok(Value(v)),
            Err(_) => Err(ComputerError::InvalidValue(value.to_string())),
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Register(usize);

impl TryFrom<&str> for Register {
    type Error = ComputerError;

    fn try_from(register: &str) -> ComputerResult<Register> {
        match register {
            "a" => Ok(Register(0)),
            "b" => Ok(Register(1)),
            "c" => Ok(Register(2)),
            "d" => Ok(Register(3)),
            _ => Err(ComputerError::InvalidRegister(register.to_string())),
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Argument {
    Value(Value),
    Register(Register),
}

impl<'a> TryFrom<&'a str> for Argument {
    type Error = ComputerError;

    fn try_from(value: &'a str) -> ComputerResult<Argument> {
        if let Ok(v) = Value::try_from(value) {
            Ok(Argument::Value(v))
        } else {
            Register::try_from(value).map(Argument::Register)
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Instruction {
    Copy(Argument, Argument),
    Increment(Argument),
    Decrement(Argument),
    JumpIfNotZero(Argument, Argument),
    Toggle(Argument),
}

impl Instruction {
    fn toggle(self) -> Self {
        match self {
            Instruction::Copy(a, b) => Instruction::JumpIfNotZero(a, b),
            Instruction::Increment(a) => Instruction::Decrement(a),
            Instruction::Decrement(a) => Instruction::Increment(a),
            Instruction::JumpIfNotZero(a, b) => Instruction::Copy(a, b),
            Instruction::Toggle(a) => Instruction::Increment(a),
        }
    }
}

impl<'a> TryFrom<&'a str> for Instruction {
    type Error = ComputerError;

    fn try_from(instruction: &'a str) -> ComputerResult<Instruction> {
        let mut iter = instruction.split_whitespace();
        match iter.next() {
            Some("cpy") => {
                let first = iter
                    .next()
                    .ok_or_else(|| ComputerError::MissingArgument(instruction.to_string()))
                    .and_then(Argument::try_from)?;
                let second = iter
                    .next()
                    .ok_or_else(|| ComputerError::MissingArgument(instruction.to_string()))
                    .and_then(Argument::try_from)?;
                Ok(Instruction::Copy(first, second))
            }
            Some("inc") => {
                let first = iter
                    .next()
                    .ok_or_else(|| ComputerError::MissingArgument(instruction.to_string()))
                    .and_then(Argument::try_from)?;
                Ok(Instruction::Increment(first))
            }
            Some("dec") => {
                let first = iter
                    .next()
                    .ok_or_else(|| ComputerError::MissingArgument(instruction.to_string()))
                    .and_then(Argument::try_from)?;
                Ok(Instruction::Decrement(first))
            }
            Some("jnz") => {
                let first = iter
                    .next()
                    .ok_or_else(|| ComputerError::MissingArgument(instruction.to_string()))
                    .and_then(Argument::try_from)?;
                let second = iter
                    .next()
                    .ok_or_else(|| ComputerError::MissingArgument(instruction.to_string()))
                    .and_then(Argument::try_from)?;
                Ok(Instruction::JumpIfNotZero(first, second))
            }
            Some("tgl") => {
                let first = iter
                    .next()
                    .ok_or_else(|| ComputerError::MissingArgument(instruction.to_string()))
                    .and_then(Argument::try_from)?;
                Ok(Instruction::Toggle(first))
            }
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Computer {
    registers: [Value; 4],
    instruction_pointer: i32,
    program: Vec<Instruction>,
}

impl Computer {
    pub fn new(program: &str) -> ComputerResult<Self> {
        let program: Vec<Instruction> = program
            .lines()
            .map(Instruction::try_from)
            .collect::<ComputerResult<Vec<Instruction>>>()?;

        Ok(Computer {
            registers: [Value(0); 4],
            instruction_pointer: 0,
            program,
        })
    }

    pub fn reset(&mut self) -> &mut Self {
        self.registers = [Value(0); 4];
        self.instruction_pointer = 0;
        self
    }

    pub fn set_registers(&mut self, registers: [i32; 4]) -> &mut Self {
        for (a, b) in registers.iter().zip(self.registers.iter_mut()) {
            *b = Value(*a);
        }
        self
    }

    pub fn run(&mut self) -> ComputerResult<&mut Self> {
        while let Some(&i) = self.program.get(self.instruction_pointer as usize) {
            match i {
                Instruction::Copy(a, b) => self.copy_instruction(a, b),
                Instruction::Increment(a) => self.increment_instruction(a),
                Instruction::Decrement(a) => self.decrement_instruction(a),
                Instruction::JumpIfNotZero(a, b) => self.jump_if_not_zero_instruction(a, b)?,
                Instruction::Toggle(a) => self.toggle_instruction(a),
            }
        }

        Ok(self)
    }

    fn copy_instruction(&mut self, a: Argument, b: Argument) {
        let a = match a {
            Argument::Register(Register(r)) => self.registers[r],
            Argument::Value(v) => v,
        };

        if let Argument::Register(Register(r)) = b {
            self.registers[r] = a;
        }

        self.instruction_pointer += 1;
    }

    fn increment_instruction(&mut self, a: Argument) {
        if let Argument::Register(Register(r)) = a {
            self.registers[r].0 += 1;
        }

        self.instruction_pointer += 1;
    }

    fn decrement_instruction(&mut self, a: Argument) {
        if let Argument::Register(Register(r)) = a {
            self.registers[r].0 -= 1;
        }

        self.instruction_pointer += 1;
    }

    fn jump_if_not_zero_instruction(
        &mut self,
        a: Argument,
        b: Argument,
    ) -> Result<(), ComputerError> {
        let a = match a {
            Argument::Value(Value(v)) => v,
            Argument::Register(Register(r)) => self.registers[r].0,
        };

        let b = match b {
            Argument::Value(Value(v)) => v,
            Argument::Register(Register(r)) => self.registers[r].0,
        };

        if a != 0 {
            self.instruction_pointer += b;
            if self.instruction_pointer < 0 {
                return Err(ComputerError::InvalidInstructionPointer(
                    self.instruction_pointer,
                ));
            }
        } else {
            self.instruction_pointer += 1;
        }

        Ok(())
    }

    fn toggle_instruction(&mut self, a: Argument) {
        let a = match a {
            Argument::Value(Value(b)) => b,
            Argument::Register(Register(r)) => self.registers[r].0,
        };

        let target = a + (self.instruction_pointer as i32);
        if target >= 0 {
            if let Some(i) = self.program.get_mut(target as usize) {
                *i = i.toggle();
            }
        }

        self.instruction_pointer += 1;
    }

    pub fn registers(&self) -> Vec<i32> {
        self.registers.iter().map(|r| r.0).collect()
    }
}
