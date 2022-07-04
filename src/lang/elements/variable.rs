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

    pub fn set_id(&mut self, id: Token) {
        self.id = id;
    }

    pub fn set_type(&mut self, type_: Type) {
        self.type_ = type_;
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

    pub fn value(&self) -> &Token {
        &self.value
    }

    pub fn stack_pos(&self) -> usize {
        self.stack_pos
    }
}
