pub mod val;

use crate::{function::eval_fn_call, parser::fn_call::FnCall};
use std::collections::BTreeMap;
use std::convert::TryFrom;

use val::*;

pub mod unit;
use unit::*;

pub mod unit_expr;

use crate::{error::CalcError, statement::Scope};

#[derive(Debug, Clone)]
pub enum Expr {
    Atom(Val),
    ParenExpr(Box<Expr>),
    Ident(String),
    FnCall(FnCall),
    Cons(Op, Vec<Expr>),
}

impl std::fmt::Display for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Expr::Atom(v) => write!(f, "{}", v),
            Expr::ParenExpr(e) => write!(f, "({:?})", e),
            Expr::Ident(n) => write!(f, "{}", n),
            Expr::FnCall(fc) => write!(f, "{:?}", fc),
            Expr::Cons(op, e) => write!(f, "({:?}, {:?})", op, e),
        }
    }
}

impl Expr {
    pub fn eval(&self, scope: &Scope) -> Result<Val, CalcError> {
        let e = |a: &Expr| a.eval(scope);
        Ok(match self {
            Expr::Atom(v) => v.clamp_num(),
            Expr::ParenExpr(ex) => e(ex)?,
            Expr::Ident(n) => {
                if let Some(v) = scope.variables.get(n) {
                    v.clone()
                } else {
                    (1.0, Unit::try_from(n)?).into()
                }
            }
            Expr::FnCall(fc) => eval_fn_call(fc, scope)?,
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

    pub fn remove_parens(&self) -> Self {
        if let Expr::ParenExpr(b) = self {
            *b.clone()
        } else {
            self.clone()
        }
    }

    pub fn resolve_aliases(&mut self, aliases: &BTreeMap<String, String>) {
        match self {
            Expr::Ident(n) => *n = aliases.get(n).unwrap_or(&n).to_string(),
            Expr::ParenExpr(e) => e.resolve_aliases(aliases),
            Expr::FnCall(FnCall { args, .. }) => {
                args.iter_mut().for_each(|e| e.resolve_aliases(aliases))
            }
            Expr::Cons(_, exprs) => exprs.iter_mut().for_each(|e| e.resolve_aliases(aliases)),
            _ => {}
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
    AddUnit(Unit, String),
}
