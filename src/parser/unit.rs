use std::convert::TryInto;

use pest::iterators::{Pair, Pairs};

use crate::{
    expr::unit::Unit,
    expr::unit_expr::{UnitExpr, UnitOp},
    parser::Rule,
};

pub fn parse_unit_expr(r: Pair<Rule>) -> UnitExpr {
    assert_eq!(r.as_rule(), Rule::unit_expr);

    fn expr_bp(inp: &mut Pairs<Rule>, bp: u8) -> UnitExpr {
        if let Some(nx) = inp.next() {
            let mut lhs = {
                match nx.as_rule() {
                    Rule::unit => {
                        let rule_str = nx.as_str();
                        let unit: Unit = rule_str.try_into().unwrap();
                        UnitExpr::Atom(unit)
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

pub fn unit_postfix_binding_power(op: &UnitOp) -> Option<(u8, ())> {
    Some(match op {
        UnitOp::Exp(_) => (3, ()),
        _ => return None,
    })
}

pub fn unit_infix_binding_power(op: &UnitOp) -> (u8, u8) {
    match op {
        UnitOp::Mul | UnitOp::Div => (1, 2),
        _ => panic!(),
    }
}
