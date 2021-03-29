use crate::expr::Expr;
use crate::parser::parse_expr;
use crate::CalcError;

use pest::iterators::Pair;

use crate::parser::Rule;

#[derive(Debug)]
pub struct FnCall {
    pub name: String,
    pub args: Vec<Expr>,
}

pub fn parse_fn_call(r: Pair<Rule>) -> Result<FnCall, CalcError> {
    assert_eq!(r.as_rule(), Rule::fn_call);
    let mut inner = r.into_inner();

    let name = inner
        .next()
        .ok_or(CalcError::Other("Invalid function".to_string()))?
        .as_str()
        .to_string();

    let mut args: Vec<Expr> = Vec::new();
    while let Some(r) = inner.next() {
        args.push(parse_expr(r)?);
    }
    Ok(FnCall { name, args })
}
