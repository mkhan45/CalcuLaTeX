use crate::expr::bool_expr::BoolExpr;
use crate::CalcError;
use std::collections::BTreeMap;
use crate::latex::ToLaTeX;

pub fn generate_ttable(args: &[String], exprs: &[BoolExpr]) -> Result<String, CalcError> {
    let num_rows = 2usize.pow(args.len() as u32);

    // super slow but easy
    let arg_rows = (0..num_rows).map(|i| format!("{:b}", i).chars().map(|c| c == '1').collect::<Vec<_>>());
    let scopes = arg_rows.map(|bool_vec| {
        args.iter()
            .zip(std::iter::repeat(&false).take(args.len() - bool_vec.len()).chain(bool_vec.iter()))
            .map(|(a, b)| (a.clone(), b.clone()))
            .collect::<BTreeMap<String, bool>>()
    });

    let table = scopes.map(|scope| {
        exprs.iter().map(|expr| expr.eval(&scope)).collect::<Vec<_>>()
    });

    let mut header = String::new();
    for expr in exprs.iter() {
        header.push_str(format!(" ${}$ &", expr.to_latex()?.to_string()).as_str());
    }
    header.pop();
    header.push_str("\\\\\n");

    let mut rows = String::new();
    for row in table {
        for result in row.iter() {
            match result {
                Ok(true) => rows.push_str(" T &"),
                Ok(false) => rows.push_str(" F &"),
                Err(e) => rows.push_str(format!(" {} &", e.to_string()).as_str()),
            }
        }

        rows.push_str("\\\\\n");
        rows.push_str("\\hline");
    }

    let mut col_specifier = String::new();
    for _ in 0..exprs.len() {
        col_specifier.push_str("|c")
    }
    col_specifier.push('|');

    let table = format!("
    \\begin{{center}}
        \\begin{{tabular}}{{{col_specifier}}}
            \\hline
            {header}
            \\hline
            {rows}
        \\end{{tabular}}
    \\end{{center}}
    ", col_specifier=col_specifier, header=header, rows=rows);

    Ok(table)
}
