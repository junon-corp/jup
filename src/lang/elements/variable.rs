// This file is part of "jup"
// Under the MIT License
// Copyright (c) Junon, Antonin Hérault

use crate::lang::tokens::Token;
use type_::Type;

#[derive(Debug, Clone)]
pub struct Variable {
    id: Token,
    type_: Token,
    value: Token,
}

impl Variable {
    pub fn id(&self) -> String {
        self.id.to_string()
    }

    pub fn type_(&self) -> Type {
        Type::from_string(self.type_.to_string())
    }

    /// When the variable is not initialized, it's initialized as `0`
    /// If the value is in fact an expression, the default return register for
    /// any expression is given as "value"
    pub fn value(&self) -> String {
        match self.value {
            Token::BracketOpen => "rbx", // TODO : Create a defaults repo
            Token::None => "0",
            _ => value,
        }.to_string(),
    }
}
