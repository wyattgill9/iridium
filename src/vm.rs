use std::fmt;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Opcode {
    HLT,
    IGL,
    LOAD,
    ADD,
    SUB,
    MUL,
    DIV,
    JMP,
    JMPF,
}

#[derive(Debug, PartialEq)]
pub enum VMError {
    ProgramCounterOutOfBounds,
    DivisionByZero,
    RegisterOutOfBounds,
    InvalidOpcode,
}

#[derive(Debug)]
pub struct VM {
    registers: [i32; 32],
    pc: usize,
    program: Vec<u8>,
    remainder: u32,
}

impl VM {
    pub fn new() -> Self {
        Self {
            registers: [0; 32],
            program: Vec::new(),
            pc: 0,
            remainder: 0,
        }
    }

    // Get register value
    pub fn get_register(&self, index: usize) -> Result<i32, VMError> {
        if index >= self.registers.len() {
            Err(VMError::RegisterOutOfBounds)
        } else {
            Ok(self.registers[index])
        }
    }

    // Get all registers
    pub fn get_registers(&self) -> &[i32; 32] {
        &self.registers
    }

    pub fn add_program(&mut self, program: Vec<u8>) {
        self.program = program;
        self.reset();
    }

    pub fn reset(&mut self) {
        self.pc = 0;
        self.registers = [0; 32];
        self.remainder = 0;
    }

    pub fn run(&mut self) -> Result<(), VMError> {
        while self.pc < self.program.len() {
            if !self.execute_instruction()? {
                break;
            }
        }
        Ok(())
    }

    pub fn run_once(&mut self) -> Result<bool, VMError> {
        self.execute_instruction()
    }

    fn execute_instruction(&mut self) -> Result<bool, VMError> {
        if self.pc >= self.program.len() {
            return Err(VMError::ProgramCounterOutOfBounds);
        }

        match self.decode_opcode() {
            Opcode::HLT => {
                return Ok(false);
            }
            Opcode::LOAD => {
                let register = self.next_8_bits()? as usize;
                let number = self.next_16_bits()? as u32;

                if register >= self.registers.len() {
                    return Err(VMError::RegisterOutOfBounds);
                }

                self.registers[register] = number as i32;
            }
            Opcode::ADD => {
                let (reg1, reg2, reg3) = self.get_three_registers()?;
                self.registers[reg3] = self.registers[reg1]
                    .checked_add(self.registers[reg2])
                    .unwrap_or(0); // handle potential overflow
            }
            Opcode::SUB => {
                let (reg1, reg2, reg3) = self.get_three_registers()?;
                self.registers[reg3] = self.registers[reg1]
                    .checked_sub(self.registers[reg2])
                    .unwrap_or(0); // handle potential underflow
            }
            Opcode::MUL => {
                let (reg1, reg2, reg3) = self.get_three_registers()?;
                self.registers[reg3] = self.registers[reg1]
                    .checked_mul(self.registers[reg2])
                    .unwrap_or(0); // handle potential overflow
            }
            Opcode::DIV => {
                let (reg1, reg2, reg3) = self.get_three_registers()?;

                if self.registers[reg2] == 0 {
                    return Err(VMError::DivisionByZero);
                }

                self.registers[reg3] = self.registers[reg1] / self.registers[reg2];
                self.remainder = (self.registers[reg1] % self.registers[reg2]) as u32;
            }
            Opcode::IGL => {
                return Err(VMError::InvalidOpcode);
            }
            Opcode::JMP => {
                let target = self.registers[self.next_8_bits().unwrap_or(0) as usize];
                self.pc = target as usize;
            }            
            Opcode::JMPF => {
                let value = self.registers[self.next_8_bits().unwrap_or(0) as usize];
                self.pc += value as usize;
            }            
        }

        Ok(true)
    }

    fn get_three_registers(&mut self) -> Result<(usize, usize, usize), VMError> {
        let reg1 = self.next_8_bits()? as usize;
        let reg2 = self.next_8_bits()? as usize;
        let reg3 = self.next_8_bits()? as usize;

        if reg1 >= self.registers.len()
            || reg2 >= self.registers.len()
            || reg3 >= self.registers.len()
        {
            return Err(VMError::RegisterOutOfBounds);
        }

        Ok((reg1, reg2, reg3))
    }

    fn decode_opcode(&mut self) -> Opcode {
        let opcode = Opcode::from(self.program[self.pc]);
        self.pc += 1;
        opcode
    }

    fn next_8_bits(&mut self) -> Result<u8, VMError> {
        if self.pc >= self.program.len() {
            return Err(VMError::ProgramCounterOutOfBounds);
        }
        let result = self.program[self.pc];
        self.pc += 1;
        Ok(result)
    }

    fn next_16_bits(&mut self) -> Result<u16, VMError> {
        if self.pc + 1 >= self.program.len() {
            return Err(VMError::ProgramCounterOutOfBounds);
        }
        let result = ((self.program[self.pc] as u16) << 8) | self.program[self.pc + 1] as u16;
        self.pc += 2;
        Ok(result)
    }
}

impl From<u8> for Opcode {
    fn from(v: u8) -> Self {
        match v {
            0 => Opcode::HLT,
            1 => Opcode::LOAD,
            2 => Opcode::ADD,
            3 => Opcode::SUB,
            4 => Opcode::MUL,
            5 => Opcode::DIV,
            6 => Opcode::JMP,
            7 => Opcode::JMPF,
            _ => Opcode::IGL,
        }
    }
}

impl fmt::Display for VMError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            VMError::ProgramCounterOutOfBounds => write!(f, "Program counter out of bounds"),
            VMError::DivisionByZero => write!(f, "Division by zero"),
            VMError::RegisterOutOfBounds => write!(f, "Register index out of bounds"),
            VMError::InvalidOpcode => write!(f, "Encountered invalid opcode"),
        }
    }
}

impl std::error::Error for VMError {}
