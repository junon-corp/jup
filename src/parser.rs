// This file is part of "jup"
// Under the MIT License
// Copyright (c) Junon, Antonin Hérault

use crate::lang::tokens::Token;
use crate::lang::elements::{ Element, function::Function };

/// Transforms tokens to a collection of `Element` to be easily used by the 
/// compiler
pub struct Parser<'a> {
    tokenized: &'a Vec<Token>,
    parsed: Vec<Element>,
    n_token: usize,
}

impl<'a> Parser<'a> {
    /// Takes a tokenized file thanks to `Tokenizer` to parse these tokens
    pub fn new(tokenized: &'a Vec<Token>) -> Self {
        Parser {
            tokenized,
            parsed: vec![],
            n_token: 0,
        }
    }

    pub fn run(&mut self) {
        for (i, token) in (*self.tokenized).iter().enumerate() {
            if i < self.n_token {
                self.n_token -= 1;
                continue;
            }

            self.n_token = i;
            let element = self.check();
            self.parsed.push(element);
        }
    }

    fn check(&mut self) -> Element {
        match &self.tokenized[self.n_token] {
            Token::Function => self.when_function(),
            token => Element::Other(token.clone()),
        }
    }

    fn when_function(&mut self) -> Element {
        let id = self.tokenized[self.n_token + 1].clone();
        self.n_token += 1;
        let params = vec![]; // TODO, then update `value`'s definition
        
        let return_type = {
            if self.tokenized[self.n_token + 2] == Token::TypeDef {
                self.n_token += 1;
                self.tokenized[self.n_token + 3].clone()
            } else {
                // A byte is returned (0 or 1, ok or err)
                Token::Other("byte".to_string())
            }
        };

        Element::Function(Function::new(id, params, return_type))
    }

    pub fn parsed(&self) -> &Vec<Element> {
        &self.parsed
    }
}

#[test]
pub fn run_parser() {
    use crate::tokenizer::Tokenizer;

    let source_code = concat!(
        "fun main {", "\n",
        "    let a: int = 5", "\n", 
        "    let b: bigint", "\n",
        "}",
    );

    let mut tokenizer = Tokenizer::from_source_code(&source_code);
    tokenizer.run();

    let mut parser = Parser::new(tokenizer.tokenized());
    parser.run();

    println!("{:?}", parser.parsed());
}
