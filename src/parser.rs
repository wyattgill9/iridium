use std::fs::File;
use std::io::Read;
use std::path::Path;

// AST Definitions
#[derive(Debug)]
pub enum Expr {
    Variable(String),
    Literal(i64),
    BinOp(Box<Expr>, BinOp, Box<Expr>),
}

#[derive(Debug)]
pub enum BinOp {
    Add,
}

#[derive(Debug)]
pub enum Statement {
    Declare(String, Expr),
    Print(Expr),
}

#[derive(Debug)]
pub struct Program {
    pub statements: Vec<Statement>,
}

// Token Definitions
#[derive(Debug, PartialEq)]
enum Token {
    Int,
    Print,
    Ident(String),
    Literal(i64),
    Equals,
    Plus,
    Semicolon,
    LParen,
    RParen,
}

// Lexer
struct Lexer {
    chars: Vec<char>,
    pos: usize,
}

impl Lexer {
    fn new(input: &str) -> Self {
        Lexer {
            chars: input.chars().collect(),
            pos: 0,
        }
    }

    fn next_token(&mut self) -> Option<Token> {
        self.skip_whitespace();
        
        if self.pos >= self.chars.len() {
            return None;
        }

        let c = self.chars[self.pos];
        
        match c {
            'a'..='z' | 'A'..='Z' => self.parse_identifier(),
            '0'..='9' => self.parse_number(),
            '=' => {
                self.pos += 1;
                Some(Token::Equals)
            }
            '+' => {
                self.pos += 1;
                Some(Token::Plus)
            }
            ';' => {
                self.pos += 1;
                Some(Token::Semicolon)
            }
            '(' => {
                self.pos += 1;
                Some(Token::LParen)
            }
            ')' => {
                self.pos += 1;
                Some(Token::RParen)
            }
            _ => panic!("Unexpected character: {} at position {}", c, self.pos),
        }
    }

    fn parse_identifier(&mut self) -> Option<Token> {
        let start = self.pos;
        while self.pos < self.chars.len() && self.chars[self.pos].is_alphanumeric() {
            self.pos += 1;
        }
        let ident: String = self.chars[start..self.pos].iter().collect();

        match ident.as_str() {
            "int" => Some(Token::Int),
            "Print" => Some(Token::Print),
            _ => Some(Token::Ident(ident)),
        }
    }

    fn parse_number(&mut self) -> Option<Token> {
        let start = self.pos;
        while self.pos < self.chars.len() && self.chars[self.pos].is_ascii_digit() {
            self.pos += 1;
        }
        let num: String = self.chars[start..self.pos].iter().collect();
        Some(Token::Literal(num.parse().unwrap()))
    }

    fn skip_whitespace(&mut self) {
        while self.pos < self.chars.len() && self.chars[self.pos].is_whitespace() {
            self.pos += 1;
        }
    }
}

// Parser
pub struct Parser {
    tokens: Vec<Token>,
    pos: usize,
}

impl Parser {
    fn new(tokens: Vec<Token>) -> Self {
        Parser { tokens, pos: 0 }
    }

    fn parse(&mut self) -> Program {
        let mut statements = Vec::new();

        while self.pos < self.tokens.len() {
            statements.push(self.parse_statement());
        }

        Program { statements }
    }

    fn parse_statement(&mut self) -> Statement {
        match self.peek() {
            Some(Token::Int) => self.parse_declaration(),
            Some(Token::Print) => self.parse_print(),
            _ => panic!("Unexpected token"),
        }
    }

    fn parse_declaration(&mut self) -> Statement {
        self.consume(Token::Int);
        let ident = self.parse_ident();
        self.consume(Token::Equals);
        let expr = self.parse_expr();
        self.consume(Token::Semicolon);
        Statement::Declare(ident, expr)
    }

    fn parse_print(&mut self) -> Statement {
        self.consume(Token::Print);
        self.consume(Token::LParen);
        let expr = self.parse_expr();
        self.consume(Token::RParen);
        self.consume(Token::Semicolon);
        Statement::Print(expr)
    }

    fn parse_expr(&mut self) -> Expr {
        let mut expr = self.parse_primary();

        while let Some(Token::Plus) = self.peek() {
            self.pos += 1;
            let right = self.parse_primary();
            expr = Expr::BinOp(Box::new(expr), BinOp::Add, Box::new(right));
        }

        expr
    }

    fn parse_primary(&mut self) -> Expr {
        match self.peek().unwrap() {
            Token::Ident(name) => {
                let name = name.clone();
                self.pos += 1;
                Expr::Variable(name)
            }
            Token::Literal(n) => {
                let n = *n;
                self.pos += 1;
                Expr::Literal(n)
            }
            _ => panic!("Unexpected token in expression"),
        }
    }

    fn parse_ident(&mut self) -> String {
        match self.peek().unwrap() {
            Token::Ident(name) => {
                let name = name.clone();
                self.pos += 1;
                name
            }
            _ => panic!("Expected identifier"),
        }
    }

    fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.pos)
    }

    fn consume(&mut self, expected: Token) {
        if let Some(token) = self.peek() {
            if std::mem::discriminant(token) == std::mem::discriminant(&expected) {
                self.pos += 1;
                return;
            }
        }
        panic!("Unexpected token");
    }
}

pub fn parse_file<P: AsRef<Path>>(path: P) -> Result<Program, std::io::Error> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let mut lexer = Lexer::new(&contents);
    let mut tokens = Vec::new();
    while let Some(token) = lexer.next_token() {
        tokens.push(token);
    }

    let mut parser = Parser::new(tokens);
    Ok(parser.parse())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parser() {
        let program = parse_file("example.sl").unwrap();
        println!("{:#?}", program);
    }
}