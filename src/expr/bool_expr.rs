use std::collections::BTreeMap;
use std::convert::TryFrom;

use crate::{error::CalcError, statement::Scope};

#[derive(Debug, Clone)]
pub enum BoolExpr {
    ParenExpr(Box<BoolExpr>),
    Ident(String),
    Cons(BoolOp, Vec<BoolExpr>),
}

impl std::fmt::Display for BoolExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BoolExpr::ParenExpr(e) => write!(f, "({:?})", e),
            BoolExpr::Ident(n) => write!(f, "{}", n),
            BoolExpr::Cons(op, e) => write!(f, "({:?}, {:?})", op, e),
        }
    }
}

fn implies(a: bool, b: bool) -> bool {
    match (a, b) {
        (true, false) => false,
        (_, _) => true,
    }
}

impl BoolExpr {
    pub fn eval(&self, scope: &BTreeMap<String, bool>) -> Result<bool, CalcError> {
        let e = |a: &BoolExpr| a.eval(scope);
        Ok(match self {
            BoolExpr::ParenExpr(ex) => e(ex)?,
            BoolExpr::Ident(n) => {
                if let Some(v) = scope.get(n) {
                    v.clone()
                } else {
                    return Err(CalcError::Other(format!("Undefined boolean: {}", n)));
                }
            }
            BoolExpr::Cons(op, xs) => match (op, xs.as_slice()) {
                (BoolOp::Negate, [a]) => !e(a)?,
                (BoolOp::And, [a, b]) => e(a)? && e(b)?,
                (BoolOp::Or, [a, b]) => e(a)? || e(b)?,
                (BoolOp::Implies, [a, b]) => implies(e(a)?, e(b)?),
                (BoolOp::Equals, [a, b]) => e(a)? == e(b)?,
                _ => return Err(CalcError::MathError),
            },
        })
    }

    pub fn _remove_parens(&self) -> Self {
        if let BoolExpr::ParenExpr(b) = self {
            *b.clone()
        } else {
            self.clone()
        }
    }
}

#[derive(Debug, Clone)]
pub enum BoolOp {
    Negate,
    And,
    Or,
    Implies,
    Equals,
}
