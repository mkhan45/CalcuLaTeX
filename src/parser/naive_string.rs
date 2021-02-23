use crate::parser::Rule;
use crate::{
    error::CalcError,
    parser::unit::{unit_infix_binding_power, unit_postfix_binding_power},
};

use pest::iterators::{Pair, Pairs};

use crate::{expr::unit_expr::UnitOp, latex::*};

#[derive(Debug, Clone)]
pub enum StringExpr {
    Atom(String),
    Cons(UnitOp, Vec<StringExpr>),
}

pub fn parse_naive_string(r: Pair<Rule>) -> Result<StringExpr, CalcError> {
    assert_eq!(r.as_rule(), Rule::unit_expr);

    fn expr_bp(inp: &mut Pairs<Rule>, bp: u8) -> Result<StringExpr, CalcError> {
        if let Some(nx) = inp.next() {
            let mut lhs = {
                match nx.as_rule() {
                    Rule::unit => {
                        let rule_str = nx.as_str();
                        StringExpr::Atom(rule_str.to_string())
                    }
                    Rule::unit_expr => {
                        let s = parse_naive_string(nx)?.to_latex()?.to_string();
                        StringExpr::Atom(s)
                    }
                    _ => unreachable!(),
                }
            };

            while let Some(nx) = inp.peek() {
                let op = match nx.as_str().trim() {
                    "*" => UnitOp::Mul,
                    "/" => UnitOp::Div,
                    s if s.starts_with('^') => {
                        let n = s.strip_prefix('^').unwrap();
                        UnitOp::Exp(n.parse().unwrap())
                    }
                    _ => {
                        dbg!(nx);
                        panic!();
                    }
                };

                if let Some((l_bp, ())) = unit_postfix_binding_power(&op) {
                    if l_bp < bp {
                        break;
                    }

                    inp.next();
                    lhs = StringExpr::Cons(op, vec![lhs]);

                    continue;
                }

                let (l_bp, r_bp) = unit_infix_binding_power(&op);
                if l_bp < bp {
                    break;
                }
                inp.next();

                let rhs = expr_bp(inp, r_bp)?;
                lhs = StringExpr::Cons(op, vec![lhs, rhs]);

                continue;
            }

            Ok(lhs)
        } else {
            unreachable!()
        }
    }

    expr_bp(&mut r.into_inner(), 0)
}

impl ToLaTeX for StringExpr {
    fn to_latex(&self) -> Result<LaTeX, CalcError> {
        self.to_latex_ext(&FormatArgs::default())
    }

    fn to_latex_ext(&self, _: &FormatArgs) -> Result<LaTeX, CalcError> {
        Ok(match self {
            StringExpr::Atom(s) => LaTeX::Math(s.to_owned()),
            StringExpr::Cons(op, e) => match (op, e.as_slice()) {
                (UnitOp::Mul, [a, b, ..]) => LaTeX::Math(format!(
                    "{} \\ {}",
                    a.to_latex()?.to_string(),
                    b.to_latex()?.to_string()
                )),
                (UnitOp::Div, [a, b, ..]) => LaTeX::Math(format!(
                    "\\frac{{{}}}{{{}}}",
                    a.to_latex()?.to_string(),
                    b.to_latex()?.to_string()
                )),
                (UnitOp::Exp(e), [a, ..]) => LaTeX::Math(format!(
                    "{}^{{{}}}",
                    a.to_latex()?.to_string(),
                    e.to_string()
                )),
                _ => todo!(),
            },
        })
    }
}
