use crate::expr::unit::Unit;

#[derive(Debug, Clone)]
pub enum UnitExpr {
    Atom(Unit),
    Cons(UnitOp, Vec<UnitExpr>),
}

#[derive(Debug, Clone)]
pub enum UnitOp {
    Mul,
    Div,
    Exp(i8),
}

impl UnitExpr {
    pub fn eval(&self) -> Unit {
        match self {
            UnitExpr::Atom(u) => u.clone(),
            UnitExpr::Cons(op, xs) => match (op, xs.as_slice()) {
                (UnitOp::Mul, [a, b]) => a.eval() * b.eval(),
                (UnitOp::Div, [a, b]) => a.eval() / b.eval(),
                (UnitOp::Exp(p), [a]) => a.eval().pow(*p),
                _ => panic!(),
            },
        }
    }
}
