use pest::Parser;

mod parser;
use parser::{parse_expr, MathParser, Rule};

mod unit_expr;

mod expr;
use expr::val::Val;

mod statement;
use statement::{Scope, State};

mod latex;

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
            "1 gm".to_string()
        );
        assert_eq!(
            full_eval("5 grams + 4 grams").to_string(),
            "9 gm".to_string()
        );
        assert_eq!(
            full_eval("5 kilograms + 4 grams").to_string(),
            "5004 gm".to_string()
        );
        assert_eq!(
            full_eval("5 meters * 4 grams").to_string(),
            "20 m gm".to_string()
        );
        assert_eq!(
            full_eval("5 meters / 4 grams").to_string(),
            "1.25 m gm^-1".to_string()
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
