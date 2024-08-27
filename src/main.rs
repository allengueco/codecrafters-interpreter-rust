use std::env;
use std::fs;
use std::io::{self, Write};

use tokenizer::Token;
mod tokenizer {
    use std::fmt::Display;

    pub enum Token {
        LeftParen,
        RightParen,
        LeftBrace,
        RightBrace,
        Star,
        Dot,
        Plus,
        Minus,
        Comma,
        Equal,
        EqualEqual,
        Semicolon,
    }

    pub enum TokenizeError {
        UnexpectedCharacter(usize, char),
    }

    impl Display for TokenizeError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            let err = match &self {
                TokenizeError::UnexpectedCharacter(line, c) => {
                    format!("[line {0}] Error: Unexpected character: {1}", line, c)
                }
            };

            write!(f, "{}", err)
        }
    }

    impl Display for Token {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            let p = match self {
                Token::LeftParen => "LEFT_PAREN ( null",
                Token::RightParen => "RIGHT_PAREN ) null",
                Token::LeftBrace => "LEFT_BRACE { null",
                Token::RightBrace => "RIGHT_BRACE } null",
                Token::Star => "STAR * null",
                Token::Dot => "DOT . null",
                Token::Plus => "PLUS + null",
                Token::Minus => "MINUS - null",
                Token::Comma => "COMMA , null",
                Token::Semicolon => "SEMICOLON ; null",
                Token::Equal => "EQUAL = null",
                Token::EqualEqual => "EQUAL_EQUAL == null",
            };

            write!(f, "{}", p)
        }
    }

    impl TryFrom<char> for Token {
        type Error = TokenizeError;

        fn try_from(value: char) -> Result<Self, Self::Error> {
            match value {
                '(' => Ok(Token::LeftParen),
                ')' => Ok(Token::RightParen),
                '}' => Ok(Token::RightBrace),
                '{' => Ok(Token::LeftBrace),
                '+' => Ok(Token::Plus),
                '-' => Ok(Token::Minus),
                ',' => Ok(Token::Comma),
                ';' => Ok(Token::Semicolon),
                '.' => Ok(Token::Dot),
                '*' => Ok(Token::Star),
                '=' => Ok(Token::Equal),
                _ => Err(TokenizeError::UnexpectedCharacter(1, value)),
            }
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

            let mut error = false;
            if let Ok(file_contents) = fs::read_to_string(filename) {
                let mut iter = file_contents.chars().peekable();
                while let Some(c) = &iter.next() {
                    match Token::try_from(*c) {
                        Ok(token) => match token {
                            // if the current token is =
                            Token::Equal => {
                                // we peek the next token if it exists
                                if let Some(c) = iter.peek() {
                                    // if the next is also an equal,
                                    if *c == '=' {
                                        iter.next();
                                        println!("{}", Token::EqualEqual)
                                    } else {
                                        // if it doesn't exist, then we are at the end
                                        println!("{}", Token::Equal)
                                    }
                                } else {
                                    println!("{}", Token::Equal)
                                }
                            }
                            _ => println!("{}", token),
                        },
                        Err(err) => {
                            error = true;
                            writeln!(io::stderr(), "{}", err).unwrap()
                        }
                    }
                }
                println!("EOf  null");
            } else {
                writeln!(io::stderr(), "Failed to read file {}", filename).unwrap();
            }

            if error {
                std::process::exit(65)
            }
        }
        _ => {
            writeln!(io::stderr(), "Unknown command: {}", command).unwrap();
            return;
        }
    }
}
