mod expr;
mod latex;
mod parser;

mod statement;
use statement::State;

pub mod error;
use error::CalcError;

pub mod function;

pub fn generate_latex(input: &str) -> Result<String, CalcError> {
    let mut state = State::new(input)?;
    state.exec()?;
    Ok(state.output)
}

// these tests don't work on windows, probably because of line break weirdness.
#[cfg(test)]
#[cfg(not(target_os = "windows"))]
mod tests {
    use crate::CalcError;

    fn run_on_file(filename: &str) -> Result<String, CalcError> {
        let contents = std::fs::read_to_string(filename).unwrap();
        super::generate_latex(&contents)
    }

    macro_rules! test_file {
        ( $name: ident ) => {
            #[test]
            fn $name() {
                let input_name = format!("test_files/{}.math", stringify!($name));
                let output_name = format!("test_outputs/{}.tex", stringify!($name));

                let output = run_on_file(input_name.as_str()).unwrap().replace(" ", "");
                println!("{}", output);
                assert_eq!(
                    output.trim(),
                    std::fs::read_to_string(output_name.as_str())
                        .unwrap()
                        .trim()
                        .replace(" ", ""),
                );
            }
        };
    }

    test_file!(basic);
    test_file!(function);
    test_file!(chemistry);
    test_file!(tutorial);
    test_file!(power_conversion);
    test_file!(adv_unit_expr);
    test_file!(amu_precision);
}
