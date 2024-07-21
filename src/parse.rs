use anyhow::Result;

use crate::expr::Expr;
use crate::expr::ExprError;
use crate::expr::ExprLiteral;
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

    pub fn parse(&mut self) -> Result<Expr, ExprError> {
        let expr = self.equality();
        expr
    }

    pub fn equality(&mut self) -> Result<Expr, ExprError> {
        let mut expr = self.comparison();
        while self.matchexpr(&[TokenType::Bang_EQUAL, TokenType::EQUAL_EQUAL]) {}

        return expr;
    }

    pub fn comparison(&mut self) -> Result<Expr, ExprError> {
        let mut expr = self.term();
        while self.matchexpr(&[
            TokenType::GreaterThan,
            TokenType::GreaterThan_EQUALS,
            TokenType::LessThan,
            TokenType::LessThan_EQUALS,
        ]) {}
        expr
    }

    pub fn term(&mut self) -> Result<Expr, ExprError> {
        let mut expr = self.factor();
        while self.matchexpr(&[TokenType::Minus, TokenType::Plus]) {}
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

    pub fn peek(&self) -> &Token {
        return &self.Tokens.get(self.index).unwrap();
    }

    pub fn prev(&self) -> &Token {
        return &self.Tokens.get(self.index - 1).unwrap();
    }
}
