// This file is part of "jup"
// Under the MIT License
// Copyright (c) Junon, Antonin Hérault

use crate::lang::tokens::Token;
use crate::lang::elements::{ Element, function::Function, variable::Variable };

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
            if i != self.n_token {
                continue;
            }

            self.n_token = i;
            let element = self.check();
            self.parsed.push(element);
        }
    }

    fn check(&mut self) -> Element {
        self.n_token += 1;
        match &self.tokenized[self.n_token -1] {
            Token::Function => self.when_function(),
            Token::Variable => self.when_variable(),
            token => Element::Other(token.clone()),
        }
    }

    fn when_function(&mut self) -> Element {
        let id = self.retrieve_id();
        
        let params = vec![]; // TODO : Parameters retrieving
        let return_type = self.retrieve_type_token();
        
        Element::Function(Function::new(id, params, return_type))
    }

    fn when_variable(&mut self) -> Element {
        Element::Variable(Variable::new(
            self.retrieve_id(),
            self.retrieve_type_token(),
            self.retrieve_value_or_expr()
        ))
    }

    fn retrieve_id(&mut self) -> Token {
        self.n_token += 1; // skip id
        self.tokenized[self.n_token -1].clone()
    }

    fn retrieve_type_token(&mut self) -> Token {
        // When the type is explicitly written
        if self.tokenized[self.n_token] == Token::TypeDef {
            self.n_token += 2; // skip Token::TypeDef and type
            self.tokenized[self.n_token -1].clone()
        } else {
            Token::None
        }
    }

    fn retrieve_value_or_expr(&mut self) -> Token {
        if self.tokenized[self.n_token] == Token::Assign {
            self.n_token += 2; // skip Token::Assign and value/Token::BracketOpen
            self.tokenized[self.n_token -1].clone()
        } else {
            Token::None
        }
    }

    pub fn parsed(&self) -> &Vec<Element> {
        &self.parsed
    }
}

#[test]
pub fn run_parser() {
    use std::path::Path;
    use crate::tokenizer::Tokenizer;

    let mut tokenizer = Tokenizer::from_path(Path::new("tests/parser.ju"))
        .unwrap();
    tokenizer.run();

    println!("{:?} :\n", tokenizer.tokenized());

    let mut parser = Parser::new(tokenizer.tokenized());
    parser.run();

    println!("{:#?}", parser.parsed());
}
