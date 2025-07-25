#![allow(unused_imports)]

pub use crate::{
    ast::{
        Ast,
        Ast::{BinOp, Value},
        Operator,
        Operator::{Add, Div, Mul, Sub},
        Source,
        Source::{Arg, Imm},
    },
    parser::{
        Token,
        Token::{
            ClosedBracket, ClosedParen, Dash, Num, OpenBracket, OpenParen, Plus, Slash, Star, Var,
        },
    },
};

#[macro_export]
macro_rules! arg {
    ( $n:expr ) => {
        Value(Arg, $n)
    };
}

#[macro_export]
macro_rules! imm {
    ( $n:expr ) => {
        Value(Imm, $n)
    };
}

#[macro_export]
macro_rules! binop {
    ( + , $a:expr, $b:expr $(,)? ) => {
        Ast::binop(Add, $a, $b)
    };
    ( - , $a:expr, $b:expr $(,)? ) => {
        Ast::binop(Sub, $a, $b)
    };
    ( * , $a:expr, $b:expr $(,)? ) => {
        Ast::binop(Mul, $a, $b)
    };
    ( / , $a:expr, $b:expr $(,)? ) => {
        Ast::binop(Div, $a, $b)
    };
}

#[macro_export]
macro_rules! assert_simulate {
    ( $prog:expr, [ $( $arg:literal ),+ ], $result:expr ) => {
        assert_eq! ( ArchSim::simulate($prog, vec![ $( $arg ),+ ]), $result );
    };
    ( $prog:expr, [ $( $arg:literal ),+ ], $result:expr, $fmt:expr $(, $( $fmtarg:tt ),*)? ) => {
        assert_eq! ( ArchSim::simulate($prog, vec![ $( $arg ),+ ]), $result, $fmt $(, $($fmtarg),*)? );
    };
}

pub use arg;
pub use assert_simulate;
pub use binop;
pub use imm;
