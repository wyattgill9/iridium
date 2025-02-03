use std::iter::Peekable;
use std::str::Chars;
use crate::ast::{Expr, Function, Program};

#[derive(Debug)]
pub enum ParseError {
    UnexpectedToken(String),
    ExpectedToken(String),
    InvalidNumber(String),
}

pub struct Parser<'a> {
    input: Peekable<Chars<'a>>,
}

impl<'a> Parser<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            input: input.chars().peekable(),
        }
    }

    fn consume_whitespace(&mut self) {
        while let Some(&c) = self.input.peek() {
            if c.is_whitespace() {
                self.input.next();
            } else {
                break;
            }
        }
    }

    fn parse_identifier(&mut self) -> Result<String, ParseError> {
        let mut ident = String::new();
        while let Some(&c) = self.input.peek() {
            if c.is_alphabetic() || c == '_' {
                ident.push(c);
                self.input.next();
            } else {
                break;
            }
        }
        if ident.is_empty() {
            Err(ParseError::UnexpectedToken("Expected identifier".to_string()))
        } else {
            Ok(ident)
        }
    }

    fn parse_number(&mut self) -> Result<Expr, ParseError> {
        let mut num_str = String::new();
        while let Some(&c) = self.input.peek() {
            if c.is_numeric() || c == '.' {
                num_str.push(c);
                self.input.next();
            } else {
                break;
            }
        }
        num_str
            .parse::<f64>()
            .map(Expr::Number)
            .map_err(|_| ParseError::InvalidNumber(num_str))
    }

    fn parse_expr(&mut self) -> Result<Expr, ParseError> {
        self.consume_whitespace();
        if let Some(&c) = self.input.peek() {
            if c.is_numeric() {
                return self.parse_number();
            } else if c.is_alphabetic() {
                let ident = self.parse_identifier()?;
                if self.input.peek() == Some(&'(') {
                    self.input.next(); // Consume '('
                    let mut args = Vec::new();
                    while self.input.peek() != Some(&')') {
                        args.push(self.parse_expr()?);
                        if self.input.peek() == Some(&',') {
                            self.input.next(); // Consume ','
                        }
                    }
                    self.input.next(); // Consume ')'
                    return Ok(Expr::Call {
                        function: ident,
                        args,
                    });
                } else {
                    return Ok(Expr::Variable(ident));
                }
            }
        }
        Err(ParseError::UnexpectedToken("Expected expression".to_string()))
    }

    fn parse_function(&mut self) -> Result<Function, ParseError> {
        self.consume_whitespace();
        if self.input.next() != Some('f') || self.input.next() != Some('n') {
            return Err(ParseError::ExpectedToken("Expected 'fn'".to_string()));
        }
        self.consume_whitespace();
        let name = self.parse_identifier()?;
        self.consume_whitespace();
        if self.input.next() != Some('(') {
            return Err(ParseError::ExpectedToken("Expected '('".to_string()));
        }
        let mut params = Vec::new();
        while self.input.peek() != Some(&')') {
            params.push(self.parse_identifier()?);
            if self.input.peek() == Some(&',') {
                self.input.next(); // Consume ','
            }
        }
        self.input.next(); // Consume ')'
        self.consume_whitespace();
        if self.input.next() != Some('{') {
            return Err(ParseError::ExpectedToken("Expected '{'".to_string()));
        }
        let mut body = Vec::new();
        while self.input.peek() != Some(&'}') {
            body.push(self.parse_expr()?);
            if self.input.peek() == Some(&';') {
                self.input.next(); // Consume ';'
            }
        }
        self.input.next(); // Consume '}'
        Ok(Function { name, params, body })
    }

    pub fn parse_program(&mut self) -> Result<Program, ParseError> {
        let mut functions = Vec::new();
        while self.input.peek().is_some() {
            functions.push(self.parse_function()?);
            self.consume_whitespace();
        }
        Ok(Program { functions })
    }
}