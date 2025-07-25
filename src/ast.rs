use std::fmt::Display;

use crate::prelude::*;

#[derive(Clone, Debug, PartialEq)]
pub enum Ast {
    BinOp(Operator, Box<Ast>, Box<Ast>),
    Value(Source, i32),
}

impl Default for Ast {
    fn default() -> Self {
        imm!(0)
    }
}

impl Display for Ast {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value(src, val) => write!(f, "{src}[{val}]"),
            BinOp(op, left, right) => write!(f, "({op} {left} {right})"),
        }
    }
}

impl Ast {
    pub fn binop(op: Operator, a: Self, b: Self) -> Self {
        Self::BinOp(op, Box::new(a), Box::new(b))
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Operator {
    Add,
    Sub,
    Mul,
    Div,
}

impl Display for Operator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Add => "+",
                Sub => "-",
                Mul => "*",
                Div => "/",
            }
        )
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Source {
    Arg,
    Imm,
}

impl Display for Source {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Arg => "arg",
                Imm => "imm",
            }
        )
    }
}
