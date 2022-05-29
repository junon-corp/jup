// This file is part of "jup"
// Under the MIT License
// Copyright (c) Junon, Antonin Hérault

use std::string::ToString;

/// All tokens list for the Junon programming
#[allow(unused)] // yes, you will probably not use all of them
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Token {
    Assembly,
    Assign,
    BracketOpen,
    BracketClose,
    Comma,
    Comment,
    Divide,
    Function,
    Minus,
    Multiply,
    ParenOpen,
    ParenClose,
    Point,
    Plus,
    Return,
    SemiColon,
    SquareBracketOpen,
    SquareBracketClose,
    Static,
    StringDot,
    TypeDef,
    Variable,

    // System calls
    Print,
    Exit,

    /// Example : "foo" or something like that is not a real token
    /// NOTE It can be a string, an value like an integer or a float
    Other(String),
    // Avoid using a 2D table and permit to do not respect line breaks
    NewLine, 
    /// Avoid `Option` usage \
    /// WARNING : It's not a a "real" token from the language
    None,
}

impl ToString for Token {
    /// A void string is returned when it cannot be converted to `String`
    fn to_string(&self) -> String {
        match *self {
            Self::Assembly => "@",
            Self::Assign => "=",
            Self::BracketOpen => "{",
            Self::BracketClose => "}",
            Self::Comma => ",",
            Self::Comment => "//",
            Self::Divide => "/",
            Self::Function => "fun",
            Self::Minus => "-",
            Self::Multiply => "*",
            Self::ParenOpen => "(",
            Self::ParenClose => ")",
            Self::Point => ".",
            Self::Plus => "+",
            Self::Return => "ret",
            Self::SemiColon => ";",
            Self::SquareBracketOpen => "[",
            Self::SquareBracketClose => "]",
            Self::Static => "static",
            Self::StringDot => "'",
            Self::TypeDef => ":",
            Self::Variable => "let",

            Self::Print => "print",
            Self::Exit => "exit",

            Self::Other(ref string) => &*string,
            Self::NewLine => "\n",
            Self::None => "",
        }.to_string()
    }
}

impl Token {
    /// Convert a string into a `Token` object \
    /// If the string does not correspond to any token, it will return a
    /// `Token::Other` object with contained string into
    pub fn token_from_str(string: &str) -> Self {
        match string {
            "@" => Self::Assembly,
            "=" => Self::Assign,
            "{" => Self::BracketOpen,
            "}" => Self::BracketClose,
            "," => Self::Comma,
            "//" => Self::Comment,
            "/" => Self::Divide,
            "fun" => Self::Function,
            "-" => Self::Minus,
            "*" => Self::Multiply,
            "(" => Self::ParenOpen,
            ")" => Self::ParenClose,
            "." => Self::Point,
            "+" => Self::Plus,
            "ret" => Self::Return,
            ";" => Self::SemiColon,
            "[" => Self::SquareBracketOpen,
            "]" => Self::SquareBracketClose,
            "static" => Self::Static,
            "'" => Self::StringDot,
            ":" => Self::TypeDef,
            "let" => Self::Variable,

            "print" => Self::Print,
            "exit" => Self::Exit,

            "\n" => Self::NewLine,
            _ => Self::Other(string.to_string()),
        }
    }

    /// The string as `&String` is converted into a `&str` before returning
    /// a `::from_str()` call
    pub fn from_string(string: &str) -> Self {
        Self::token_from_str(string)
    }
}

// Don't forget to add "-- --nocapture" flags to the command line arguments
// when you execute `cargo test`

#[test]
fn convert_to_string() {
    let token_function = Token::Function;
    println!("{}", token_function.to_string());
}

#[test]
fn from_string() {
    let string = String::from("fun");
    println!("{:?}", Token::from_string(&string));
}

#[test]
fn from_str() {
    let string: &str = "fun";
    println!("{:?}", Token::token_from_str(string));
}
