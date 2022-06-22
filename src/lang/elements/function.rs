// This file is part of "jup"
// Under the MIT License
// Copyright (c) Junon, Antonin Hérault

use crate::lang::tokens::Token;
use super::params::Params;
use super::type_::Type;

#[derive(Debug, Clone)]
pub struct Function {
    id: Token,
    params: Params,
    return_type: Type,
}

impl Function {
    pub fn new(id: Token, params: Params, return_type: Type) -> Self {
        Function {
            id,
            params,
            return_type,
        }
    }

    pub fn id(&self) -> String {
        self.id.to_string()
    }

    pub fn params(&self) -> Params {
        self.params.clone()
    }

    pub fn return_type(&self) -> &Type {
        &self.return_type
    }
}
