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
        Semicolon,
    }

    pub enum TokenizeError {
        UnexpectedCharacter(char),
    }

    impl Display for TokenizeError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            let err = match &self {
                TokenizeError::UnexpectedCharacter(c) => {
                    format!("[line 1] Error: Unexpected character: {c}")
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
                _ => Err(TokenizeError::UnexpectedCharacter(value)),
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

            let file_contents = fs::read_to_string(filename).unwrap_or_else(|_| {
                writeln!(io::stderr(), "Failed to read file {}", filename).unwrap();
                String::new()
            });

            let mut error = false;
            //Uncomment this block to pass the first stage
            if !file_contents.is_empty() {
                for c in file_contents.chars() {
                    match Token::try_from(c) {
                        Ok(token) => println!("{}", token),
                        Err(err) => {
                            error = true;
                            writeln!(io::stderr(), "{}", err).unwrap()
                        }
                    }
                }
                println!("EOF  null");
                if error {
                    std::process::exit(65)
                }
            } else {
                println!("EOF  null")
            }
        }
        _ => {
            writeln!(io::stderr(), "Unknown command: {}", command).unwrap();
            return;
        }
    }
}
