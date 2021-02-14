use pest::Parser;

mod parser;
use parser::{parse_expr, MathParser, Rule};

mod expr;
use expr::val::Val;

mod statement;
use statement::{Scope, State};

fn full_eval(s: &str) -> Val {
    let scope = Scope::default();

    parse_expr(
        MathParser::parse(Rule::expression, s)
            .unwrap()
            .next()
            .unwrap(),
    )
    .eval(&scope)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        assert_eq!(full_eval("5 - 3").to_string(), "2".to_string());
        assert_eq!(
            full_eval("5 grams - 4 grams").to_string(),
            "1 gram".to_string()
        );
        assert_eq!(
            full_eval("5 grams + 4 grams").to_string(),
            "9 grams".to_string()
        );
        assert_eq!(
            full_eval("5 kilograms + 4 grams").to_string(),
            "5004 grams".to_string()
        );
        assert_eq!(
            full_eval("5 meters * 4 grams").to_string(),
            "20 meter grams".to_string()
        );
        assert_eq!(
            full_eval("5 meters / 4 grams").to_string(),
            "1.25 meter gram^-1s".to_string()
        );
    }
}

fn main() {
    let args = std::env::args().collect::<Vec<String>>();
    let filename = &args[1];
    let contents = std::fs::read_to_string(filename).unwrap();

    let mut state = State {
        scope: Scope::default(),
        statements: parser::parse_block(&contents),
    };

    state.exec();
}
