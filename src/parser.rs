use pest::iterators::Pairs;
use pest::Parser;
use pest_derive::*;

use crate::{
    expr::unit::Unit,
    expr::val::Val,
    expr::{Expr, Op},
};

#[derive(Parser)]
#[grammar = "parser/grammar.pest"]
pub struct MathParser;

pub fn parse_expr(in_str: &str) -> Expr {
    fn expr_bp(inp: &mut Pairs<Rule>, bp: u8) -> Expr {
        if let Some(nx) = inp.next() {
            let mut lhs = match nx.as_rule() {
                Rule::number => Expr::Atom(Val {
                    unit: Unit::empty(),
                    num: 0.0, // TODO fix
                }),
                _ => {
                    dbg!(nx);
                    todo!();
                }
            };

            loop {
                if let Some(nx) = inp.peek() {
                    let op = match nx.as_rule() {
                        Rule::operation => match nx.as_str().trim() {
                            "+" => Op::Plus,
                            "-" => Op::Minus,
                            _ => panic!("Bad operator {}", nx.as_str().trim()),
                        },
                        Rule::unit => todo!(),
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
                } else {
                    break;
                }
            }

            lhs
        } else {
            Expr::Empty
        }
    }

    let mut inp = MathParser::parse(Rule::expression, in_str).unwrap();
    expr_bp(&mut inp.next().unwrap().into_inner(), 0)
}

fn postfix_binding_power(op: &Op) -> Option<(u8, ())> {
    Some(match op {
        Op::AddUnit(_) => (1, ()),
        _ => return None,
    })
}

fn infix_binding_power(op: &Op) -> (u8, u8) {
    match op {
        Op::Plus | Op::Minus => (1, 2),
        _ => panic!(),
    }
}
