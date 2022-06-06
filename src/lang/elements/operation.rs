// This file is part of "jup"
// Under the MIT License
// Copyright (c) Junon, Antonin Hérault

use crate::lang::tokens::Token;

#[derive(Debug, Clone)]
pub struct Operation {
    operator: Token,
    arg1: Token,
    arg2: Token,
}

impl Operation {
    pub fn new(operator: Token, arg1: Token, arg2: Token) -> Self {
        Self {
            operator,
            arg1,
            arg2,
        }
    }

    pub fn operator(&self) -> &Token {
        &self.operator
    }

    pub fn arg1(&self) -> &Token {
        &self.arg1
    }

    pub fn arg2(&self) -> &Token {
        &self.arg2
    }
}
