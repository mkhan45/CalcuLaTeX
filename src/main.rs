mod parser;
use parser::parse_expr;

mod expr;
use expr::val::Val;

fn full_eval(s: &str) -> Val {
    parse_expr(s).eval()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        assert_eq!(full_eval("5 - 3").to_string(), "2".to_string());
        assert_eq!(full_eval("5 - 4 grams").to_string(), "1 grams".to_string());
        assert_eq!(full_eval("5 + 4 grams").to_string(), "9 grams".to_string());
    }
}

fn main() {
    let args = std::env::args().collect::<Vec<String>>();
    println!("{}", full_eval(&args[1]));
}
