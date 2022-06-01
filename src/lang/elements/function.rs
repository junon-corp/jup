// This file is part of "jup"
// Under the MIT License
// Copyright (c) Junon, Antonin Hérault

use params::Params;
use type_::Type;

pub struct Function {
    id: Token,
    params: Params,
    return_type: Token,
}

impl Function {
    pub fn id(&self) -> String {
        self.id.to_string(),
    }

    pub fn params(&self) -> Params {
        self.params
    }

    pub fn return_type(&self) -> Type {
        Type::from_string(self.return_type_.to_string())
    }
}
