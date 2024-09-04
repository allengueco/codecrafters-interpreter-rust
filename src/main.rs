use std::fs;
use std::io::{self, Write};

use clap::Parser;
use cli::Args;
use cli::Commands;
use tokenizer::Token;
use tokenizer::TokenPair;
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
        Bang,
    }

    pub enum TokenPair {
        EqualEqual,
        BangEqual,
    }

    /**
     *
     */
    impl<T> TryFrom<(T, T)> for TokenPair
    where
        T: TryInto<Token>,
    {
        type Error = ();

        fn try_from(value: (T, T)) -> Result<TokenPair, ()> {
            match (value.0.try_into(), value.1.try_into()) {
                (Ok(Token::Equal), Ok(Token::Equal)) => Ok(TokenPair::EqualEqual),
                (Ok(Token::Bang), Ok(Token::Equal)) => Ok(TokenPair::BangEqual),
                _ => Err(()),
            }
        }
    }

    impl Display for TokenPair {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            let pair = match self {
                TokenPair::BangEqual => "BANG_EQUAL != null",
                TokenPair::EqualEqual => "EQUAL_EQUAL == null",
            };
            write!(f, "{}", pair)
        }
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
                Token::Bang => "BANG ! null",
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
                '!' => Ok(Token::Bang),
                _ => Err(TokenizeError::UnexpectedCharacter(1, value)),
            }
        }
    }
}

mod cli {
    use clap::{Parser, Subcommand};

    #[derive(Parser, Debug)]
    pub struct Args {
        #[command(subcommand)]
        pub cmd: Commands,
    }

    #[derive(Subcommand, Debug, Clone)]
    pub enum Commands {
        Tokenize { filename: String },
    }
}
fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    match args.cmd {
        Commands::Tokenize { filename } => {
            let mut error = false;
            let file_contents = fs::read_to_string(filename)?;
            let mut iter = file_contents.chars().peekable();
            while let Some(t1) = iter.next() {
                // check if the next symbol can be one combined
                if let Some(next) = iter.peek() {
                    if let Ok(pair) = TokenPair::try_from((t1, *next)) {
                        println!("{}", pair);
                        iter.next();
                    }
                } else {
                    match Token::try_from(t1) {
                        Ok(token) => println!("{}", token),
                        Err(e) => {
                            error = true;
                            writeln!(io::stderr(), "{}", e)
                        }?,
                    };
                }
            }
            println!("EOF  null");
            if error {
                std::process::exit(65);
            } else {
                Ok(())
            }
        }
    }
}
