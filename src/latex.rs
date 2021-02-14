use crate::expr::unit::BASE_UNITS;
use crate::expr::{unit::Unit, val::Val, Expr, Op};
use num::rational::Ratio;
use num::One;
use num::Zero;

pub enum LaTeX {
    Text(String),
    Math(String),
}

pub trait ToLaTeX {
    fn to_latex(&self) -> LaTeX;
}

impl ToString for LaTeX {
    fn to_string(&self) -> String {
        match self {
            LaTeX::Text(t) => t.to_owned(),
            LaTeX::Math(m) => m.to_string(),
        }
    }
}

impl ToLaTeX for Expr {
    fn to_latex(&self) -> LaTeX {
        match self {
            Expr::Atom(v) => LaTeX::Math(v.to_string()),
            Expr::Ident(n) => LaTeX::Math(n.to_string()),
            Expr::Cons(op, e) => match (op, e.as_slice()) {
                (Op::Plus, [a, b, ..]) => LaTeX::Math(format!(
                    "({} + {})",
                    a.to_latex().to_string(),
                    b.to_latex().to_string()
                )),
                (Op::Minus, [a, b, ..]) => LaTeX::Math(format!(
                    "({} - {})",
                    a.to_latex().to_string(),
                    b.to_latex().to_string()
                )),
                (Op::Mul, [a, b, ..]) => LaTeX::Math(format!(
                    "{} \\times {}",
                    a.to_latex().to_string(),
                    b.to_latex().to_string()
                )),
                (Op::Div, [a, b, ..]) => LaTeX::Math(format!(
                    "\\frac{{{}}}{{{}}}",
                    a.to_latex().to_string(),
                    b.to_latex().to_string()
                )),
                (Op::AddUnit(u), [v]) => LaTeX::Math(format!(
                    "{} {}",
                    v.to_latex().to_string(),
                    u.to_latex().to_string()
                )),
                (Op::AddMultiUnit(p, u), [v]) => LaTeX::Math(format!(
                    "{} {} \\times 10^{{{}}}",
                    v.to_latex().to_string(),
                    u.to_latex().to_string(),
                    p.to_string(),
                )),
                _ => todo!(),
            },
        }
    }
}

impl ToLaTeX for Val {
    fn to_latex(&self) -> LaTeX {
        let unit_str = self.unit.to_latex().to_string();
        let out = if !unit_str.is_empty() {
            format!("{} \\ {}", self.num, unit_str)
        } else {
            self.num.to_string()
        };
        LaTeX::Math(out.trim().to_string())
    }
}

impl ToLaTeX for Unit {
    fn to_latex(&self) -> LaTeX {
        match self {
            Unit::Base(arr) => {
                let res =
                    arr.iter()
                        .zip(BASE_UNITS.iter())
                        .fold("".to_string(), |acc, (pow, unit)| match pow {
                            r if r == &Ratio::zero() => acc,
                            r if r == &Ratio::one() => format!("{} {}", acc, unit.to_string()),
                            _ => format!("{} {}^{{{}}}", acc, unit.to_string(), pow),
                        });
                LaTeX::Math(res.trim().to_string())
            }
            Unit::Custom(_map) => {
                todo!()
            }
        }
    }
}
