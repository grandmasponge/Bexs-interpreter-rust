use anyhow::Error;

use crate::expr::{Expr, ExprError, ExprLiteral};
use crate::smnt::Statment;
use crate::Token;
use crate::TokenType;

pub struct Parser {
    pub tokens: Vec<Token>,
    pub expr: Vec<Expr>,
    index: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            index: 0,
            tokens,
            expr: Vec::new(),
        }
    }

    pub fn stmt_parser(&mut self) -> Result<Vec<Statment>, ExprError> {
        let mut statments = Vec::new();
        while !self.is_at_end() {
            statments.push(self.statement()?);
        }
        Ok(statments)
    }

    pub fn statement(&mut self) -> Result<Statment, ExprError> {
        if self.matchexpr(&[TokenType::Var]) {
            //then it is a declaration :3
            self.var_decloration()
        } else if self.matchexpr(&[TokenType::Print]) {
            self.print_statment()
        } else {
            self.expr_statment()
        }
    }

    pub fn var_decloration(&mut self) -> Result<Statment, ExprError> {
        let identifier = self.parse()?;
        //should actually check if it is of identifier type
        if let Expr::Literal(ExprLiteral::Identifier(value)) = &identifier {
        } else {
            return Err(ExprError::new("expected an identifier".to_string(), 100));
        }

        //
        if self.matchexpr(&[TokenType::SemiColon]) {
            // then set it to nil
            return Ok(Statment::VarDec(
                identifier,
                Expr::Literal(ExprLiteral::Nil),
            ));
        } else if self.matchexpr(&[TokenType::EQUAL]) {
            // then set it to that expression
            let expr = self.parse()?;
            if self.matchexpr(&[TokenType::SemiColon]) {
                // return ok
                return Ok(Statment::VarDec(identifier, expr));
            } else {
                return Err(ExprError::new("SemiColon expected".to_string(), 100));
            }
        } else {
            return Err(ExprError::new("Unexpected token".to_string(), 100));
        }
    }

    pub fn expr_statment(&mut self) -> Result<Statment, ExprError> {
        let expr = self.parse()?;
        if self.matchexpr(&[TokenType::SemiColon]) {
            Ok(Statment::ExprStmt(expr))
        } else {
            Err(ExprError::new("Semicolon expected".to_string(), 100))
        }
    }

    pub fn print_statment(&mut self) -> Result<Statment, ExprError> {
        let expr = self.parse()?;
        if self.matchexpr(&[TokenType::SemiColon]) {
            Ok(Statment::PrintStmt(expr))
        } else {
            Err(ExprError::new("Semicolon expected".to_string(), 100))
        }
    }

    pub fn parse(&mut self) -> Result<Expr, ExprError> {
        self.equality()
    }

    pub fn equality(&mut self) -> Result<Expr, ExprError> {
        let mut expr = self.comparison()?;
        while self.matchexpr(&[TokenType::BangEqual, TokenType::EqualEqual]) {
            let operator = self.prev().clone();
            let right = self.comparison()?;
            expr = Expr::Binary(operator, Box::new(expr), Box::new(right));
        }
        Ok(expr)
    }

    pub fn comparison(&mut self) -> Result<Expr, ExprError> {
        let mut expr = self.term()?;
        while self.matchexpr(&[
            TokenType::GreaterThan,
            TokenType::GreaterThanEquals,
            TokenType::LessThan,
            TokenType::LessThanEquals,
        ]) {
            let operator = self.prev().clone();
            let right = self.term()?;
            expr = Expr::Binary(operator, Box::new(expr), Box::new(right));
        }
        Ok(expr)
    }

    pub fn term(&mut self) -> Result<Expr, ExprError> {
        let mut expr = self.factor()?;
        while self.matchexpr(&[TokenType::Minus, TokenType::Plus]) {
            let operator = self.prev().clone();
            let right = self.factor()?;
            expr = Expr::Binary(operator, Box::new(expr), Box::new(right));
        }
        Ok(expr)
    }

    pub fn factor(&mut self) -> Result<Expr, ExprError> {
        let mut expr = self.unary()?;
        while self.matchexpr(&[TokenType::Slash, TokenType::Star]) {
            let operator = self.prev().clone();
            let right = self.unary()?;
            expr = Expr::Binary(operator, Box::new(expr), Box::new(right));
        }
        Ok(expr)
    }

    pub fn unary(&mut self) -> Result<Expr, ExprError> {
        if self.matchexpr(&[TokenType::Bang, TokenType::Minus]) {
            let operator = self.prev().clone();
            let expr = self.unary()?;
            return Ok(Expr::Unary(operator, Box::new(expr)));
        }

        self.primary()
    }

    pub fn primary(&mut self) -> Result<Expr, ExprError> {
        let current = self
            .tokens
            .get(self.index)
            .ok_or_else(|| ExprError::new("Unexpected end of input".to_string(), 65))?;
        match current._type {
            TokenType::Identifer => {
                let identifer_string = current._string.clone();

                self.advance();
                Ok(Expr::Literal(ExprLiteral::Identifier(identifer_string)))
            }
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
                let value = current
                    ._value
                    .clone()
                    .ok_or_else(|| ExprError::new("Expected string value".to_string(), 65))?;
                self.advance();
                Ok(Expr::Literal(ExprLiteral::String(value)))
            }
            TokenType::Number => {
                let value = current
                    ._value
                    .clone()
                    .ok_or_else(|| ExprError::new("Expected number value".to_string(), 65))?;
                self.advance();
                Ok(Expr::Literal(ExprLiteral::Number(value)))
            }
            TokenType::LeftParen => {
                self.advance();
                let inner = self.equality()?;
                if let Some(tok) = self.tokens.get(self.index) {
                    if tok._type == TokenType::RightParen {
                        self.advance();
                        Ok(Expr::Grouping(Box::new(inner)))
                    } else {
                        Err(ExprError::new("Expected ')'".to_string(), 65))
                    }
                } else {
                    Err(ExprError::new("Expected ')'".to_string(), 65))
                }
            }
            _ => Err(ExprError::new("Unexpected token".to_string(), 65)),
        }
    }

    pub fn matchexpr(&mut self, types: &[TokenType]) -> bool {
        for _type in types {
            if self.peek()._type == *_type {
                self.advance();
                return true;
            }
        }
        false
    }

    pub fn advance(&mut self) {
        if !self.is_at_end() {
            self.index += 1;
        }
    }

    pub fn is_at_end(&self) -> bool {
        matches!(self.peek()._type, TokenType::EOF)
    }

    pub fn peek(&self) -> &Token {
        self.tokens
            .get(self.index)
            .expect("Unexpected end of input")
    }

    pub fn prev(&self) -> &Token {
        self.tokens
            .get(self.index.saturating_sub(1))
            .expect("No previous token")
    }
}

