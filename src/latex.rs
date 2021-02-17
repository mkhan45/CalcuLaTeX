use crate::expr::unit_expr::{UnitExpr, UnitOp};
use std::convert::TryFrom;

use crate::expr::unit::BASE_UNITS;
use crate::expr::unit::UNIT_PREFIXES_ABBR;
use crate::expr::unit::{BaseUnit, UnitDesc};
use crate::expr::{unit::Unit, val::Val, Expr, Op};
use num::rational::Ratio;
use num::One;
use num::Signed;
use num::Zero;

pub enum LaTeX {
    Text(String),
    Math(String),
}

#[derive(Debug)]
pub enum FormatArgs {
    UnitHint { string: String, value: Unit },
}

pub trait ToLaTeX {
    fn to_latex_ext(&self, args: Option<&FormatArgs>) -> LaTeX;
    fn to_latex(&self) -> LaTeX {
        self.to_latex_ext(None)
    }
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
    fn to_latex_ext(&self, _: Option<&FormatArgs>) -> LaTeX {
        match self {
            Expr::Atom(v) => LaTeX::Math(v.to_latex().to_string()),
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
                (Op::Exp, [a, b, ..]) => LaTeX::Math(format!(
                    "{}^{{{}}}",
                    a.to_latex().to_string(),
                    b.to_latex().to_string()
                )),
                (Op::AddUnit(_, s), [v]) => {
                    LaTeX::Math(format!("{}\\ {}", (v).to_latex().to_string(), s,))
                }
                _ => todo!(),
            },
        }
    }
}

impl ToLaTeX for UnitExpr {
    fn to_latex_ext(&self, _: Option<&FormatArgs>) -> LaTeX {
        match self {
            UnitExpr::Atom(u) => LaTeX::Math(u.to_latex().to_string()),
            UnitExpr::Cons(op, e) => match (op, e.as_slice()) {
                (UnitOp::Mul, [a, b, ..]) => LaTeX::Math(format!(
                    "{} \\ {}",
                    a.to_latex().to_string(),
                    b.to_latex().to_string()
                )),
                (UnitOp::Div, [a, b, ..]) => LaTeX::Math(format!(
                    "\\frac{{{}}}{{{}}}",
                    a.to_latex().to_string(),
                    b.to_latex().to_string()
                )),
                (UnitOp::Exp(e), [a, ..]) => LaTeX::Math(format!(
                    "{}^{{{}}}",
                    a.to_latex().to_string(),
                    e.to_string()
                )),
                _ => todo!(),
            },
        }
    }
}

impl ToLaTeX for Val {
    fn to_latex_ext(&self, args: Option<&FormatArgs>) -> LaTeX {
        match args {
            Some(FormatArgs::UnitHint {
                string,
                value: Unit { desc, exp, mult },
            }) if desc == &self.unit.desc => {
                let out = format!(
                    "{} \\ {}",
                    (&self.num
                        / rug::Rational::try_from(10f64.powi((*exp - self.unit.exp) as i32))
                            .unwrap()
                        / rug::Rational::from(mult / &self.unit.mult))
                    .to_f64(),
                    string
                );
                LaTeX::Math(out.trim().to_string())
            }
            Some(FormatArgs::UnitHint { string, .. }) => {
                panic!(
                    "Unit hint {} does not match value with unit {}",
                    string, self.unit
                )
            }
            None => {
                let unit_str = self.unit.to_latex().to_string();
                let num = rug::Rational::from(&self.num * &self.unit.mult)
                    * &rug::Rational::try_from(10f64.powi(self.unit.exp as i32)).unwrap();
                let out = if !unit_str.is_empty() {
                    format!("{} \\ {}", num.to_f64(), unit_str)
                } else {
                    num.to_f64().to_string()
                };
                LaTeX::Math(out.trim().to_string())
            }
        }
    }
}

impl ToLaTeX for Unit {
    fn to_latex_ext(&self, _: Option<&FormatArgs>) -> LaTeX {
        match self.desc.clone() {
            UnitDesc::Base(arr) => {
                let mut numerator = Vec::new();
                let mut denominator = Vec::new();
                arr.iter()
                    .rev()
                    .zip(BASE_UNITS.iter().rev())
                    .for_each(|(pow, unit)| {
                        use std::cmp::Ordering::*;

                        match pow.cmp(&Ratio::zero()) {
                            Greater => numerator.push((pow, unit)),
                            Less => denominator.push((pow, unit)),
                            _ => {}
                        }
                    });

                let latexify_single_unit = |(pow, unit): &(&Ratio<i8>, &BaseUnit)| {
                    if pow.abs() == Ratio::one() {
                        unit.to_string()
                    } else {
                        format!("{}^{{{}}}", unit.to_string(), pow.abs())
                    }
                };

                let numerator_string = numerator.iter().fold("".to_string(), |acc, unit_info| {
                    format!("{} {}\\,", acc, latexify_single_unit(unit_info))
                });
                let denominator_string =
                    denominator.iter().fold("".to_string(), |acc, unit_info| {
                        format!("{} {}\\,", acc, latexify_single_unit(unit_info))
                    });

                if numerator_string.is_empty() && denominator_string.is_empty() {
                    LaTeX::Math("".to_string())
                } else if numerator_string.is_empty() {
                    LaTeX::Math(format!(
                        "\\frac{{1}}{{{}{}}}",
                        UNIT_PREFIXES_ABBR.get_by_right(&self.exp).unwrap(),
                        denominator_string
                    ))
                } else if denominator.is_empty() {
                    LaTeX::Math(format!(
                        "{}{}",
                        UNIT_PREFIXES_ABBR.get_by_right(&self.exp).unwrap(),
                        numerator_string
                    ))
                } else {
                    LaTeX::Math(format!(
                        "\\frac{{{}{}}}{{{}}}",
                        UNIT_PREFIXES_ABBR.get_by_right(&self.exp).unwrap(),
                        numerator_string,
                        denominator_string
                    ))
                }
            }
            UnitDesc::Custom(_map) => {
                todo!()
            }
        }
    }
}
