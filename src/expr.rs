pub mod val;
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
            Expr::Ident(n) => scope.variables.get(n).unwrap().clone(),
            Expr::Cons(op, xs) => match (op, xs.as_slice()) {
                (Op::Plus, [a, b, ..]) => e(a) + e(b),
                (Op::Minus, [a, b, ..]) => e(a) - e(b),
                (Op::Mul, [a, b, ..]) => e(a) * e(b),
                (Op::Div, [a, b, ..]) => e(a) / e(b),
                (Op::AddUnit(u), [v, ..]) => e(v).with_unit(&u),
                (Op::AddMultiUnit(pow, u), [v, ..]) => (e(v)
                    * Val {
                        num: 10f64.powi(*pow as i32),
                        unit: Unit::empty(),
                    })
                .with_unit(&u),
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
    AddUnit(Unit),
    AddMultiUnit(i8, Unit),
}
