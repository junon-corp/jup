// This file is part of "jup"
// Under the MIT License
// Copyright (c) Junon, Antonin Hérault

use crate::lang::tokens::Token;
use crate::lang::elements::{
    Element, 
    function::Function, 
    operation::Operation, 
    variable::Variable
};

/// Transforms tokens to a collection of `Element` to be easily used by the 
/// compiler
pub struct Parser {
    tokenized: Vec<Token>,
    parsed: Vec<Element>,
    n_token: usize,
}

impl Parser {
    /// Takes a tokenized file thanks to `Tokenizer` to parse these tokens
    pub fn new(tokenized: Vec<Token>) -> Self {
        Parser {
            tokenized,
            parsed: vec![],
            n_token: 0,
        }
    }

    pub fn run(&mut self) {
        for (i, token) in self.tokenized.clone().iter().enumerate() {
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
            Token::BracketOpen => self.when_expression(),
            Token::Function => self.when_function(),
            Token::Plus | Token::Minus | Token::Multiply | Token::Divide 
                | Token::Assign => self.when_operation(),
            Token::Return => self.when_return(),
            Token::Variable => self.when_variable(),
            token => Element::Other(token.clone()),
        }
    }

    fn when_expression(&mut self) -> Element {
        // Retrieves tokens into the expression
        let mut expr_tokens = self.tokenized[self.n_token..].to_vec();

        // Skip sub expressions into the expression to avoid finishing the 
        // expression before it's really finished
        let mut i_end_expr = 0;
        let mut is_sub_expression = 0;

        for token in expr_tokens.iter() {
            match *token {
                Token::BracketOpen => is_sub_expression += 1,
                Token::BracketClose => {
                    if is_sub_expression == 0 {
                        break;
                    }
                    is_sub_expression -= 1;
                }
                _ => {}
            }
            i_end_expr += 1;
        }

        expr_tokens = expr_tokens[..i_end_expr].to_vec();
        
        // Parse these tokens
        let mut expr_parser = Self::new(expr_tokens.clone());
        expr_parser.run();

        self.n_token += expr_tokens.len() + 1;

        Element::Expression(expr_parser.parsed().clone())
    }

    fn when_function(&mut self) -> Element {
        let id = self.retrieve_id();
        
        let params = vec![]; // TODO : Parameters retrieving
        let return_type = self.retrieve_type_token();
        
        Element::Function(Function::new(id, params, return_type))
    }

    fn when_operation(&mut self) -> Element {        
        let operation = Element::Operation(Operation::new(
            self.tokenized[self.n_token - 1].clone(),
            self.tokenized[self.n_token - 2].clone(),
            self.tokenized[self.n_token].clone(),
        ));
        self.n_token += 1;
        operation
    }

    fn when_return(&mut self) -> Element {
        Element::Return(self.retrieve_value_or_expr())
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
            self.n_token += 1;
            let ret = self.tokenized[self.n_token].clone();
            if ret != Token::BracketOpen {
                self.n_token += 1; // skip value
            }
            ret
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

    let mut parser = Parser::new(tokenizer.tokenized().clone());
    parser.run();

    println!("{:#?}", parser.parsed());
}
