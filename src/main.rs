use core::fmt;
use std::char;
use std::env;
use std::fs;
use std::io::stderr;
use std::io::{self, Write};
use std::iter::Peekable;
use std::process::exit;

mod expr;
mod parse;

struct Lexer {
    line: i32,
    tokens: Vec<Token>,
}

impl Lexer {
    fn new() -> Self {
        let mut line = 1;
        Self {
            line,
            tokens: Vec::new(),
        }
    }

    fn tokenize<I>(&mut self, characters: &mut Peekable<I>) -> i32
    where
        I: Iterator<Item = char> + Clone,
    {
        let mut exitcode = 0;
        let mut tokens = Vec::new();

        while let Some(char) = characters.next() {
            match char {
                '\n' => {
                    self.line += 1;
                }
                ' ' | '\r' | '\t' => {}
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
                '!' => {
                    let mut peeker = characters.clone().peekable();
                    if peeker.next() == Some('=') {
                        tokens.push(Token::newToken(
                            TokenType::Bang_EQUAL,
                            "!=".to_string(),
                            None,
                        ));
                        characters.next();
                    } else {
                        tokens.push(Token::newToken(TokenType::Bang, char.to_string(), None));
                    }
                }
                '<' => {
                    let mut peeker = characters.clone().peekable();
                    if peeker.next() == Some('=') {
                        tokens.push(Token::newToken(
                            TokenType::LessThan_EQUALS,
                            "<=".to_string(),
                            None,
                        ));
                        characters.next();
                    } else {
                        tokens.push(Token::newToken(TokenType::LessThan, char.to_string(), None));
                    }
                }
                '>' => {
                    let mut peeker = characters.clone().peekable();
                    if peeker.next() == Some('=') {
                        tokens.push(Token::newToken(
                            TokenType::GreaterThan_EQUALS,
                            ">=".to_string(),
                            None,
                        ));
                        characters.next();
                    } else {
                        tokens.push(Token::newToken(
                            TokenType::GreaterThan,
                            char.to_string(),
                            None,
                        ));
                    }
                }
                '/' => {
                    let mut peeker = characters.clone().peekable();
                    if peeker.peek() == Some(&'/') {
                        //omgeh we have a comment
                        while let Some(end) = characters.next() {
                            if end == '\n' {
                                self.line += 1;
                                break;
                            }
                        }
                    } else if peeker.peek() == Some(&'*') {
                        while let Some(end) = characters.next() {
                            if end == '*' && characters.next() == Some('/') {
                                self.line += 1;
                                break;
                            }
                            if end == '\n' {
                                self.line += 1;
                            }
                        }
                    } else {
                        tokens.push(Token::newToken(TokenType::Slash, char.to_string(), None))
                    }
                }
                '"' => {
                    let mut error = true;
                    let mut value = Vec::new();
                    while let Some(char) = characters.next() {
                        match char {
                            '\n' => self.line += 1,
                            '"' => {
                                error = false;
                                let inner = value.iter().collect::<String>();
                                tokens.push(Token::newToken(
                                    TokenType::String,
                                    format!("\"{}\"", inner),
                                    Some(inner),
                                ));
                                break;
                            }
                            _ => value.push(char),
                        }
                    }
                    if error {
                        exitcode = 65;
                        let error = TokenError::new(
                            "Error: Unterminated string.".to_string(),
                            self.line,
                            65,
                        );
                        writeln!(stderr(), "{}", error).unwrap()
                    }
                }
                _ => {
                    if char.is_alphabetic() || char == '_' {
                        let mut buf = String::from(char);
                        while let Some(&next_char) = characters.peek() {
                            if next_char.is_ascii_alphanumeric() || next_char == '_' {
                                buf.push(next_char);
                                characters.next();
                            } else {
                                break;
                            }
                        }
                        match buf.as_str() {
                            "and" => tokens.push(Token::newToken(
                                TokenType::And,
                                "and".to_string(),
                                None,
                            )),
                            "class" => tokens.push(Token::newToken(
                                TokenType::Class,
                                "class".to_string(),
                                None,
                            )),
                            "else" => tokens.push(Token::newToken(
                                TokenType::Else,
                                "else".to_string(),
                                None,
                            )),
                            "false" => tokens.push(Token::newToken(
                                TokenType::False,
                                "false".to_string(),
                                None,
                            )),
                            "for" => tokens.push(Token::newToken(
                                TokenType::For,
                                "for".to_string(),
                                None,
                            )),
                            "fun" => tokens.push(Token::newToken(
                                TokenType::Fun,
                                "fun".to_string(),
                                None,
                            )),
                            "if" => {
                                tokens.push(Token::newToken(TokenType::If, "if".to_string(), None))
                            }
                            "nil" => tokens.push(Token::newToken(
                                TokenType::Nil,
                                "nil".to_string(),
                                None,
                            )),
                            "or" => {
                                tokens.push(Token::newToken(TokenType::or, "or".to_string(), None))
                            }
                            "print" => tokens.push(Token::newToken(
                                TokenType::Print,
                                "print".to_string(),
                                None,
                            )),
                            "return" => tokens.push(Token::newToken(
                                TokenType::Return,
                                "return".to_string(),
                                None,
                            )),
                            "super" => tokens.push(Token::newToken(
                                TokenType::Super,
                                "super".to_string(),
                                None,
                            )),
                            "this" => tokens.push(Token::newToken(
                                TokenType::This,
                                "this".to_string(),
                                None,
                            )),
                            "true" => tokens.push(Token::newToken(
                                TokenType::True,
                                "true".to_string(),
                                None,
                            )),
                            "var" => tokens.push(Token::newToken(
                                TokenType::Var,
                                "var".to_string(),
                                None,
                            )),
                            "while" => tokens.push(Token::newToken(
                                TokenType::While,
                                "while".to_string(),
                                None,
                            )),
                            _ => tokens.push(Token::newToken(TokenType::Identifer, buf, None)),
                        }
                    } else if char.is_ascii_digit() {
                        let mut has_dot = false;
                        let mut number = Vec::new();
                        number.push(char);

                        while let Some(&next_char) = characters.peek() {
                            if next_char.is_ascii_digit() {
                                number.push(next_char);
                                characters.next();
                            } else if next_char == '.' && !has_dot {
                                number.push(next_char);
                                has_dot = true;
                                characters.next();
                            } else {
                                break;
                            }
                        }
                        let mut numstr = number.clone().iter().collect::<String>();

                        if numstr.ends_with('.') {
                            numstr.push('0');
                            let mut actual: String = number.iter().collect::<String>();
                            actual.pop();

                            tokens.push(Token::newToken(TokenType::Number, actual, Some(numstr)));
                            tokens.push(Token::newToken(TokenType::Dot, ".".to_string(), None));
                        } else if numstr.ends_with('0') {
                            numstr.pop();
                            let actual = number.iter().collect::<String>();

                            tokens.push(Token::newToken(TokenType::Number, actual, Some(numstr)));
                        } else {
                            if !numstr.contains('.') || !has_dot {
                                numstr.push_str(".0");
                            }
                            tokens.push(Token::newToken(
                                TokenType::Number,
                                number.iter().collect::<String>(),
                                Some(numstr),
                            ));
                        }
                    } else {
                        exitcode = 65;
                        let error = TokenError::new(
                            format!("Error: Unexpected character: {}", char),
                            self.line,
                            65,
                        );
                        if let Err(e) = writeln!(stderr(), "{error}") {
                            eprintln!("Failed to write to stderr: {}", e);
                        }
                    }
                }
            };
        }
        self.tokens = tokens;
        exitcode
    }
}
#[derive(Clone)]
pub struct Token {
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
    pub fn newToken(_type: TokenType, _string: String, _value: Option<String>) -> Self {
        Self {
            _type,
            _string,
            _value,
        }
    }
}
#[derive(Clone, Copy, PartialEq)]
pub enum TokenType {
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
    Bang,
    Bang_EQUAL,
    LessThan,
    LessThan_EQUALS,
    GreaterThan,
    GreaterThan_EQUALS,
    Slash,
    String,
    Number,
    Identifer,
    And,
    Class,
    False,
    Else,
    For,
    Fun,
    If,
    Nil,
    or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,
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
            TokenType::EQUAL_EQUAL => write!(f, "EQUAL_EQUAL"),
            TokenType::Bang => write!(f, "BANG"),
            TokenType::Bang_EQUAL => write!(f, "BANG_EQUAL"),
            TokenType::GreaterThan => write!(f, "GREATER"),
            TokenType::GreaterThan_EQUALS => write!(f, "GREATER_EQUAL"),
            TokenType::LessThan_EQUALS => write!(f, "LESS_EQUAL"),
            TokenType::LessThan => write!(f, "LESS"),
            TokenType::Slash => write!(f, "SLASH"),
            TokenType::String => write!(f, "STRING"),
            TokenType::Number => write!(f, "NUMBER"),
            TokenType::Identifer => write!(f, "IDENTIFIER"),
            TokenType::And => write!(f, "AND"),
            TokenType::Class => write!(f, "CLASS"),
            TokenType::Else => write!(f, "ELSE"),
            TokenType::False => write!(f, "FALSE"),
            TokenType::Fun => write!(f, "FUN"),
            TokenType::For => write!(f, "FOR"),
            TokenType::If => write!(f, "IF"),
            TokenType::Nil => write!(f, "NIL"),
            TokenType::or => write!(f, "OR"),
            TokenType::Print => write!(f, "PRINT"),
            TokenType::Return => write!(f, "RETURN"),
            TokenType::Super => write!(f, "SUPER"),
            TokenType::This => write!(f, "THIS"),
            TokenType::True => write!(f, "TRUE"),
            TokenType::Var => write!(f, "VAR"),
            TokenType::While => write!(f, "WHILE"),
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

