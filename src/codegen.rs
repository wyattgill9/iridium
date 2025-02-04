use crate::parser::{Program, Statement, Expr, BinOp};

pub fn codegen(program: Program) -> Vec<String> {
    let mut asm = Vec::new();
    let mut reg_counter = 0;

    for statement in program.statements {
        match statement {
            Statement::Declare(var, expr) => {
                let reg = generate_expr(&expr, &mut asm, &mut reg_counter);
                asm.push(format!("; {} is in r{}", var, reg));
            }
            Statement::Print(expr) => {
                let result_reg = generate_expr(&expr, &mut asm, &mut reg_counter);
                asm.push(format!("PRINT r{}", result_reg));
            }
        }
    }

    asm.push("HLT".to_string());
    asm
}

fn generate_expr(expr: &Expr, asm: &mut Vec<String>, reg_counter: &mut usize) -> usize {
    match expr {
        Expr::Variable(var) => {
            let reg = allocate_register(reg_counter);
            asm.push(format!("; assuming {} is in r{}", var, reg));
            reg
        }
        Expr::Literal(value) => {
            let reg = allocate_register(reg_counter);
            asm.push(format!("LOAD r{} {}", reg, value));
            reg
        }
        Expr::BinOp(left, op, right) => {
            let left_reg = generate_expr(left, asm, reg_counter);
            let right_reg = generate_expr(right, asm, reg_counter);
            let result_reg = allocate_register(reg_counter);
            match op {
                BinOp::Add => {
                    asm.push(format!("ADD r{} r{} r{}", left_reg, right_reg, result_reg));
                }
            }
            result_reg
        }
    }
}

fn allocate_register(reg_counter: &mut usize) -> usize {
    let reg = *reg_counter;
    *reg_counter += 1;
    reg
}