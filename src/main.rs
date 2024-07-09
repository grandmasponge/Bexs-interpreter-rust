use core::fmt;
use core::panic;
use std::env;
use std::fs;
use std::io::stdout;
use std::io::{self, Write};
use std::iter::Peekable;
use std::process::exit;
use std::ptr::write;
use std::str::Chars;

use anyhow::Ok;

struct Lexer<'a> {
    line: i32,
    charaters: Chars<'a>,
}

impl<'a> Lexer<'a> {
    fn new(contents: &'a str) -> Self {
        let mut line = 1;
        let chars = contents.chars();
        Self {
            line,
            charaters: chars,
        }
    }

    fn tokenize(&mut self) -> i32 {
        let mut exitcode = 0;
        let mut tokens = Vec::new();
        let mut characters = self.charaters.clone().peekable();
        while let Some(char) = characters.next() {
            match char {
                '\n' => {
                    self.line += 1;
                }
                '(' => tokens.push(Token::newToken(
                    TokenType::LeftParen,
                    char.to_string(),
                    None,
                )),
                ')' => tokens.push(Token::newToken(
                    TokenType::RightParen,
                    char.to_string(),
                    None,
                )),
                '{' => tokens.push(Token::newToken(
                    TokenType::LeftBrace,
                    char.to_string(),
                    None,
                )),
                '}' => tokens.push(Token::newToken(
                    TokenType::RightBrace,
                    char.to_string(),
                    None,
                )),
                ',' => tokens.push(Token::newToken(TokenType::Comma, char.to_string(), None)),
                '.' => tokens.push(Token::newToken(TokenType::Dot, char.to_string(), None)),
                '-' => tokens.push(Token::newToken(TokenType::Minus, char.to_string(), None)),
                '+' => tokens.push(Token::newToken(TokenType::Plus, char.to_string(), None)),
                '*' => tokens.push(Token::newToken(TokenType::Star, char.to_string(), None)),
                ';' => tokens.push(Token::newToken(
                    TokenType::SemiColon,
                    char.to_string(),
                    None,
                )),
                '=' => {
                    let mut peeker = characters.clone().peekable();
                    if peeker.next() == Some('=') {
                        tokens.push(Token::newToken(
                            TokenType::EQUAL_EQUAL,
                            "==".to_string(),
                            None,
                        ));
                        characters.next();
                    } else {
                        tokens.push(Token::newToken(TokenType::EQUAL, char.to_string(), None));
                    }
                }
                _ => {
                    exitcode = 65;
                    let error = TokenError::new(
                        format!("Error: Unexpected charater: {}", char),
                        self.line,
                        65,
                    );
                    println!("{error}");
                }
            };
        }
        for token in tokens {
            println!("{token}");
        }
        exitcode
    }
}

struct Token {
    _type: TokenType,
    _string: String,
    _value: Option<String>,
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let value = match &self._value {
            Some(value) => value,
            None => "null",
        };
        write!(f, "{} {} {}", self._type, self._string, value)
    }
}

impl Token {
    fn newToken(_type: TokenType, _string: String, _value: Option<String>) -> Self {
        Self {
            _type,
            _string,
            _value,
        }
    }
}

enum TokenType {
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Star,
    Plus,
    SemiColon,
    EOF,
    EQUAL,
    EQUAL_EQUAL,
    NewLine,
}

#[derive(Debug)]
struct TokenError {
    msg: String,
    line: i32,
    exitcode: i32,
}

impl TokenError {
    fn new(msg: String, line: i32, exitcode: i32) -> Self {
        Self {
            msg,
            line,
            exitcode,
        }
    }
}

impl std::fmt::Display for TokenError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[line {}] {}", self.line, self.msg)
    }
}

impl std::fmt::Display for TokenType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TokenType::LeftParen => write!(f, "LEFT_PAREN"),
            TokenType::LeftBrace => write!(f, "LEFT_BRACE"),
            TokenType::RightParen => write!(f, "RIGHT_PAREN"),
            TokenType::RightBrace => write!(f, "RIGHT_BRACE"),
            TokenType::Comma => write!(f, "COMMA"),
            TokenType::Dot => write!(f, "DOT"),
            TokenType::Minus => write!(f, "MINUS"),
            TokenType::Plus => write!(f, "PLUS"),
            TokenType::SemiColon => write!(f, "SEMICOLON"),
            TokenType::Star => write!(f, "STAR"),
            TokenType::EQUAL => write!(f, "EQUAL"),
            &TokenType::EQUAL_EQUAL => write!(f, "EQUAL_EQUAL"),
            _ => write!(f, "EOF"),
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        writeln!(io::stderr(), "Usage: {} tokenize <filename>", args[0]).unwrap();
        return;
    }

    let command = &args[1];
    let filename = &args[2];

    match command.as_str() {
        "tokenize" => {
            // You can use print statements as follows for debugging, they'll be visible when running tests.
            writeln!(io::stderr(), "Logs from your program will appear here!").unwrap();

            let file_contents = fs::read_to_string(filename).unwrap_or_else(|_| {
                writeln!(io::stderr(), "Failed to read file {}", filename).unwrap();
                String::new()
            });
            let mut lexer = Lexer::new(&file_contents);

            let result = lexer.tokenize();
            println!("EOF  null");
            exit(result)
        }
        _ => {
            writeln!(io::stderr(), "Unknown command: {}", command).unwrap();
            return;
        }
    }
}
