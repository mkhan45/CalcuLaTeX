use crate::parser::naive_string::parse_naive_string;
use crate::{error::CalcError, latex::UnitHint};
use pest::iterators::Pair;
use pest::Parser;
use pest_derive::*;

use crate::statement::Statement;

pub mod unit;
use unit::parse_unit_expr;

pub mod expr;
use expr::parse_expr;

pub mod naive_string;

#[derive(Parser)]
#[grammar = "parser/grammar.pest"]
pub struct MathParser;

fn parse_var_dec(r: Pair<Rule>) -> Result<Statement, CalcError> {
    assert_eq!(r.as_rule(), Rule::var_dec);
    let mut inner = r.into_inner();
    let lhs = inner.next().unwrap();
    let rhs = inner.next().unwrap();
    Ok(Statement::VarDec {
        lhs: lhs.as_str().to_string(),
        rhs: parse_expr(rhs)?,
    })
}

fn parse_print_stmt(r: Pair<Rule>) -> Result<Statement, CalcError> {
    assert_eq!(r.as_rule(), Rule::print_expr);
    let mut inner = r.into_inner();
    let lhs = inner.next().unwrap();

    let unit_hint = match inner.next() {
        Some(r) => Some(UnitHint {
            unit: parse_unit_expr(r.clone())?.eval(),
            pretty_string: parse_naive_string(r)?,
        }),
        None => None,
    };

    Ok(Statement::PrintExpr {
        expr: parse_expr(lhs)?,
        unit_hint,
    })
}

fn parse_dec_print_stmt(r: Pair<Rule>) -> Result<Statement, CalcError> {
    assert_eq!(r.as_rule(), Rule::dec_print_expr);
    let mut inner = r.into_inner();
    let lhs = inner.next().unwrap();
    let rhs = inner.next().unwrap();

    let unit_hint = match inner.next() {
        Some(r) => Some(UnitHint {
            unit: parse_unit_expr(r.clone())?.eval(),
            pretty_string: parse_naive_string(r)?,
        }),
        None => None,
    };

    Ok(Statement::DecPrintExpr {
        lhs: lhs.as_str().to_string(),
        rhs: parse_expr(rhs)?,
        unit_hint,
    })
}

fn parse_digit_set(r: Pair<Rule>) -> Statement {
    assert_eq!(r.as_rule(), Rule::digit_set);
    let mut inner = r.into_inner();
    let n_digits = inner.next().unwrap().as_str().parse::<usize>().unwrap();
    Statement::DigitSet(n_digits)
}

pub fn parse_block(s: &str) -> Result<Vec<(usize, Statement)>, CalcError> {
    let inp = MathParser::parse(Rule::program, s)?;
    inp.map(|s| {
        let stmt = s.into_inner().next().unwrap();
        let (line, _) = stmt.as_span().start_pos().line_col();
        let add_line = |e: CalcError| e.add_line(line);
        Ok((
            line,
            match stmt.as_rule() {
                Rule::digit_set => parse_digit_set(stmt),
                Rule::set_scientific => Statement::SetScientific,
                Rule::var_dec => parse_var_dec(stmt).map_err(add_line)?,
                Rule::print_expr => parse_print_stmt(stmt).map_err(add_line)?,
                Rule::dec_print_expr => parse_dec_print_stmt(stmt).map_err(add_line)?,
                Rule::line_gap_stmt => Statement::LineGap,
                Rule::latex_block => Statement::RawLaTeX(
                    stmt.as_str()
                        .trim_start_matches("'''")
                        .trim_end_matches("'''")
                        .to_owned(),
                ),
                Rule::error => {
                    return Err(CalcError::Other(format!(
                        "Invalid statement {}",
                        stmt.as_str()
                    )))
                    .map_err(add_line)?
                }
                _ => unreachable!(),
            },
        ))
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
