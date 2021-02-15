use notify::{self, Watcher};

mod parser;
mod unit_expr;

mod expr;
use expr::val::Val;

mod statement;
use statement::State;

mod latex;

use std::io::Write;

#[cfg(test)]
fn full_eval(s: &str) -> Val {
    use crate::parser::*;
    use pest::Parser;
    use statement::Scope;

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

    let (tx, rx) = std::sync::mpsc::channel();
    let mut watcher = notify::PollWatcher::with_delay_ms(tx, 500).unwrap();

    watcher
        .watch(filename, notify::RecursiveMode::NonRecursive)
        .unwrap();

    loop {
        match rx.recv() {
            Ok(_) => {
                println!("rebuilding pdf");
                let contents = std::fs::read_to_string(filename).unwrap();

                let mut state = State::new(&contents);

                state.exec();

                let mut md_file = tempfile::NamedTempFile::new().unwrap();
                write!(md_file, "{}", state.output).unwrap();

                let mut pandoc = pandoc::new();
                pandoc.set_input_format(pandoc::InputFormat::Latex, Vec::new());
                pandoc.add_input(&md_file.path());

                pandoc.set_output(pandoc::OutputKind::File(args[2].to_string().into()));
                pandoc.execute().unwrap();
                println!("done rebuilding pdf");
            }
            Err(e) => println!("watch error: {:?}", e),
        }
    }
}
