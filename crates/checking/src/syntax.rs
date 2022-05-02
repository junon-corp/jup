// This file is part of "jup"
// Under the MIT License
// Copyright (c) Junon, Antonin Hérault

use rslog::{
    *,
    level::LogLevel,
    log::Log,
    logger::Logger,
};

use crate::lang::tokens::Token;

pub struct SyntaxChecker<'a> {
    source: &'a str,
    parsed: &'a Vec<Token>,
    logger: Logger,

    previous_token: &'a Token,
    current_token: &'a Token,

    /// The current line index
    line_i: usize,
    // The current toke index (in all file, != than `token_i_on_line`)
    token_i: usize,
    /// The current token index on the current line (`line_i`)
    token_i_on_line: usize,

    /// "Should the line be read more ?"
    /// It stops analyzing the tokens on the current line
    break_line: bool,

    /// Positions of `NewLine` tokens
    new_lines: Vec<usize>,
}

impl<'a> SyntaxChecker<'a> {
    pub fn new(source: &'a String, parsed: &'a Vec<Token>) -> Self {
        // Retrieve all `NewLine` tokens positions
        let mut new_lines: Vec<usize> = vec![];
        for (i, token) in parsed.iter().enumerate() {
            if *token == Token::NewLine {
                new_lines.push(i + 1);
            }
        }

        Self {
            source,
            parsed,
            logger: Logger::new(),

            previous_token: &Token::None,
            current_token: &Token::None,

            line_i: 0,
            token_i: 0,
            token_i_on_line: 0,

            break_line: false,

            new_lines,
        }
    }

    pub fn run(&mut self) {
        for token in self.parsed {
            self.previous_token = self.current_token;
            self.current_token = token;

            // Skip token check until `Token::NewLine` found
            if self.break_line {
                self.increment();

                if *self.previous_token != Token::NewLine {
                    continue;
                }

                self.break_line = false;
            } else {
                self.increment();
            }

            self.check_token();
        }

        // Checking for the last token
        self.previous_token = self.current_token;
        self.check_token();

        self.logger.interpret();
    }

    /// We moved to the next token
    pub fn increment(&mut self) {
        self.token_i += 1;
        self.token_i_on_line += 1;
    }

    /// Checks the previous token when it's not already skipped with
    /// `break_line`
    pub fn check_token(&mut self) {
        let cause = source_to_string(
            self.source.to_string(),
            self.line_i,
            self.token_i
        );

        match *self.previous_token {
            Token::Assembly => { self.break_line = true; },
            Token::Assign => {},
            Token::BracketOpen => {},
            Token::BracketClose => {},
            Token::Comma => {},
            Token::Function => { self.break_line = true; },
            Token::ParenOpen => {},
            Token::ParenClose => {},
            Token::Point => {},
            Token::Return => { self.break_line = true; },
            Token::StringDot => {},
            Token::TypeDef => {},
            Token::Variable | Token::Static => {
                self.break_line = true;
            }

            // System calls
            Token::Print => { self.break_line = true; },
            Token::Exit => { self.break_line = true; },
            Token::NewLine => {
                self.token_i_on_line = 0;
                self.line_i += 1;
            }

            // First token of the parsed content
            Token::None => (),

            _ => {
                self.logger.add_log(
                    Log::new(
                        LogLevel::Error,
                        "Invalid token".to_string(),
                        format!(
                            "{}No valid instruction found for token '{}'",
                            &self.fmt_generate_line(),
                            self.previous_token.to_string()
                        )
                    )
                    .add_cause(&cause)
                    .finish()
                );
            }
        }
    }

    fn generate_line(&self) -> Vec<Token> {
        let i = self.new_lines[self.line_i - 1];

        let j = if i == self.new_lines[self.new_lines.len() - 1] {
            self.new_lines.len() - 1
        } else if self.line_i != self.new_lines.len() {
            self.new_lines[self.line_i]
        } else {
            self.new_lines.len() - 1
        };

        self.parsed[i..j - 1].to_vec()
    }

    fn fmt_generate_line(&self) -> String {
        line_to_string(&self.generate_line(), self.token_i_on_line)
    }
}
