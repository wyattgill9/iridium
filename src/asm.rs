use std::collections::HashMap;
use std::fmt;

#[derive(Debug)]
pub enum AssemblerError {
    SyntaxError(String),
    UnknownInstruction(String),
    UnknownRegister(String),
    LabelNotFound(String),
}

pub struct Assembler {
    symbols: HashMap<String, usize>,
}

impl Assembler {
    pub fn new() -> Self {
        Self {
            symbols: HashMap::new(),
        }
    }

    pub fn compile(&mut self, source: &str) -> Result<Vec<u8>, AssemblerError> {
        // collect symbols in first pass
        let mut first_pass_lines = Vec::new();
        let mut current_address = 0;

        for line in source.lines() {
            // remove comments
            let line = line.split(';').next().unwrap_or("").trim();

            if line.is_empty() {
                continue;
            }

            if line.ends_with(':') {
                // symbol
                let label = line[..line.len() - 1].trim();
                self.symbols.insert(label.to_string(), current_address);
                continue;
            }

            first_pass_lines.push(line);
            current_address += self.estimate_instruction_size(line)?;
        }

        // Compile instructions
        let mut bytecode = Vec::new();

        for line in first_pass_lines {
            let tokens: Vec<&str> = line.split_whitespace().collect();

            match tokens[0].to_uppercase().as_str() {
                "LOAD" => {
                    if tokens.len() < 3 {
                        return Err(AssemblerError::SyntaxError(format!(
                            "Invalid LOAD instruction: {}",
                            line
                        )));
                    }
                    let register = self.parse_register(&tokens[1])?;
                    let value = self.parse_value(&tokens[2])?;

                    bytecode.push(1); // LOAD 
                    bytecode.push(register);
                    bytecode.extend_from_slice(&value.to_be_bytes());
                }
                "ADD" | "SUB" | "MUL" | "DIV" => {
                    if tokens.len() < 4 {
                        return Err(AssemblerError::SyntaxError(format!(
                            "Invalid arithmetic instruction: {}",
                            line
                        )));
                    }
                    let opcode = match tokens[0].to_uppercase().as_str() {
                        "ADD" => 2,
                        "SUB" => 3,
                        "MUL" => 4,
                        "DIV" => 5,
                        _ => unreachable!(),
                    };

                    let reg1 = self.parse_register(&tokens[1])?;
                    let reg2 = self.parse_register(&tokens[2])?;
                    let reg3 = self.parse_register(&tokens[3])?;

                    bytecode.push(opcode);
                    bytecode.push(reg1);
                    bytecode.push(reg2);
                    bytecode.push(reg3);
                }
                "HLT" => {
                    bytecode.push(0); // HLT
                }
                _ => {
                    return Err(AssemblerError::UnknownInstruction(tokens[0].to_string()));
                }
            }
        }

        // Pad bytecode to 32 bytes
        while bytecode.len() < 32 {
            bytecode.push(0);
        }

        Ok(bytecode)
    }

    fn parse_register(&self, token: &str) -> Result<u8, AssemblerError> {
        if !token.starts_with('r') {
            return Err(AssemblerError::UnknownRegister(token.to_string()));
        }

        let register_num: usize = token[1..]
            .parse()
            .map_err(|_| AssemblerError::UnknownRegister(token.to_string()))?;

        if register_num >= 32 {
            return Err(AssemblerError::UnknownRegister(token.to_string()));
        }

        Ok(register_num as u8)
    }

    fn parse_value(&self, token: &str) -> Result<u16, AssemblerError> {
        // Check if the token is a label
        if let Some(label_value) = self.symbols.get(token) {
            return Ok(*label_value as u16);
        }

        token
            .parse()
            .map_err(|_| AssemblerError::SyntaxError(format!("Invalid value: {}", token)))
    }

    fn estimate_instruction_size(&self, line: &str) -> Result<usize, AssemblerError> {
        let tokens: Vec<&str> = line.split_whitespace().collect();

        match tokens[0].to_uppercase().as_str() {
            "LOAD" => Ok(4), // opcode (1) + register (1) + 16-bit value (2)
            "ADD" | "SUB" | "MUL" | "DIV" => Ok(4), // opcode (1) + 3 registers (3)
            "HLT" => Ok(1),  // single byte opcode
            _ => Err(AssemblerError::UnknownInstruction(tokens[0].to_string())),
        }
    }
}

impl fmt::Display for AssemblerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AssemblerError::SyntaxError(msg) => write!(f, "Syntax Error: {}", msg),
            AssemblerError::UnknownInstruction(inst) => write!(f, "Unknown Instruction: {}", inst),
            AssemblerError::UnknownRegister(reg) => write!(f, "Unknown Register: {}", reg),
            AssemblerError::LabelNotFound(label) => write!(f, "Label Not Found: {}", label),
        }
    }
}

impl std::error::Error for AssemblerError {}
