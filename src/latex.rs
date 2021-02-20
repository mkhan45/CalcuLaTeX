use crate::{
    expr::unit_expr::{UnitExpr, UnitOp},
    parser::naive_string::StringExpr,
};

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

#[derive(Debug, Clone)]
pub struct UnitHint {
    pub unit: Unit,
    pub pretty_string: StringExpr,
}

pub struct FormatArgs {
    pub unit_hint: Option<UnitHint>,
}

impl Default for FormatArgs {
    fn default() -> Self {
        FormatArgs { unit_hint: None }
    }
}

pub trait ToLaTeX {
    fn to_latex_ext(&self, args: &FormatArgs) -> LaTeX;
    fn to_latex(&self) -> LaTeX {
        self.to_latex_ext(&FormatArgs::default())
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
    fn to_latex_ext(&self, _: &FormatArgs) -> LaTeX {
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
                    LaTeX::Math(format!("{}\\ {}", v.to_latex().to_string(), s))
                }
                _ => todo!(),
            },
        }
    }
}

impl ToLaTeX for UnitExpr {
    fn to_latex_ext(&self, _: &FormatArgs) -> LaTeX {
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
    fn to_latex_ext(&self, args: &FormatArgs) -> LaTeX {
        match &args.unit_hint {
            Some(UnitHint {
                unit,
                pretty_string,
            }) if unit.desc == self.unit.desc => {
                let out = format!(
                    "{} \\ {}",
                    (self.num
                        / 10f64.powi((unit.exp - self.unit.exp) as i32)
                        / (unit.mult / self.unit.mult)),
                    pretty_string.to_latex().to_string()
                );
                LaTeX::Math(out.trim().to_string())
            }
            Some(UnitHint { unit, .. }) => {
                panic!(
                    "Unit hint {} does not match value with unit {}",
                    unit.to_string(),
                    self.unit
                )
            }
            None => {
                let unit_str = self.unit.to_latex().to_string();
                let out = if !unit_str.is_empty() {
                    format!("{} \\ {}", self.num, unit_str)
                } else {
                    self.num.to_string()
                };
                LaTeX::Math(out.trim().to_string())
            }
        }
    }
}

impl ToLaTeX for Unit {
    fn to_latex_ext(&self, _: &FormatArgs) -> LaTeX {
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
                    if let Some(prefix) = UNIT_PREFIXES_ABBR.get_by_right(&self.exp) {
                        LaTeX::Math(format!("\\frac{{1}}{{{}{}}}", prefix, denominator_string))
                    } else {
                        LaTeX::Math(format!(
                            "\\frac{{1}}{{{}\\times 10^{{{}}}}}",
                            denominator_string, self.exp
                        ))
                    }
                } else if denominator.is_empty() {
                    if let Some(prefix) = UNIT_PREFIXES_ABBR.get_by_right(&self.exp) {
                        LaTeX::Math(format!("{}{}", prefix, numerator_string))
                    } else {
                        LaTeX::Math(format!("{}\\times 10^{{{}}}", numerator_string, self.exp))
                    }
                } else {
                    if let Some(prefix) = UNIT_PREFIXES_ABBR.get_by_right(&self.exp) {
                        LaTeX::Math(format!(
                            "\\frac{{{}{}}}{{{}}}",
                            prefix, numerator_string, denominator_string
                        ))
                    } else {
                        LaTeX::Math(format!(
                            "\\frac{{{}}}{{{}}}\\times 10^{{{}}}",
                            numerator_string, denominator_string, self.exp
                        ))
                    }
                }
            }
            UnitDesc::Custom(_map) => {
                todo!()
            }
        }
    }
}
