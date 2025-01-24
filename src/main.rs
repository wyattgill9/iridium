pub mod vm;
use crate::vm::{VMError, VM};

// Example main function demonstrating usage
fn main() -> Result<(), VMError> {
    let mut vm = VM::new();
    let program = vec![
        1, 0, 0, 42,   // LOAD r0 with 42
        1, 1, 0, 10,   // LOAD r1 with 10
        2, 0, 1, 2,    // ADD r0, r1 into r2
        0,             // HLT
    ];
    
    vm.add_program(program);
    vm.run()?;
    
    // Now using the new method to print registers
    println!("Registers: {:?}", vm.get_registers());
    
    // Or get a specific register value
    println!("Register 2 value: {}", vm.get_register(2)?);
    
    Ok(())
}
