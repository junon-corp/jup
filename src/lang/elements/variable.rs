// This file is part of "jup"
// Under the MIT License
// Copyright (c) Junon, Antonin Hérault

use crate::lang::tokens::Token;
use super::type_::Type;

#[derive(Debug, Clone)]
pub struct Variable {
    id: Token,
    type_: Type,
    value: Token,
    stack_pos: usize,
}

impl Variable {
    pub fn new(id: Token, type_: Type, value: Token) -> Self {
        Self {
            id,
            type_,
            value,
            stack_pos: 0, // changed by the compiler
        }
    }

    pub fn set_value(&mut self, value: String) {
        self.value = Token::from_string(value.as_str());
    }

    pub fn set_stack_pos(&mut self, stack_pos: usize) {
        self.stack_pos = stack_pos;
    }
    
    pub fn id(&self) -> String {
        self.id.to_string()
    }

    pub fn type_(&self) -> &Type {
        &self.type_
    }

    /// When the variable is not initialized, it's initialized as `0`
    /// If the value is in fact an expression, the default return register for
    /// any expression is given as "value"
    pub fn value(&self) -> String {
        match &self.value {
            Token::BracketOpen => "rbx".to_string(), // Todo : Create a defaults repo
            Token::None => "0".to_string(),
            tok_value => tok_value.to_string(),
        }
    }

    pub fn stack_pos(&self) -> usize {
        self.stack_pos
    }
}
