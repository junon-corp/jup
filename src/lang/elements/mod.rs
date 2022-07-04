// This file is part of "jup"
// Under the MIT License
// Copyright (c) Junon, Antonin Hérault

pub mod function;
pub mod operation;
pub mod type_;
pub mod variable;

use crate::lang::tokens::Token;

use function::Function;
use operation::Operation;
use type_::Type;
use variable::Variable;

/// Language's element that parsed will create from the tokens
#[derive(Debug, Clone)]
pub enum Element {
    Array(Vec<Token>),
    Assembly(Token),
    Expression(Vec<Element>),
    Operation(Operation),
    Function(Function),
    Parameters(Vec<Element>),
    Return(Token),
    Variable(Variable),
    Other(Token),
}
