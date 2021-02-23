use crate::latex::FormatArgs;
use crate::latex::UnitHint;
use crate::CalcError;
use std::collections::BTreeMap;

use crate::{expr::val::Val, parser};
use crate::{expr::Expr, latex::ToLaTeX};

#[derive(Default)]
pub struct Scope {
    pub variables: BTreeMap<String, Val>,
}

#[derive(Debug)]
pub enum Statement {
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
    DigitSet(usize),
    SetScientific,
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
    pub statements: Vec<(usize, Statement)>,
    // The LaTeX output buffer
    pub output: String,
    pub format_args: FormatArgs,
}

impl State {
    pub fn new(contents: &str) -> Result<Self, CalcError> {
        let output = "\\documentclass{article}\n\\begin{document}\n".to_string();

        Ok(State {
            scope: Scope::default(),
            statements: parser::parse_block(&contents)?,
            output,
            format_args: FormatArgs::default(),
        })
    }

    pub fn exec(&mut self) -> Result<(), CalcError> {
        for (line, stmt) in self.statements.iter() {
            let add_line = |e: CalcError| e.add_line(*line);
            match stmt {
                Statement::LineGap => self.output.push_str("\\\\"),
                Statement::DigitSet(n) => self.format_args.max_digits = *n,
                Statement::SetScientific => self.format_args.scientific_notation = true,
                Statement::RawLaTeX(s) => self.output.push_str(s),
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
                            rhs.to_latex_ext(&self.format_args)
                                .map_err(add_line)?
                                .to_string()
                                .trim_end(),
                        )
                        .as_str(),
                    );
                    self.scope
                        .variables
                        .insert(lhs.clone(), rhs.eval(&self.scope).map_err(add_line)?);
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
                        ..self.format_args
                    };

                    self.output.push_str(
                        format!(
                            "${} = {}$\\\\\n",
                            expr.to_latex_ext(&self.format_args)
                                .map_err(add_line)?
                                .to_string()
                                .trim(),
                            expr.eval(&self.scope)
                                .map_err(add_line)?
                                .to_latex_ext(&format_args)
                                .map_err(add_line)?
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
                    let val = rhs.eval(&self.scope).map_err(add_line)?;
                    let format_args = FormatArgs {
                        unit_hint: unit_hint.clone(),
                        ..self.format_args
                    };

                    self.output.push_str(
                        format!(
                            "${} = {} = {}$\\\\\n",
                            lhs.trim(),
                            rhs.to_latex_ext(&self.format_args)
                                .map_err(add_line)?
                                .to_string()
                                .trim_end(),
                            val.to_latex_ext(&format_args)
                                .map_err(add_line)?
                                .to_string()
                                .trim_end(),
                        )
                        .as_str(),
                    );
                    self.scope.variables.insert(lhs.clone(), val.clamp_num());
                }
            }
        }
        self.output.push_str("\\end{document}");
        Ok(())
    }
}
