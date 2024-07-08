use core::fmt;
use core::panic;
use std::env;
use std::fs;
use std::io::{self, Write};

enum Token {
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    EOF,
}

impl Token {
    fn output(tok: char) -> Token {
        match tok {
            '(' => Token::LeftParen,
            ')' => Token::RightParen,
            '{' => Token::LeftBrace,
            '}' => Token::RightBrace,
            _ => Token::EOF,
        }
    }
}

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Token::LeftParen => write!(f, "LEFT_PAREN ( null"),
            Token::RightParen => write!(f, "RIGHT_PAREN ) null"),
            Token::LeftBrace => write!(f, "LEFT_BRACE {{ null"),
            Token::RightBrace => write!(f, "RIGHT_BRACE }} null"),
            Token::EOF => write!(f, "EOF  null"),
            _ => write!(f, "huh"),
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
    let chars = contents.chars();
    for char in chars {
        let token = Token::output(char);
        println!("{token}");
    }
    println!("EOF  null");
}
