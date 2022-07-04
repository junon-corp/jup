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
            Token::SquareBracketOpen => self.when_array(),
            Token::Assembly => self.when_assembly(),
            Token::BracketOpen => self.when_expression(),
            Token::Function => self.when_function(),
            Token::ParenOpen => self.when_parameters(),
            Token::Variable => self.when_variable(),
            Token::Return => self.when_return(),
            Token::Plus | Token::Minus | Token::Multiply | Token::Divide 
                | Token::Assign => self.when_operation(),
            token => vec![Element::Other(token.clone())],
        }
    }

    /// Retrieves all tokens into `start_token` and `end_token` and skips sub 
    /// parts
    fn retrieve_token_into(&mut self, start_token: Token, end_token: Token) -> Vec<Token> {
        // Retrieves tokens into
        let mut tokens_into = self.tokenized[self.n_token..].to_vec();

        // Skips sub parts into to avoid finishing before it's really finished
        let mut i_end = 0;
        let mut is_sub_part = 0;

        for token in tokens_into.iter() {
            if token == &start_token {
                is_sub_part += 1;
            } else if token == &end_token {
                if is_sub_part == 0 {
                    break;
                }
                is_sub_part -= 1;
            }

            i_end += 1;
        }

        tokens_into[..i_end].to_vec()
    }

    /// Creates one `Element::Array` object with all the array's values tokens
    fn when_array(&mut self) -> Vec<Element> {
        let array_tokens = self.retrieve_token_into(
            Token::SquareBracketOpen, Token::SquareBracketClose
        );

        self.n_token += array_tokens.len() + 1;

        // Creates the array's values object
        let mut values: Vec<Token> = vec![];
        
        for token in array_tokens {
            if token == Token::Comma {
                continue;
            }
            values.push(token.clone());
        }
        
        vec![Element::Array(values)]
    }

    fn when_assembly(&mut self) -> Vec<Element> {
        let code: Token = self.tokenized[self.n_token].clone();
        self.n_token += 1;
        vec![Element::Assembly(code)]
    }

    /// Creates one `Element::Expression` object with a parsed the parsed 
    /// elements retrieved into the expression
    fn when_expression(&mut self) -> Vec<Element> {
        let expr_tokens = self.retrieve_token_into(
            Token::BracketOpen, Token::BracketClose
        );
        
        // Parse these tokens
        let mut expr_parser = Self::new(expr_tokens.clone());
        expr_parser.run();
        
        self.n_token += expr_tokens.len() + 1;

        vec![Element::Expression(expr_parser.parsed().clone())]
    }

    /// Creates one `Element::Function` object
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
                match arg2 {
                    Token::BracketOpen => {
                        self.n_token += 1;
                        ret_elements.extend(self.when_expression());
                    }
                    Token::SquareBracketOpen => {
                        self.n_token += 1;
                        ret_elements.extend(self.when_array());
                    }
                    _ => {}
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

    fn when_parameters(&mut self) -> Vec<Element> {
        let params = self.retrieve_token_into(
            Token::ParenOpen, 
            Token::ParenClose
        );
        self.n_token += params.len() + 1;
        
        vec![
            Element::Parameters(params)
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
            Token::BracketOpen | Token::SquareBracketOpen => next,
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
