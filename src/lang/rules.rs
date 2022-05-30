// This file is part of "jup"
// Under the MIT License
// Copyright (c) Junon, Antonin Hérault

use crate::lang::tokens::Token;

#[derive(Debug)]
pub enum RuleItem {
    /// Used as the rule's operator, the main token of the expression
    Main(Token),

    Value,
    Label,
    Expression,
    Token(Token),

    /// Tokens to be skipped because checked by another way
    ///
    /// Examples : an expression, an arguments list
    Skip,
}

macro_rules! rule {
    () => { Rule { items: vec![] }};
    ($x:expr $(, $more:expr)*) => {
        Rule { 0: vec![$($more), *] }
    }
}

/// When the rule contains no `RuleItem`, there is no rule available
#[repr(transparent)]
#[derive(Debug)]
pub struct Rule(Vec<RuleItem>);

impl Rule {
    /// Make a series of possibilities for an operator
    ///
    /// Example : "5 + 5" where the rule is made for "+"
    fn from_operator(op: &Token) -> Vec<Self> {
        vec![
            rule!(RuleItem::Value, RuleItem::Main(op.clone()), RuleItem::Value),
            rule!(RuleItem::Value, RuleItem::Main(op.clone()), RuleItem::Label),
            rule!(
                RuleItem::Value,
                RuleItem::Main(op.clone()),
                RuleItem::Expression
            ),
            rule!(RuleItem::Label, RuleItem::Main(op.clone()), RuleItem::Value),
            rule!(RuleItem::Label, RuleItem::Main(op.clone()), RuleItem::Label),
            rule!(
                RuleItem::Label,
                RuleItem::Main(op.clone()),
                RuleItem::Expression
            ),
            rule!(
                RuleItem::Expression,
                RuleItem::Main(op.clone()),
                RuleItem::Value
            ),
            rule!(
                RuleItem::Expression,
                RuleItem::Main(op.clone()),
                RuleItem::Label
            ),
            rule!(
                RuleItem::Expression,
                RuleItem::Main(op.clone()),
                RuleItem::Expression
            ),
        ]
    }

    fn from_basic_scheme(first: &Token) -> Vec<Self> {
        vec![
            rule!(RuleItem::Main(first.clone()), RuleItem::Value),
            rule!(RuleItem::Main(first.clone()), RuleItem::Label),
            rule!(RuleItem::Main(first.clone()), RuleItem::Expression),
        ]
    }

    /// Gets a list of rules to be used with a token
    pub fn from_token(token: Token) -> Vec<Self> {
        match token {
            Token::Assembly => {
                vec![
                    rule!(RuleItem::Main(Token::Assembly)),
                    rule!(RuleItem::Main(Token::Assembly), RuleItem::Value),
                ]
            }
            Token::Assign => {
                vec![
                    rule!(
                        RuleItem::Label,
                        RuleItem::Main(Token::Assign),
                        RuleItem::Value
                    ),
                    rule!(
                        RuleItem::Label,
                        RuleItem::Main(Token::Assign),
                        RuleItem::Label
                    ),
                    rule!(
                        RuleItem::Label,
                        RuleItem::Main(Token::Assign),
                        RuleItem::Expression
                    ),
                ]
            }
            Token::Divide => Self::from_operator(&Token::Divide),
            Token::Function => {
                vec![
                    rule!(RuleItem::Main(Token::Function), RuleItem::Label),
                    rule!(
                        RuleItem::Main(Token::Function),
                        RuleItem::Label,
                        RuleItem::Token(Token::ParenOpen),
                        RuleItem::Skip,
                        RuleItem::Token(Token::ParenClose)
                    ),
                ]
            }
            Token::Minus => Self::from_operator(&Token::Minus),
            Token::Multiply => Self::from_operator(&Token::Multiply),
            Token::Point => {
                vec![rule!(
                    RuleItem::Label,
                    RuleItem::Main(Token::Point),
                    RuleItem::Label
                )]
            }
            Token::Plus => Self::from_operator(&Token::Plus),
            Token::Return => Self::from_basic_scheme(&Token::Return),
            Token::Static => {
                vec![
                    rule!(
                        RuleItem::Main(Token::Static),
                        RuleItem::Label,
                        RuleItem::Token(Token::TypeDef),
                        RuleItem::Label
                    ),
                    rule!(
                        RuleItem::Main(Token::Static),
                        RuleItem::Label,
                        RuleItem::Token(Token::TypeDef),
                        RuleItem::Label,
                        RuleItem::Token(Token::Assign),
                        RuleItem::Value
                    ),
                    rule!(
                        RuleItem::Main(Token::Static),
                        RuleItem::Label,
                        RuleItem::Token(Token::TypeDef),
                        RuleItem::Label,
                        RuleItem::Token(Token::Assign),
                        RuleItem::Label
                    ),
                    rule!(
                        RuleItem::Main(Token::Static),
                        RuleItem::Label,
                        RuleItem::Token(Token::TypeDef),
                        RuleItem::Label,
                        RuleItem::Token(Token::Assign),
                        RuleItem::Expression
                    ),
                ]
            }
            Token::TypeDef => {
                vec![rule!(RuleItem::Main(Token::TypeDef), RuleItem::Label)]
            }
            Token::Variable => {
                vec![
                    rule!(
                        RuleItem::Main(Token::Variable),
                        RuleItem::Label,
                        RuleItem::Token(Token::TypeDef),
                        RuleItem::Label
                    ),
                    rule!(
                        RuleItem::Main(Token::Variable),
                        RuleItem::Label,
                        RuleItem::Token(Token::TypeDef),
                        RuleItem::Label,
                        RuleItem::Token(Token::Assign),
                        RuleItem::Value
                    ),
                    rule!(
                        RuleItem::Main(Token::Variable),
                        RuleItem::Label,
                        RuleItem::Token(Token::TypeDef),
                        RuleItem::Label,
                        RuleItem::Token(Token::Assign),
                        RuleItem::Label
                    ),
                    rule!(
                        RuleItem::Main(Token::Variable),
                        RuleItem::Label,
                        RuleItem::Token(Token::TypeDef),
                        RuleItem::Label,
                        RuleItem::Token(Token::Assign),
                        RuleItem::Expression
                    ),
                ]
            }

            // System calls
            Token::Print => Self::from_basic_scheme(&Token::Print),
            Token::Exit => Self::from_basic_scheme(&Token::Exit),

            _ => vec![],
        }
    }
}
