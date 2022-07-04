// This file is part of "jup"
// Under the MIT License
// Copyright (c) Junon, Antonin Hérault

use crate::lang::tokens::Token;
use super::{
    Element,
    type_::Type
};

#[derive(Debug, Clone)]
pub struct Function {
    id: Token,
    /// This element has to be `Element::Parameters`
    params: Box<Element>,
    return_type: Type,
}

impl Function {
    pub fn new(id: Token, params: Element, return_type: Type) -> Self {
        Function {
            id,
            params: Box::new(params),
            return_type,
        }
    }

    pub fn id(&self) -> String {
        self.id.to_string()
    }

    pub fn params(&self) -> Element {
        *self.params.clone()
    }

    pub fn return_type(&self) -> &Type {
        &self.return_type
    }
}
