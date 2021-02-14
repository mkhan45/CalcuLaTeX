use std::collections::BTreeMap;

use crate::expr::Expr;
use crate::Val;

pub struct Scope {
    pub variables: BTreeMap<String, Val>,
}

#[derive(Debug)]
pub enum Statement {
    ExprStmt(Expr),
    VarDec { lhs: String, rhs: Expr },
    PrintExpr(Expr),
}

pub struct State {
    pub scope: Scope,
    pub statements: Vec<Statement>,
}
