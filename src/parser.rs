use crate::expr::unit::UNIT_PREFIXES_ABBR;
use std::convert::TryInto;

use pest::iterators::{Pair, Pairs};
use pest::Parser;
use pest_derive::*;

use crate::{
    statement::Statement,
    unit_expr::{UnitExpr, UnitOp},
};

use crate::{
    expr::unit::{Unit, UNIT_PREFIXES},
    expr::val::Val,
    expr::{Expr, Op},
};

use crate::latex::FormatArgs;

#[derive(Parser)]
#[grammar = "parser/grammar.pest"]
pub struct MathParser;

pub fn parse_unit_expr(r: Pair<Rule>) -> UnitExpr {
    assert_eq!(r.as_rule(), Rule::unit_expr);

    fn expr_bp(inp: &mut Pairs<Rule>, bp: u8) -> UnitExpr {
        if let Some(nx) = inp.next() {
            let mut lhs = {
                match nx.as_rule() {
                    Rule::unit => {
                        let rule_str = nx.as_str();
                        let mut res = UNIT_PREFIXES.iter().find_map(|(prefix, pow)| {
                            if let Some(stripped) = rule_str.strip_prefix(prefix) {
                                Some((stripped, pow))
                            } else {
                                None
                            }
                        });

                        if res == None {
                            res = UNIT_PREFIXES_ABBR.iter().find_map(|(prefix, pow)| {
                                if let Some(stripped) = rule_str.strip_prefix(prefix) {
                                    Some((stripped, pow))
                                } else {
                                    None
                                }
                            });
                        }

                        let (stripped, pow) = res.unwrap_or((rule_str, &0));
                        let unit: Unit = stripped.try_into().unwrap();
                        UnitExpr::Atom(Unit {
                            desc: unit.desc,
                            exp: unit.exp + pow,
                            mult: unit.mult,
                        })
                    }
                    Rule::unit_expr => {
                        let unit = parse_unit_expr(nx).eval();
                        UnitExpr::Atom(unit)
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
                    lhs = UnitExpr::Cons(op, vec![lhs]);

                    continue;
                }

                let (l_bp, r_bp) = unit_infix_binding_power(&op);
                if l_bp < bp {
                    break;
                }
                inp.next();

                let rhs = expr_bp(inp, r_bp);
                lhs = UnitExpr::Cons(op, vec![lhs, rhs]);

                continue;
            }

            lhs
        } else {
            unreachable!()
        }
    }

    expr_bp(&mut r.into_inner(), 0)
}

pub fn parse_expr(r: Pair<Rule>) -> Expr {
    assert_eq!(r.as_rule(), Rule::expression);

    fn expr_bp(inp: &mut Pairs<Rule>, bp: u8) -> Expr {
        if let Some(nx) = inp.next() {
            let mut lhs = match nx.as_rule() {
                Rule::number => Expr::Atom(Val {
                    unit: Unit::empty(),
                    num: nx.as_str().trim().parse().unwrap(),
                }),
                Rule::ident => Expr::Ident(nx.as_str().trim().to_string()),
                Rule::expression => parse_expr(nx),
                _ => {
                    dbg!(nx);
                    unreachable!();
                }
            };

            while let Some(nx) = inp.peek() {
                let op = match nx.as_rule() {
                    Rule::operation => match nx.as_str().trim() {
                        "+" => Op::Plus,
                        "-" => Op::Minus,
                        "*" => Op::Mul,
                        "/" => Op::Div,
                        "^" => Op::Exp,
                        _ => panic!("Bad operator {}", nx.as_str().trim()),
                    },
                    Rule::unit_expr => {
                        let unit = parse_unit_expr(nx).eval();
                        Op::AddUnit(unit)
                    }
                    _ => todo!(),
                };

                if let Some((l_bp, ())) = postfix_binding_power(&op) {
                    if l_bp < bp {
                        break;
                    }
                    inp.next();

                    lhs = Expr::Cons(op, vec![lhs]);

                    continue;
                }

                let (l_bp, r_bp) = infix_binding_power(&op);
                if l_bp < bp {
                    break;
                }
                inp.next();

                let rhs = expr_bp(inp, r_bp);
                lhs = Expr::Cons(op, vec![lhs, rhs]);
            }

            lhs
        } else {
            unreachable!()
        }
    }

    expr_bp(&mut r.into_inner(), 0)
}

fn postfix_binding_power(op: &Op) -> Option<(u8, ())> {
    Some(match op {
        Op::AddUnit(_) => (9, ()),
        _ => return None,
    })
}

fn unit_postfix_binding_power(op: &UnitOp) -> Option<(u8, ())> {
    Some(match op {
        UnitOp::Exp(_) => (3, ()),
        _ => return None,
    })
}

fn unit_infix_binding_power(op: &UnitOp) -> (u8, u8) {
    match op {
        UnitOp::Mul | UnitOp::Div => (1, 2),
        _ => panic!(),
    }
}

fn infix_binding_power(op: &Op) -> (u8, u8) {
    match op {
        Op::Plus | Op::Minus => (1, 2),
        Op::Mul | Op::Div => (3, 4),
        Op::Exp => (5, 6),
        _ => panic!(),
    }
}

fn parse_var_dec(r: Pair<Rule>) -> Statement {
    assert_eq!(r.as_rule(), Rule::var_dec);
    let mut inner = r.into_inner();
    let lhs = inner.next().unwrap();
    let rhs = inner.next().unwrap();
    Statement::VarDec {
        lhs: lhs.as_str().to_string(),
        rhs: parse_expr(rhs),
    }
}

fn parse_print_stmt(r: Pair<Rule>) -> Statement {
    assert_eq!(r.as_rule(), Rule::print_expr);
    let mut inner = r.into_inner();
    let lhs = inner.next().unwrap();
    let unit_hint = inner.next().map(|n| {
        let s = n.as_str();
        FormatArgs::UnitHint {
            value: parse_unit_expr(n).eval(),
            string: s.to_string(),
        }
    });

    Statement::PrintExpr {
        expr: parse_expr(lhs),
        unit_hint,
    }
}

fn parse_dec_print_stmt(r: Pair<Rule>) -> Statement {
    assert_eq!(r.as_rule(), Rule::dec_print_expr);
    let mut inner = r.into_inner();
    let lhs = inner.next().unwrap();
    let rhs = inner.next().unwrap();
    let unit_hint = inner.next().map(|n| {
        let s = n.as_str();
        FormatArgs::UnitHint {
            value: parse_unit_expr(n).eval(),
            string: s.to_string(),
        }
    });

    Statement::DecPrintExpr {
        lhs: lhs.as_str().to_string(),
        rhs: parse_expr(rhs),
        unit_hint,
    }
}

pub fn parse_block(s: &str) -> Vec<Statement> {
    let inp = MathParser::parse(Rule::program, s).unwrap();
    inp.map(|s| {
        let stmt = s.into_inner().next().unwrap();
        match stmt.as_rule() {
            Rule::expression => Statement::ExprStmt(parse_expr(stmt)),
            Rule::var_dec => parse_var_dec(stmt),
            Rule::print_expr => parse_print_stmt(stmt),
            Rule::dec_print_expr => parse_dec_print_stmt(stmt),
            _ => unreachable!(),
        }
    })
    .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parser() {
        // just check if it doesn't crash rn
        parse_block(
            "
                x = 5
                5 + 10
            ",
        );
    }
}
