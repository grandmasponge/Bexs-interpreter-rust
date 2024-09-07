use anyhow::Result;

use crate::expr::Expr;
use crate::expr::ExprError;
use crate::expr::ExprLiteral;
use crate::smnt::Statment;
use crate::Token;
use crate::TokenType;
use std::iter::Peekable;

pub struct Parser {
    pub Tokens: Vec<Token>,
    pub expr: Vec<Expr>,
    index: usize,
}
impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            index: 0,
            Tokens: tokens,
            expr: Vec::new(),
        }
    }

    pub fn stmtParser(&mut self) -> Result<Vec<Statment>, ExprError> {
        let mut statments = Vec::new();
        let mut index = 0;
        while !self.IsAtEnd() {
            statments.insert(index, self.statement()?);
            index += 1;
        }
        return Ok(statments);
    }

    pub fn statement(&mut self) -> Result<Statment, ExprError> {
        if self.matchexpr(&[TokenType::Print]) {
            return self.printStatment();
        }
        return self.ExprStatment();
    }

    pub fn ExprStatment(&mut self) -> Result<Statment, ExprError> {
        let expr = self.parse()?;
        if self.matchexpr(&[TokenType::SemiColon]) {
            return Ok(Statment::ExprStmt(expr)); // somthing
        } else {
            //return error
            return Ok(Statment::ExprStmt(expr));
        }
    }

    pub fn printStatment(&mut self) -> Result<Statment, ExprError> {
        let expr = self.parse()?;
        if self.matchexpr(&[TokenType::SemiColon]) {
            return Ok(Statment::PrintStmt(expr)); // somthing
        } else {
            //return error
            return Ok(Statment::PrintStmt(expr));
        }
    }

    pub fn parse(&mut self) -> Result<Expr, ExprError> {
        self.equality()
    }

    pub fn equality(&mut self) -> Result<Expr, ExprError> {
        let mut expr = self.comparison();
        while self.matchexpr(&[TokenType::Bang_EQUAL, TokenType::EQUAL_EQUAL]) {
            let operator = self.prev().clone().to_owned();
            let right = self.comparison();
            expr = Ok(Expr::Binary(operator, Box::new(expr?), Box::new(right?)));
        }

        return expr;
    }

    pub fn comparison(&mut self) -> Result<Expr, ExprError> {
        let mut expr = self.term();
        while self.matchexpr(&[
            TokenType::GreaterThan,
            TokenType::GreaterThan_EQUALS,
            TokenType::LessThan,
            TokenType::LessThan_EQUALS,
        ]) {
            let operator = self.prev().clone().to_owned();
            let right = self.factor();
            expr = Ok(Expr::Binary(operator, Box::new(expr?), Box::new(right?)));
        }
        expr
    }

    pub fn term(&mut self) -> Result<Expr, ExprError> {
        let mut expr = self.factor();
        while self.matchexpr(&[TokenType::Minus, TokenType::Plus]) {
            let operator = self.prev().clone().to_owned();
            let right = self.factor();
            expr = Ok(Expr::Binary(operator, Box::new(expr?), Box::new(right?)))
        }
        expr
    }

    pub fn factor(&mut self) -> Result<Expr, ExprError> {
        let mut expr = self.unary();

        while self.matchexpr(&[TokenType::Slash, TokenType::Star]) {
            let operator = self.prev().clone().to_owned();
            let right = self.unary();
            expr = Ok(Expr::Binary(operator, Box::new(expr?), Box::new(right?)));
        }
        expr
    }

    pub fn unary(&mut self) -> Result<Expr, ExprError> {
        if self.matchexpr(&[TokenType::Bang, TokenType::Minus]) {
            let operator = self.prev().clone().to_owned();
            let expr = self.unary()?;
            return Ok(Expr::Unary(operator, Box::new(expr)));
        }

        self.primary()
    }

    pub fn primary(&mut self) -> Result<Expr, ExprError> {
        let current = self.Tokens.get(self.index);
        match current {
            Some(Tok) => match Tok._type {
                TokenType::True => {
                    self.advance();
                    Ok(Expr::Literal(ExprLiteral::Bool(true)))
                }
                TokenType::False => {
                    self.advance();
                    Ok(Expr::Literal(ExprLiteral::Bool(false)))
                }
                TokenType::Nil => {
                    self.advance();
                    Ok(Expr::Literal(ExprLiteral::Nil))
                }
                TokenType::String => {
                    let value = current.unwrap().clone()._value.unwrap();
                    self.advance();
                    Ok(Expr::Literal(ExprLiteral::String(value)))
                }
                TokenType::Number => {
                    let value = current.unwrap().clone()._value.unwrap();
                    self.advance();
                    Ok(Expr::Literal(ExprLiteral::Number(value)))
                }
                TokenType::LeftParen => {
                    self.advance();
                    let inner = self.equality()?;
                    if let Some(Tok) = self.Tokens.get(self.index) {
                        if Tok._type == TokenType::RightParen {
                            self.advance();
                            return Ok(Expr::Grouping(Box::new(inner)));
                        } else {
                            return Err(ExprError::MissingToken(65));
                        }
                    } else {
                        return Err(ExprError::new("oops".to_string(), 65));
                    }
                }
                _ => Err(ExprError::MissingToken(65)),
            },
            None => Err(ExprError::new("bad expr".to_string(), 65)),
        }
    }

    pub fn matchexpr(&mut self, types: &[TokenType]) -> bool {
        for _type in types {
            if _type == &self.peek()._type {
                self.advance();
                return true;
            }
        }
        false
    }

    pub fn advance(&mut self) {
        self.index += 1
    }

    pub fn IsAtEnd(&mut self) -> bool {
        if self.peek()._type == TokenType::EOF {
            return true;
        }
        false
    }

    pub fn peek(&self) -> &Token {
        return &self.Tokens.get(self.index).unwrap();
    }

    pub fn prev(&self) -> &Token {
        return &self.Tokens.get(self.index - 1).unwrap();
    }
}
