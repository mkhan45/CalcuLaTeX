use std::collections::BTreeMap;

use crate::Val;
use crate::{expr::Expr, latex::ToLaTeX};

#[derive(Default)]
pub struct Scope {
    pub variables: BTreeMap<String, Val>,
}

#[derive(Debug)]
pub enum Statement {
    ExprStmt(Expr),
    VarDec { lhs: String, rhs: Expr },
    PrintExpr(Expr),
    DecPrintExpr { lhs: String, rhs: Expr },
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
                Statement::ExprStmt(expr) => {
                    let _res = expr.eval(&self.scope);
                }
                Statement::VarDec { lhs, rhs } => {
                    println!(
                        "${} = {}$\\newline",
                        lhs.trim(),
                        rhs.to_latex().to_string().trim_end(),
                    );
                    self.scope
                        .variables
                        .insert(lhs.clone(), rhs.eval(&self.scope));
                }
                Statement::PrintExpr(expr) => {
                    println!(
                        "${} = {}$\\newline",
                        expr.to_latex().to_string().trim(),
                        expr.eval(&self.scope).to_latex().to_string().trim_end(),
                    );
                }
                Statement::DecPrintExpr { lhs, rhs } => {
                    let val = rhs.eval(&self.scope);
                    println!(
                        "${} = {} = {}$\\newline",
                        lhs.trim(),
                        rhs.to_latex().to_string().trim_end(),
                        val.to_latex().to_string().trim_end(),
                    );
                    self.scope.variables.insert(lhs.clone(), val);
                }
            }
        }
    }
}
