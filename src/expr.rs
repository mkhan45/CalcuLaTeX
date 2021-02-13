pub mod val;
use val::*;

pub mod unit;
use unit::*;

#[derive(Debug)]
pub enum Expr {
    Atom(Val),
    Cons(Op, Vec<Expr>),
}

impl std::fmt::Display for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Expr::Atom(v) => write!(f, "{}", v),
            Expr::Cons(op, e) => write!(f, "({:?}, {:?})", op, e),
        }
    }
}

impl Expr {
    pub fn eval(&self) -> Val {
        match self {
            Expr::Atom(v) => v.clone(),
            Expr::Cons(op, xs) => match (op, xs.as_slice()) {
                (Op::Plus, [a, b, ..]) => a.eval() + b.eval(),
                (Op::Minus, [a, b, ..]) => a.eval() - b.eval(),
                (Op::Mul, [a, b, ..]) => a.eval() * b.eval(),
                (Op::Div, [a, b, ..]) => a.eval() / b.eval(),
                (Op::AddUnit(u), [v, ..]) => v.eval().with_unit(&u),
                (Op::AddMultiUnit(pow, u), [v, ..]) => (v.eval()
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
