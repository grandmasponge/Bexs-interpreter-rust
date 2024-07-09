use core::fmt;
use core::panic;
use std::env;
use std::fs;
use std::io::stdout;
use std::io::{self, Write};
use std::process::exit;

enum Token {
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
    NewLine,
}

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

impl Token {
    fn output(tok: char, line: i32) -> Result<Token, TokenError> {
        match tok {
            '(' => Ok(Token::LeftParen),
            ')' => Ok(Token::RightParen),
            '{' => Ok(Token::LeftBrace),
            '}' => Ok(Token::RightBrace),
            ',' => Ok(Token::Comma),
            '.' => Ok(Token::Dot),
            '-' => Ok(Token::Minus),
            '+' => Ok(Token::Plus),
            ';' => Ok(Token::SemiColon),
            '*' => Ok(Token::Star),
            '\n' => Ok(Token::NewLine),
            _ => Err(TokenError::new(
                format!("Error: Unexpected character: {}", tok),
                line,
                65,
            )),
        }
    }

    fn PrintOutput() {}
}

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Token::LeftParen => write!(f, "LEFT_PAREN ( null"),
            Token::RightParen => write!(f, "RIGHT_PAREN ) null"),
            Token::LeftBrace => write!(f, "LEFT_BRACE {{ null"),
            Token::RightBrace => write!(f, "RIGHT_BRACE }} null"),
            Token::Comma => write!(f, "COMMA , null"),
            Token::Dot => write!(f, "DOT . null"),
            Token::Minus => write!(f, "MINUS - null"),
            Token::Plus => write!(f, "PLUS + null"),
            Token::SemiColon => write!(f, "SEMICOLON ; null"),
            Token::Star => write!(f, "STAR * null"),
            _ => write!(f, "EOF  null"),
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
            tokenize(&file_contents);
        }
        _ => {
            writeln!(io::stderr(), "Unknown command: {}", command).unwrap();
            return;
        }
    }
}

fn tokenize(contents: &str) {
    let mut line: i32 = 1;
    let chars = contents.chars();
    for char in chars {
        let token = Token::output(char, line);
        match token {
            Ok(tok) => {
                println!("{}", tok);
            }
            Err(e) => {
                writeln!(io::stderr(), "{}", e);
                exit(e.exitcode);
            }
        }
    }
    writeln!(stdout(), "EOF null");
}
