use crate::expr::unit::Unit;
use crate::latex::FormatArgs;
use crate::unit_expr::UnitPow;
use std::collections::BTreeMap;

use crate::{expr::Expr, latex::ToLaTeX};
use crate::{parser, Val};

#[derive(Default)]
pub struct Scope {
    pub variables: BTreeMap<String, Val>,
}

#[derive(Debug)]
pub enum Statement {
    ExprStmt(Expr),
    VarDec {
        lhs: String,
        rhs: Expr,
    },
    PrintExpr {
        expr: Expr,
        unit_hint: Option<FormatArgs>,
    },
    DecPrintExpr {
        lhs: String,
        rhs: Expr,
        unit_hint: Option<FormatArgs>,
    },
}

#[derive(Default)]
pub struct State {
    pub scope: Scope,
    pub statements: Vec<Statement>,
    pub output: String,
}

impl State {
    pub fn new(contents: &str) -> Self {
        let output = "\\documentclass{article}\n\\begin{document}\n".to_string();

        State {
            scope: Scope::default(),
            statements: parser::parse_block(&contents),
            output,
        }
    }

    pub fn exec(&mut self) {
        for stmt in self.statements.iter() {
            match stmt {
                Statement::ExprStmt(expr) => {
                    let _res = expr.eval(&self.scope);
                }
                Statement::VarDec { lhs, rhs } => {
                    self.output.push_str(
                        format!(
                            "${} = {}$\\\\\n",
                            lhs.trim(),
                            rhs.to_latex().to_string().trim_end(),
                        )
                        .as_str(),
                    );
                    self.scope
                        .variables
                        .insert(lhs.clone(), rhs.eval(&self.scope));
                }
                Statement::PrintExpr { expr, unit_hint } => {
                    self.output.push_str(
                        format!(
                            "${} = {}$\\\\\n",
                            expr.to_latex().to_string().trim(),
                            expr.eval(&self.scope)
                                .to_latex_ext(unit_hint.as_ref())
                                .to_string()
                                .trim_end(),
                        )
                        .as_str(),
                    );
                }
                Statement::DecPrintExpr {
                    lhs,
                    rhs,
                    unit_hint,
                } => {
                    let val = rhs.eval(&self.scope);
                    self.output.push_str(
                        format!(
                            "${} = {} = {}$\\\\\n",
                            lhs.trim(),
                            rhs.to_latex().to_string().trim_end(),
                            val.to_latex_ext(unit_hint.as_ref()).to_string().trim_end(),
                        )
                        .as_str(),
                    );
                    self.scope.variables.insert(lhs.clone(), val);
                }
            }
        }
        self.output.push_str("\\end{document}")
    }
}
