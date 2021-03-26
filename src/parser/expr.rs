use crate::parser::fn_call::parse_fn_call;
use crate::parser::naive_string::parse_naive_string;
use crate::CalcError;
use pest::iterators::{Pair, Pairs};

use crate::{
    expr::val::Val,
    expr::{Expr, Op},
    parser::{parse_unit_expr, Rule},
};

use crate::latex::ToLaTeX;

pub fn parse_expr(r: Pair<Rule>) -> Result<Expr, CalcError> {
    assert_eq!(r.as_rule(), Rule::expression);

    fn expr_bp(inp: &mut Pairs<Rule>, bp: u8) -> Result<Expr, CalcError> {
        if let Some(nx) = inp.next() {
            let mut lhs = match nx.as_rule() {
                Rule::number => Expr::Atom(Val::empty(nx.as_str().trim().parse::<f64>().unwrap())),
                Rule::ident => Expr::Ident(nx.as_str().trim().to_string()),
                Rule::fn_call => Expr::FnCall(parse_fn_call(nx)?),
                Rule::expression => parse_expr(nx)?,
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
                        let naive_expr = parse_naive_string(nx.clone())?.to_latex()?;
                        let unit_expr = parse_unit_expr(nx)?;
                        Op::AddUnit(unit_expr.eval(), naive_expr.to_string())
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

                let rhs = expr_bp(inp, r_bp)?;
                lhs = Expr::Cons(op, vec![lhs, rhs]);
            }

            Ok(lhs)
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

#[cfg(test)]
mod test {

    use super::*;

    use crate::{
        expr::unit::Unit,
        parser::{expr::parse_expr, MathParser, Rule},
        statement::Scope,
    };

    use pest::Parser;

    fn full_eval(s: &str) -> Val {
        parse_expr(
            MathParser::parse(Rule::expression, s)
                .unwrap()
                .next()
                .unwrap(),
        )
        .unwrap()
        .eval(&Scope::default())
        .unwrap()
    }

    impl PartialEq<&str> for Val {
        fn eq(&self, s: &&str) -> bool {
            &self.to_string().as_str() == s
        }
    }

    #[test]
    fn basic_sub() {
        assert_eq!(full_eval("5 - 3"), "2");
        assert_eq!(full_eval("5 m - 1 m"), "4 m");
        assert_eq!(full_eval("5 grams - 3 g"), "2 g");
    }

    #[test]
    fn basic_add() {
        assert_eq!(full_eval("5 + 3"), "8");
        assert_eq!(full_eval("5 m + 1 m"), "6 m");
        assert_eq!(full_eval("5 grams + 3 g"), "8 g");
    }

    #[test]
    fn basic_div() {
        assert_eq!(full_eval("4 / 2"), "2");
        assert_eq!(full_eval("9 m / 3 meters"), "3");
        assert_eq!(full_eval("12 grams / 4 g"), "3");
    }

    #[test]
    fn basic_mult() {
        assert_eq!(full_eval("4 * 2"), "8");
        assert_eq!(full_eval("2 m * 3 meters"), "6 m^2");
        assert_eq!(full_eval("1 grams * 4 g"), "4 g^2");
    }

    #[test]
    fn complex_sub() {
        assert_eq!(full_eval("2 N - 0.5 N"), "1500 m g s^-2");
        assert_eq!(full_eval("2 N"), "2000 m g s^-2");
        assert_eq!(full_eval("2 kN - 1 centinewton"), "1999990 m g s^-2");
    }

    #[test]
    fn complex_add() {
        assert_eq!(full_eval("2 N + 0.5 N"), "2500 m g s^-2");
        assert_eq!(full_eval("2 kN + 1 centinewton"), "2000010 m g s^-2");
    }

    #[test]
    fn complex_add_neg() {
        assert_eq!(full_eval("2 N + -0.5 N"), "1500 m g s^-2");
        assert_eq!(full_eval("2 kN + -1 centinewton"), "1999990 m g s^-2");
    }
}
