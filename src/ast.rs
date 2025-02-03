#[derive(Debug)]
pub enum Expr {
    Number(f64),
    Variable(String),
    BinaryOp {
        op: BinOp,
        left: Box<Expr>,
        right: Box<Expr>,
    },
    Call {
        function: String,
        args: Vec<Expr>,
    },
}

#[derive(Debug)]
pub enum BinOp {
    Add,
    Subtract,
    Multiply,
    Divide,
}

#[derive(Debug)]
pub struct Function {
    pub name: String,
    pub params: Vec<String>,
    pub body: Vec<Expr>,
}

#[derive(Debug)]
pub struct Program {
    pub functions: Vec<Function>,
}