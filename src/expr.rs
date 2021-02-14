pub mod val;
use val::*;

pub mod unit;
use unit::*;

#[derive(Debug)]
pub enum Expr {
    Atom(Val),
    Cons(Op, Vec<Expr>),
    Empty,
}

impl std::fmt::Display for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Expr::Atom(v) => write!(f, "{}", v),
            Expr::Cons(op, e) => write!(f, "({:?}, {:?})", op, e),
            Expr::Empty => Ok(()),
        }
    }
}

impl Expr {
    pub fn eval(&self) -> Val {
        todo!();
    }
}

#[derive(Debug)]
pub enum Op {
    Plus,
    Minus,
    AddUnit(Unit),
}
