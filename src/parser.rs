use pest::iterators::{Pair, Pairs};
use pest::Parser;
use pest_derive::*;

use crate::statement::Statement;

use crate::{
    expr::unit::{Unit, UNIT_PREFIXES},
    expr::val::Val,
    expr::{Expr, Op},
};

#[derive(Parser)]
#[grammar = "parser/grammar.pest"]
pub struct MathParser;

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
                _ => unreachable!(),
            };

            loop {
                if let Some(nx) = inp.peek() {
                    let op = match nx.as_rule() {
                        Rule::operation => match nx.as_str().trim() {
                            "+" => Op::Plus,
                            "-" => Op::Minus,
                            "*" => Op::Mul,
                            "/" => Op::Div,
                            _ => panic!("Bad operator {}", nx.as_str().trim()),
                        },
                        Rule::unit => {
                            let token = nx.as_str();
                            let res = UNIT_PREFIXES.iter().find_map(|(prefix, pow)| {
                                if token.starts_with(prefix) {
                                    Some(Op::AddMultiUnit(*pow, token[prefix.len()..].into()))
                                } else {
                                    None
                                }
                            });

                            if let Some(op) = res {
                                op
                            } else {
                                Op::AddUnit(token.into())
                            }
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
                } else {
                    break;
                }
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
        Op::AddUnit(_) | Op::AddMultiUnit(_, _) => (9, ()),
        _ => return None,
    })
}

fn infix_binding_power(op: &Op) -> (u8, u8) {
    match op {
        Op::Plus | Op::Minus => (1, 2),
        Op::Mul | Op::Div => (3, 4),
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
    Statement::PrintExpr {
        string: lhs.as_str().to_string(),
        parsed: parse_expr(lhs),
    }
}

pub fn parse_block(s: &str) -> Vec<Statement> {
    let inp = MathParser::parse(Rule::program, s).unwrap();
    inp.map(|s| {
        let stmt = s.into_inner().next().unwrap();
        match stmt.as_rule() {
            Rule::expression => Statement::ExprStmt {
                string: stmt.as_str().to_string(),
                parsed: parse_expr(stmt),
            },
            Rule::var_dec => parse_var_dec(stmt),
            Rule::print_expr => parse_print_stmt(stmt),
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
