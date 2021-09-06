use crate::expr::bool_expr::BoolExpr;
use crate::latex::UnitHint;
use crate::CalcError;
use crate::{expr::unit::Unit, latex::FormatArgs};
use std::collections::BTreeMap;

use crate::{expr::val::Val, parser};
use crate::{expr::Expr, latex::ToLaTeX};

pub struct Scope {
    pub variables: BTreeMap<String, Val>,
}

impl Default for Scope {
    fn default() -> Self {
        let mut variables = BTreeMap::new();
        variables.insert(
            "\\pi".to_owned(),
            Val {
                num: std::f64::consts::PI,
                unit: Unit::empty(),
            },
        );
        variables.insert(
            "e".to_owned(),
            Val {
                num: std::f64::consts::E,
                unit: Unit::empty(),
            },
        );
        Scope { variables }
    }
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
    Alias {
        lhs: String,
        rhs: String,
    },
    DigitSet(usize),
    SetScientific,
    LineGap,
    TTable {
        args: Vec<String>,
        exprs: Vec<BoolExpr>,
    },
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
    pub aliases: BTreeMap<String, String>,
}

impl State {
    pub fn new(contents: &str) -> Result<Self, CalcError> {
        let output = "\\documentclass{article}\n\\begin{document}\n".to_string();

        let mut aliases = BTreeMap::new();
        aliases.insert("pi".to_string(), "\\pi".to_string());

        Ok(State {
            scope: Scope::default(),
            statements: parser::parse_block(&contents)?,
            output,
            format_args: FormatArgs::default(),
            aliases,
        })
    }

    fn resolve_alias(&self, name: &str) -> String {
        let trimmed = name.trim().to_string();
        self.aliases.get(&trimmed).unwrap_or(&trimmed).to_string()
    }

    pub fn exec(&mut self) -> Result<(), CalcError> {
        for (line, stmt) in self.statements.iter() {
            let add_line = |e: CalcError| e.add_line(*line);
            match stmt {
                Statement::LineGap => self.output.push_str("\\\\"),
                Statement::DigitSet(n) => self.format_args.max_digits = *n,
                Statement::SetScientific => {
                    self.format_args.scientific_notation = !self.format_args.scientific_notation
                }
                Statement::Alias { lhs, rhs } => {
                    self.aliases.insert(lhs.to_owned(), rhs.to_owned());
                }
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
                    let lhs = self.resolve_alias(lhs);
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

                    let mut expr = expr.clone();
                    expr.resolve_aliases(&self.aliases);

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
                    let lhs = self.resolve_alias(lhs);
                    let mut rhs = rhs.clone();
                    rhs.resolve_aliases(&self.aliases);

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
                Statement::TTable { args, exprs } => {
                    self.output
                        .push_str(&crate::ttable::generate_ttable(args, exprs)?);
                }
            }
        }
        self.output.push_str("\\end{document}");
        Ok(())
    }
}
