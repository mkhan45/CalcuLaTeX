mod expr;
mod latex;
mod parser;

mod statement;
use statement::State;

pub mod error;
use error::CalcError;

pub fn generate_latex(input: &str) -> Result<String, CalcError> {
    let mut state = State::new(input)?;
    state.exec()?;
    Ok(state.output)
}
