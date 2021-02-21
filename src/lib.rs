mod expr;
mod latex;
mod parser;

mod statement;
use statement::State;

pub fn generate_latex(input: &str) -> String {
    let mut state = State::new(input);
    state.exec();
    state.output
}
