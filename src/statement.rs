use crate::latex::FormatArgs;
use crate::latex::UnitHint;
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
        unit_hint: Option<UnitHint>,
    },
    DecPrintExpr {
        lhs: String,
        rhs: Expr,
        unit_hint: Option<UnitHint>,
    },
    LineGap,
    RawLaTeX(String),
}

#[derive(Default)]
pub struct State {
    // Contains the variables in the program.
    // Currently there is only one global scope
    // and I don't think more is necessary
    pub scope: Scope,
    // The statements to be executed
    pub statements: Vec<Statement>,
    // The LaTeX output buffer
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
                Statement::LineGap => self.output.push_str("\\\\"),
                Statement::RawLaTeX(s) => self.output.push_str(s),
                Statement::ExprStmt(expr) => {
                    // expr statements don't really have a use since
                    // there are no functions with side effects
                    // realistically this shouldn't even be evaluated
                    let _res = expr.eval(&self.scope);
                }
                Statement::VarDec { lhs, rhs } => {
                    // lhs is just the variable name.
                    // rhs is an expression. In this case, we don't
                    // evaluate the expression, just latexify it.
                    // Example: `x = 5 * 10 g` gets parsed roughly as
                    //
                    // ```
                    // Statement::VarDec {
                    //      lhs: "x",
                    //      rhs: parse_expr("5 * 10 g")
                    // }
                    // ```
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
                    // Example: `5 * 10 kg = ? g` gets parsed roughly as
                    //
                    // ```
                    // Statement::PrintExpr {
                    //      expr: parse_expr("5 * 10 g"),
                    //      unit_hint: Gram
                    // }
                    // ```
                    let format_args = FormatArgs {
                        unit_hint: unit_hint.clone(),
                    };

                    self.output.push_str(
                        format!(
                            "${} = {}$\\\\\n",
                            expr.to_latex().to_string().trim(),
                            expr.eval(&self.scope)
                                .to_latex_ext(&format_args)
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
                    // `DecPrintExpr` could probably be merged with `VarDec`,
                    // basically it's a combination of `PrintExpr` and `VarDec`
                    let val = rhs.eval(&self.scope);
                    let format_args = FormatArgs {
                        unit_hint: unit_hint.clone(),
                    };

                    self.output.push_str(
                        format!(
                            "${} = {} = {}$\\\\\n",
                            lhs.trim(),
                            rhs.to_latex().to_string().trim_end(),
                            val.to_latex_ext(&format_args).to_string().trim_end(),
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
