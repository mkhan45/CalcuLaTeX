use crate::expr::unit::Unit;

#[derive(Debug)]
pub enum UnitExpr {
    Atom(Unit, i8),
    Cons(UnitOp, Vec<UnitExpr>),
}

#[derive(Debug)]
pub enum UnitOp {
    Mul,
    Div,
}

impl UnitExpr {
    pub fn eval(&self) -> Unit {
        match self {
            UnitExpr::Atom(u, p) => u.clone(),
            UnitExpr::Cons(op, xs) => match (op, xs.as_slice()) {
                (UnitOp::Mul, [a, b]) => a.eval() * b.eval(),
                (UnitOp::Div, [a, b]) => a.eval() / b.eval(),
                _ => panic!(),
            },
        }
    }
}
