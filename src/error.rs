use std::error::Error;

use pest::error::Error as PestError;

use crate::parser::Rule;

#[derive(Debug)]
pub enum CalcError {
    ParseError(PestError<Rule>),
    UnitError(String),
    MathError,
    Other(String),
}

impl std::fmt::Display for CalcError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CalcError::ParseError(e) => {
                let location = &e.line_col;
                match location {
                    pest::error::LineColLocation::Pos((line, col)) => {
                        write!(f, "Parsing Error near line {} column {}\n", line, col)
                    }
                    pest::error::LineColLocation::Span((l1, c1), (l2, c2)) => {
                        write!(
                            f,
                            "Parsing Error near span from line {}, column {} to line {} column {}\n",
                            l1, c1, l2, c2,
                        )
                    }
                }
            }
            CalcError::UnitError(s) => {
                write!(f, "{}", s)
            }
            CalcError::MathError => {
                write!(f, "Bad Math")
            }
            CalcError::Other(s) => {
                write!(f, "{}", s)
            }
        }
    }
}

impl Error for CalcError {
    fn description(&self) -> &str {
        use CalcError::*;
        match self {
            ParseError(_) => "Parse Error",
            UnitError(_) => "Unit Error",
            MathError => "Math Error",
            Other(_) => "Error",
        }
    }
}

impl From<PestError<Rule>> for CalcError {
    fn from(e: PestError<Rule>) -> Self {
        CalcError::ParseError(e)
    }
}

impl From<&str> for CalcError {
    fn from(s: &str) -> Self {
        CalcError::Other(s.to_owned())
    }
}

impl CalcError {
    pub fn add_line(self, line: usize) -> Self {
        CalcError::Other(format!("Line {}: {}", line, self))
    }
}
