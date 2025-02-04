use crate::{Assembler, VM, parser, codegen};

pub fn pipeline() -> Result<(), Box<dyn std::error::Error>> {

    let mut assembler = Assembler::new();

    let program = parser::parse_file("example.sl").unwrap();

    let asm = codegen::codegen(program);
    
    let asm_code = asm.join("\n").to_string();
    println!("{}", asm_code);
    
    let bytecode = assembler.compile(&asm_code)?;
    // println!("Bytecode: {:#?}", bytecode);

    let mut vm = VM::new();
    vm.add_program(bytecode);

    vm.run()?;

    Ok(())
}
