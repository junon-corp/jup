// This file is part of "jup"
// Under the MIT License
// Copyright (c) Junon, Antonin Hérault

pub mod function;
pub mod variable;
pub mod params;
pub mod type_;

use function::Function;
use variable::Variable;
use params::Params;
use type_::Type;

/// Language's element that parsed will create from the tokens
pub enum Element {
    Function(Function),
    Variable(Variable),
    Params(Params),
    Type(Type),
}
