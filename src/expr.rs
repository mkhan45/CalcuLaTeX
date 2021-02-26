pub mod val;

use std::convert::TryFrom;

use val::*;

pub mod unit;
use unit::*;

pub mod unit_expr;

use crate::{error::CalcError, statement::Scope};

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
    pub fn eval(&self, scope: &Scope) -> Result<Val, CalcError> {
        let e = |a: &Expr| a.eval(scope);
        Ok(match self {
            Expr::Atom(v) => v.clamp_num(),
            Expr::Ident(n) => {
                if let Some(v) = scope.variables.get(n) {
                    v.clone()
                } else {
                    (1.0, Unit::try_from(n)?).into()
                }
            }
            Expr::Cons(op, xs) => match (op, xs.as_slice()) {
                (Op::Plus, [a, b]) => (e(a)? + e(b)?)?,
                (Op::Minus, [a, b]) => (e(a)? - e(b)?)?,
                (Op::Mul, [a, b]) => e(a)? * e(b)?,
                (Op::Div, [a, b]) => e(a)? / e(b)?,
                (Op::Exp, [a, b]) => e(a)?.pow(&e(b)?),
                (Op::AddUnit(u, _), [v]) => e(v)?.with_unit(&u),
                _ => return Err(CalcError::MathError),
            },
        })
    }
}

#[derive(Debug, Clone)]
pub enum Op {
    Plus,
    Minus,
    Mul,
    Div,
    Exp,
    AddUnit(Unit, String),
}
