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
    Exp(i8),
}

#[derive(Clone, Debug)]
pub struct UnitPow {
    pub unit: Unit,
    pub pow: i8,
}

impl std::ops::Mul for UnitPow {
    type Output = UnitPow;

    fn mul(self, rhs: Self) -> Self::Output {
        UnitPow {
            unit: self.unit.clone() * rhs.unit,
            pow: self.pow,
        }
    }
}

impl std::ops::Div for UnitPow {
    type Output = UnitPow;

    fn div(self, rhs: Self) -> Self::Output {
        UnitPow {
            unit: self.unit.clone() / rhs.unit,
            pow: self.pow,
        }
    }
}

impl UnitPow {
    pub fn pow(self, rhs: i8) -> Self {
        UnitPow {
            unit: self.unit.pow(rhs),
            pow: self.pow,
        }
    }
}

impl UnitExpr {
    pub fn eval(&self) -> UnitPow {
        match self {
            UnitExpr::Atom(u, p) => UnitPow {
                unit: u.clone(),
                pow: *p,
            },
            UnitExpr::Cons(op, xs) => match (op, xs.as_slice()) {
                (UnitOp::Mul, [a, b]) => a.eval() * b.eval(),
                (UnitOp::Div, [a, b]) => a.eval() / b.eval(),
                (UnitOp::Exp(p), [a]) => a.eval().pow(*p),
                _ => panic!(),
            },
        }
    }
}
