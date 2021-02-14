use std::collections::BTreeMap;

use crate::Val;
use crate::{expr::Expr, latex::ToLaTeX};

#[derive(Default)]
pub struct Scope {
    pub variables: BTreeMap<String, Val>,
}

#[derive(Debug)]
pub enum Statement {
    ExprStmt { parsed: Expr, string: String },
    VarDec { lhs: String, rhs: Expr },
    PrintExpr { parsed: Expr, string: String },
}

#[derive(Default)]
pub struct State {
    pub scope: Scope,
    pub statements: Vec<Statement>,
}

impl State {
    pub fn exec(&mut self) {
        for stmt in self.statements.iter() {
            match stmt {
                Statement::ExprStmt { parsed: expr, .. } => {
                    let _res = expr.eval(&self.scope);
                }
                Statement::VarDec { lhs, rhs } => {
                    println!(
                        "${} = {}$\\newline",
                        lhs,
                        rhs.eval(&self.scope).to_latex().to_string()
                    );
                    self.scope
                        .variables
                        .insert(lhs.clone(), rhs.eval(&self.scope));
                }
                Statement::PrintExpr { parsed: expr, .. } => {
                    println!(
                        "${} = {}\\newline$",
                        expr.to_latex().to_string(),
                        expr.eval(&self.scope).to_latex().to_string(),
                    );
                }
            }
        }
    }
}
