// This file is part of "jup"
// Under the MIT License
// Copyright (c) Junon, Antonin Hérault

use crate::lang::tokens::Token;
use crate::lang::elements::{
    Element, 
    function::Function, 
    operation::Operation,
    type_::Type,
    variable::Variable
};

/// Transforms tokens to a collection of `Element` to be easily used by the 
/// compiler
pub struct Parser {
    tokenized: Vec<Token>,
    parsed: Vec<Element>,
    n_token: usize,
    asked_for_push: Option<Element>,
}

impl Parser {
    /// Takes a tokenized file thanks to `Tokenizer` to parse these tokens
    pub fn new(tokenized: Vec<Token>) -> Self {
        Parser {
            tokenized,
            parsed: vec![],
            n_token: 0,
            asked_for_push: None,
        }
    }

    pub fn run(&mut self) {
        for (i, token) in self.tokenized.clone().iter().enumerate() {
            if i != self.n_token {
                continue;
            }

            self.n_token = i;
            let elements = self.check();
            
            for element in elements {
                self.parsed.push(element);
            }
        }
    }

    fn check(&mut self) -> Vec<Element> {
        self.n_token += 1;

        match &self.tokenized[self.n_token -1] {
            Token::BracketOpen => self.when_expression(),
            Token::Function => self.when_function(),
            Token::Variable => self.when_variable(),
            Token::Return => self.when_return(),
            Token::Plus | Token::Minus | Token::Multiply | Token::Divide 
                | Token::Assign => self.when_operation(),
            token => vec![Element::Other(token.clone())],
        }
    }

    fn when_expression(&mut self) -> Vec<Element> {
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

        vec![Element::Expression(expr_parser.parsed().clone())]
    }

    fn when_function(&mut self) -> Vec<Element> {
        let id = self.retrieve_id();
        
        let params = vec![]; // Todo : Parameters retrieving
        let return_type = self.retrieve_type_token();
        
        vec![Element::Function(Function::new(id, params, return_type))]
    }

    fn when_operation(&mut self) -> Vec<Element> {   
        self.parsed.pop(); // arg1 has to be only integrated in the operation 

        let mut ret_elements: Vec<Element> = vec![];

        let operation = Element::Operation(Operation::new(
            // Operator
            self.tokenized[self.n_token - 1].clone(),
            // Argument 1
            self.tokenized[self.n_token - 2].clone(),        
            // Argument 2
            {
                let arg2 = self.tokenized[self.n_token].clone();
                if arg2 == Token::BracketOpen {
                    self.n_token += 1;
                    ret_elements.extend(self.when_expression());
                }
                arg2
            }
        ));

        self.n_token += 1;
        ret_elements.insert(0, operation);
        ret_elements
    }

    fn when_return(&mut self) -> Vec<Element> {
        vec![
            Element::Return(self.retrieve_value_or_expr())
        ]
    }

    fn when_variable(&mut self) -> Vec<Element> {
        vec![
            Element::Variable(Variable::new(
                self.retrieve_id(),
                self.retrieve_type_token(),
                if self.tokenized[self.n_token] == Token::Assign {
                    self.n_token += 1;
                    self.retrieve_value_or_expr()
                } else {
                    Token::None
                }            
            ))
        ]
    }

    fn retrieve_id(&mut self) -> Token {
        self.n_token += 1; // skip id
        self.tokenized[self.n_token -1].clone()
    }

    fn retrieve_type_token(&mut self) -> Type {
        // When the type is explicitly written
        if self.tokenized[self.n_token] == Token::TypeDef {
            self.n_token += 2; // skip Token::TypeDef and type
            
            let type_token = self.tokenized[self.n_token -1].clone();
            
            // Array type found
            if self.tokenized[self.n_token] == Token::SquareBracketOpen {
                // Token::SquareBracketOpen, ::Other and ::SquareBracketClose
                self.n_token += 3;

                let array_size = self.tokenized[self.n_token -2]
                    .clone()
                    .to_string()
                    .parse::<usize>()
                    .unwrap();
                
                Type::array_from_string(type_token.to_string(), array_size)
            } else {
                Type::from_string(type_token.to_string())
            }
        } else {
            Type::None
        }
    }

    /// - When `Token::BracketOpen` is returned, we know it's an expression
    /// - When `Token::Other(...)` is returned, we know it's a value
    /// - When `Token::None` is returned it's because there is no value or expr
    fn retrieve_value_or_expr(&mut self) -> Token {
        let next = self.tokenized[self.n_token].clone();
        match next {
            Token::BracketOpen => {
                next
            },
            Token::Other(_) => {
                self.n_token += 1;
                next
            }
            _ => Token::None,
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
