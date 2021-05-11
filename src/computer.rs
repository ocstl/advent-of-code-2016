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
enum ValueOrRegister {
    Value(Value),
    Register(Register),
}

impl<'a> TryFrom<&'a str> for ValueOrRegister {
    type Error = ComputerError;

    fn try_from(value: &'a str) -> ComputerResult<ValueOrRegister> {
        if let Ok(v) = Value::try_from(value) {
            Ok(ValueOrRegister::Value(v))
        } else {
            Register::try_from(value).map(ValueOrRegister::Register)
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Instruction {
    Copy(ValueOrRegister, Register),
    Increment(Register),
    Decrement(Register),
    JumpIfNotZero(ValueOrRegister, Value),
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
                    .and_then(ValueOrRegister::try_from)?;
                let second = iter
                    .next()
                    .ok_or_else(|| ComputerError::MissingArgument(instruction.to_string()))
                    .and_then(Register::try_from)?;
                Ok(Instruction::Copy(first, second))
            }
            Some("inc") => {
                let first = iter
                    .next()
                    .ok_or_else(|| ComputerError::MissingArgument(instruction.to_string()))
                    .and_then(Register::try_from)?;
                Ok(Instruction::Increment(first))
            }
            Some("dec") => {
                let first = iter
                    .next()
                    .ok_or_else(|| ComputerError::MissingArgument(instruction.to_string()))
                    .and_then(Register::try_from)?;
                Ok(Instruction::Decrement(first))
            }
            Some("jnz") => {
                let first = iter
                    .next()
                    .ok_or_else(|| ComputerError::MissingArgument(instruction.to_string()))
                    .and_then(ValueOrRegister::try_from)?;
                let second = iter
                    .next()
                    .ok_or_else(|| ComputerError::MissingArgument(instruction.to_string()))
                    .and_then(Value::try_from)?;
                Ok(Instruction::JumpIfNotZero(first, second))
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
                Instruction::Copy(a, Register(b)) => {
                    match a {
                        ValueOrRegister::Value(v) => self.registers[b] = v,
                        ValueOrRegister::Register(Register(r)) => {
                            self.registers[b] = self.registers[r]
                        }
                    };
                    self.instruction_pointer += 1;
                }
                Instruction::Increment(Register(r)) => {
                    self.registers[r].0 += 1;
                    self.instruction_pointer += 1;
                }
                Instruction::Decrement(Register(r)) => {
                    self.registers[r].0 -= 1;
                    self.instruction_pointer += 1;
                }
                Instruction::JumpIfNotZero(a, Value(v)) => {
                    if match a {
                        ValueOrRegister::Value(Value(b)) => b,
                        ValueOrRegister::Register(Register(r)) => self.registers[r].0,
                    } != 0
                    {
                        self.instruction_pointer += v;
                        if self.instruction_pointer < 0 {
                            return Err(ComputerError::InvalidInstructionPointer(
                                self.instruction_pointer,
                            ));
                        }
                    } else {
                        self.instruction_pointer += 1;
                    }
                }
            }
        }

        Ok(self)
    }

    pub fn registers(&self) -> Vec<i32> {
        self.registers.iter().map(|r| r.0).collect()
    }
}