    let file_contents = fs::read_to_string(filename).unwrap_or_else(|_| {
        writeln!(io::stderr(), "Failed to read file {}", filename).unwrap();
        String::new()
    });

    match command.as_str() {
        "tokenize" => {
            // You can use print statements as follows for debugging, they'll be visible when running tests.
            writeln!(io::stderr(), "Logs from your program will appear here!").unwrap();

            let mut file_contents = file_contents.chars().peekable();
            let mut lexer = Lexer::new();

            let result = lexer.tokenize(&mut file_contents);
            for token in lexer.tokens {
                println!("{token}");
            }
            println!("EOF  null");
            exit(result)
        }
        "parse" => {
            let mut file_contents = file_contents.chars().peekable();
            let mut lexer = Lexer::new();
            let mut result = lexer.tokenize(&mut file_contents);
            if result == 65 {
                exit(65);
            }
            lexer
                .tokens
                .push(Token::newToken(TokenType::EOF, "".to_string(), None));

            let mut tokens = lexer.tokens.clone();

            let mut parser = parse::Parser::new(tokens.clone());

            let expr = parser.parse();
            match expr {
                Ok(tehe) => println!("{tehe}"),
                Err(e) => {
                    result = 65;
                    println!("{e}")
                }
            };
            exit(result);
        }
        _ => {
            writeln!(io::stderr(), "Unknown command: {}", command).unwrap();
            return;
        }
    }
}
