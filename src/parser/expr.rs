use pest::iterators::{Pair, Pairs};

use crate::{
    expr::unit::Unit,
    expr::val::Val,
    expr::{Expr, Op},
    parser::{parse_unit_expr, Rule},
};

pub fn parse_expr(r: Pair<Rule>) -> Expr {
    assert_eq!(r.as_rule(), Rule::expression);

    fn expr_bp(inp: &mut Pairs<Rule>, bp: u8) -> Expr {
        if let Some(nx) = inp.next() {
            let mut lhs = match nx.as_rule() {
                Rule::number => Expr::Atom(Val {
                    unit: Unit::empty(),
                    num: rug::Rational::from_f64(nx.as_str().trim().parse::<f64>().unwrap())
                        .unwrap(),
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
                        let s = nx.as_str().to_string();
                        let unit = parse_unit_expr(nx).eval();
                        Op::AddUnit(unit, s)
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
        Op::AddUnit(_, _) => (6, ()),
        _ => return None,
    })
}

fn infix_binding_power(op: &Op) -> (u8, u8) {
    match op {
        Op::Plus | Op::Minus => (1, 2),
        Op::Mul | Op::Div => (3, 4),
        Op::Exp => (7, 8),
        _ => panic!(),
    }
}
