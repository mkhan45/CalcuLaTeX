use crate::expr::bool_expr::BoolOp;
use crate::parser::BoolExpr;
use crate::CalcError;
use pest::iterators::{Pair, Pairs};

use crate::parser::Rule;

pub fn parse_bool_expr(r: Pair<Rule>) -> Result<BoolExpr, CalcError> {
    assert_eq!(r.as_rule(), Rule::bool_expr);

    fn expr_bp(inp: &mut Pairs<Rule>, bp: u8) -> Result<BoolExpr, CalcError> {
        if let Some(nx) = inp.next() {
            let mut lhs = match nx.as_rule() {
                Rule::ident => BoolExpr::Ident(nx.as_str().trim().to_string()),
                Rule::bool_expr => BoolExpr::ParenExpr(Box::new(parse_bool_expr(nx)?)),
                Rule::bool_operation => {
                    let op = match nx.as_str().trim() {
                        "not" => BoolOp::Negate,
                        _ => return Err(CalcError::Other("Invalid prefix operation".to_string())),
                    };

                    if let Some(((), r_bp)) = prefix_binding_power(&op) {
                        let rhs = expr_bp(inp, r_bp)?;
                        BoolExpr::Cons(op, vec![rhs])
                    } else {
                        return Err(CalcError::Other("Invalid prefix operation".to_string()));
                    }
                }
                _ => {
                    dbg!(nx);
                    unreachable!();
                }
            };

            while let Some(nx) = inp.peek() {
                let op = match nx.as_rule() {
                    Rule::bool_operation => match nx.as_str().trim() {
                        "not" => BoolOp::Negate,
                        "and" => BoolOp::And,
                        "or" => BoolOp::Or,
                        "implies" => BoolOp::Implies,
                        "equals" => BoolOp::Equals,
                        _ => panic!("Bad operator {}", nx.as_str().trim()),
                    },
                    _ => {
                        dbg!(nx.as_str(), nx.as_rule());
                        unimplemented!();
                    }
                };

                if let Some((l_bp, ())) = postfix_binding_power(&op) {
                    if l_bp < bp {
                        break;
                    }
                    inp.next();

                    lhs = BoolExpr::Cons(op, vec![lhs]);

                    continue;
                }

                let (l_bp, r_bp) = infix_binding_power(&op);
                if l_bp < bp {
                    break;
                }
                inp.next();

                let rhs = expr_bp(inp, r_bp)?;
                lhs = BoolExpr::Cons(op, vec![lhs, rhs]);
            }

            Ok(lhs)
        } else {
            unreachable!()
        }
    }

    expr_bp(&mut r.into_inner(), 0)
}

fn prefix_binding_power(op: &BoolOp) -> Option<((), u8)> {
    Some(match op {
        BoolOp::Negate => ((), 9),
        _ => return None,
    })
}

#[allow(unreachable_code)]
fn postfix_binding_power(op: &BoolOp) -> Option<(u8, ())> {
    Some(match op {
        _ => return None,
    })
}

fn infix_binding_power(op: &BoolOp) -> (u8, u8) {
    match op {
        BoolOp::Equals => (0, 1),
        BoolOp::Implies => (2, 3),
        BoolOp::Or => (4, 5),
        BoolOp::And => (6, 7),
        _ => panic!(),
    }
}
