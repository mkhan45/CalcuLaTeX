use pest::{iterators::Pairs, Parser};

use pest_derive::*;

#[derive(Parser)]
#[grammar = "parser/grammar.pest"]
pub struct MathParser;

#[derive(Debug)]
pub enum Expr {
    Atom(Val),
    Cons(Op, Vec<Expr>),
    Empty,
}

impl std::fmt::Display for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Expr::Atom(v) => write!(f, "{}", v),
            Expr::Cons(op, e) => write!(f, "({:?}, {:?})", op, e),
            Expr::Empty => Ok(()),
        }
    }
}

impl Expr {
    fn eval(&self) -> Val {
        match self {
            Expr::Atom(v) => v.clone(),
            Expr::Cons(op, xs) => match (op, xs.as_slice()) {
                (Op::AddUnit(u), [v, ..]) => v.eval().add_unit(u.clone()),
                (Op::Plus, [a, b, ..]) => a.eval() + b.eval(),
                (Op::Minus, [a, b, ..]) => a.eval() - b.eval(),
                _ => todo!(),
            },
            _ => todo!(),
        }
    }
}

#[derive(Debug)]
pub enum Op {
    Plus,
    Minus,
    AddUnit(Unit),
}

#[derive(Debug, Clone)]
pub struct Val {
    unit: Option<Unit>,
    num: Num,
}

impl Val {
    fn add_unit(&self, u: Unit) -> Val {
        Val {
            unit: Some(u),
            num: self.num,
        }
    }
}

impl std::ops::Add<Val> for Val {
    type Output = Val;

    fn add(self, rhs: Val) -> Self::Output {
        assert!(self.unit == rhs.unit);
        Val {
            unit: self.unit,
            num: self.num + rhs.num,
        }
    }
}

impl std::ops::Sub<Val> for Val {
    type Output = Val;

    fn sub(self, rhs: Val) -> Self::Output {
        assert!(self.unit == rhs.unit);
        Val {
            unit: self.unit,
            num: self.num - rhs.num,
        }
    }
}

impl std::fmt::Display for Val {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(unit) = &self.unit {
            write!(f, "{} {}", self.num, unit)
        } else {
            write!(f, "{}", self.num)
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Unit {
    base: BaseUnit,
    power: isize,
}

impl Unit {
    fn from_str(s: &str) -> Self {
        Unit {
            base: BaseUnit(s.to_string()),
            power: 1,
        }
    }
}

impl std::fmt::Display for Unit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let prefix = match self.power {
            -2 => "centi",
            -1 => "deci",
            1 => "",
            2 => "deca",
            3 => "kilo",
            _ => todo!(),
        };

        write!(f, "{}{}", prefix, self.base.0)
    }
}

#[derive(Debug, Copy, Clone)]
pub enum Num {
    Int(isize),
    Float(f64),
}

impl std::ops::Add<Num> for Num {
    type Output = Num;

    fn add(self, rhs: Num) -> Self::Output {
        match (self, rhs) {
            (Num::Int(a), Num::Int(b)) => Num::Int(a + b),
            (Num::Float(a), Num::Float(b)) => Num::Float(a + b),
            _ => todo!(),
        }
    }
}

impl std::ops::Sub<Num> for Num {
    type Output = Num;

    fn sub(self, rhs: Num) -> Self::Output {
        match (self, rhs) {
            (Num::Int(a), Num::Int(b)) => Num::Int(a - b),
            (Num::Float(a), Num::Float(b)) => Num::Float(a - b),
            _ => todo!(),
        }
    }
}

impl std::fmt::Display for Num {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Num::Int(i) => write!(f, "{}", i),
            Num::Float(n) => write!(f, "{}", n),
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct BaseUnit(String);

pub fn parse_expr(mut inp: Pairs<Rule>) -> Expr {
    fn expr_bp(inp: &mut Pairs<Rule>, bp: u8) -> Expr {
        if let Some(nx) = inp.next() {
            let mut lhs = match nx.as_rule() {
                Rule::number => Expr::Atom(Val {
                    unit: None,
                    num: Num::Int(nx.as_str().trim().parse().unwrap()),
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
                        Rule::unit => Op::AddUnit(Unit::from_str(nx.as_str().trim())),
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

fn full_eval(s: &str) -> Val {
    let parsed = MathParser::parse(Rule::expression, s).unwrap();
    parse_expr(parsed).eval()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        assert_eq!(full_eval("5 - 3").to_string(), "2".to_string());
        assert_eq!(full_eval("5 - 4 grams").to_string(), "1 grams".to_string());
        assert_eq!(full_eval("5 + 4 grams").to_string(), "9 grams".to_string());
    }
}

fn main() {
    let args = std::env::args().collect::<Vec<String>>();
    println!("{}", full_eval(&args[1]));
}
