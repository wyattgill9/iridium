use vm_project::{Assembler, VM, Parser};
use vm_project::ast::{Program, Function, Expr, BinOp};

// Functions that the language should support:
// Conditional branching (e.g., if, while)
// Variables or memory (e.g., registers, arrays)
// The ability to perform iteration or recursion

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut assembler = Assembler::new();

    let asm_code = r#"
    ; Complex program demonstrating multiple operations
    LOAD r0 10    ; Load initial value
    LOAD r1 5     ; Load second value
    LOAD r2 3     ; Load third value
    
    ; Multiply first two values
    MUL r0 r1 r3  ; r3 = r0 * r1
    
    ; Add the result to third value
    ADD r3 r2 r4  ; r4 = r3 + r2
    
    ; Subtract to verify
    SUB r4 r1 r5  ; r5 = r4 - r1
    
    ; Divide to finalize
    DIV r4 r2 r6  ; r6 = r4 / r2
    
    HLT           ; Halt the program
    "#;

    let bytecode = assembler.compile(asm_code)?;

    println!("Bytecode: {:?}", bytecode);

    let mut vm = VM::new();
    vm.add_program(bytecode);
    vm.run()?;

    println!("Registers after execution:");
    for (i, &val) in vm.get_registers().iter().enumerate() {
        if val != 0 {
            println!("r{}: {}", i, val);
        }
    }

    println!("r3 value: {}", vm.get_register(3)?);
    println!("r4 value: {}", vm.get_register(4)?);
    println!("r5 value: {}", vm.get_register(5)?);
    println!("r6 value: {}", vm.get_register(6)?);

    Ok(())
}
