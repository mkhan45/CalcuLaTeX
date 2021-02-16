pub mod val;
use std::convert::TryInto;

use val::*;

pub mod unit;
use unit::*;

use crate::statement::Scope;

#[derive(Debug)]
pub enum Expr {
    Atom(Val),
    Ident(String),
    Cons(Op, Vec<Expr>),
}

impl std::fmt::Display for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Expr::Atom(v) => write!(f, "{}", v),
            Expr::Ident(n) => write!(f, "{}", n),
            Expr::Cons(op, e) => write!(f, "({:?}, {:?})", op, e),
        }
    }
}

impl Expr {
    pub fn eval(&self, scope: &Scope) -> Val {
        let e = |a: &Expr| a.eval(scope);
        match self {
            Expr::Atom(v) => v.clone(),
            Expr::Ident(n) => {
                if let Some(v) = scope.variables.get(n) {
                    v.clone()
                } else {
                    Val {
                        num: 1.0,
                        unit: n.as_str().try_into().unwrap(),
                    }
                }
            }
            Expr::Cons(op, xs) => match (op, xs.as_slice()) {
                (Op::Plus, [a, b]) => e(a) + e(b),
                (Op::Minus, [a, b]) => e(a) - e(b),
                (Op::Mul, [a, b]) => e(a) * e(b),
                (Op::Div, [a, b]) => e(a) / e(b),
                (Op::Exp, [a, b]) => e(a).pow(&e(b)),
                (Op::AddUnit(u), [v]) => e(v).with_unit(&u),
                _ => todo!(),
            },
        }
    }
}

#[derive(Debug, Clone)]
pub enum Op {
    Plus,
    Minus,
    Mul,
    Div,
    Exp,
    AddUnit(Unit),
}
