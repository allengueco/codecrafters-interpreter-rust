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
        Greater,
        Less,
        Slash,
        Tab,
        Space,
    }

    pub enum TokenPair {
        EqualEqual,
        BangEqual,
        LessEqual,
        SlashSlash,
        GreaterEqual,
    }

    impl<T> TryFrom<(T, T)> for TokenPair
    where
        T: TryInto<Token>,
    {
        type Error = ();

        fn try_from(value: (T, T)) -> Result<TokenPair, ()> {
            match (value.0.try_into(), value.1.try_into()) {
                (Ok(Token::Equal), Ok(Token::Equal)) => Ok(TokenPair::EqualEqual),
                (Ok(Token::Bang), Ok(Token::Equal)) => Ok(TokenPair::BangEqual),
                (Ok(Token::Less), Ok(Token::Equal)) => Ok(TokenPair::LessEqual),
                (Ok(Token::Greater), Ok(Token::Equal)) => Ok(TokenPair::GreaterEqual),
                (Ok(Token::Slash), Ok(Token::Slash)) => Ok(TokenPair::SlashSlash),
                _ => Err(()),
            }
        }
    }

    impl Display for TokenPair {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            let pair = match self {
                TokenPair::BangEqual => "BANG_EQUAL != null\n",
                TokenPair::EqualEqual => "EQUAL_EQUAL == null\n",
                TokenPair::GreaterEqual => "GREATER_EQUAL >= null\n",
                TokenPair::LessEqual => "LESS_EQUAL <= null\n",
                TokenPair::SlashSlash => "",
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
                Token::LeftParen => "LEFT_PAREN ( null\n",
                Token::RightParen => "RIGHT_PAREN ) null\n",
                Token::LeftBrace => "LEFT_BRACE { null\n",
                Token::RightBrace => "RIGHT_BRACE } null\n",
                Token::Star => "STAR * null\n",
                Token::Dot => "DOT . null\n",
                Token::Plus => "PLUS + null\n",
                Token::Minus => "MINUS - null\n",
                Token::Comma => "COMMA , null\n",
                Token::Semicolon => "SEMICOLON ; null\n",
                Token::Equal => "EQUAL = null\n",
                Token::Bang => "BANG ! null\n",
                Token::Greater => "GREATER > null\n",
                Token::Less => "LESS < null\n",
                Token::Slash => "SLASH / null\n",
                Token::Space => "",
                Token::Tab => "",
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
                '>' => Ok(Token::Greater),
                '<' => Ok(Token::Less),
                '/' => Ok(Token::Slash),
                ' ' => Ok(Token::Space),
                '\t' => Ok(Token::Tab),
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
                        match pair {
                            TokenPair::SlashSlash => while let Some(_) = iter.next() {},
                            _ => {
                                print!("{}", pair);
                                iter.next();
                            }
                        }
                    } else {
                        match Token::try_from(t1) {
                            Ok(token) => print!("{}", token),
                            Err(e) => {
                                error = true;
                                writeln!(io::stderr(), "{}", e)
                            }?,
                        };
                    }
                } else {
                    match Token::try_from(t1) {
                        Ok(token) => print!("{}", token),
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
