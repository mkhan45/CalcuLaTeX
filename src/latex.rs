use crate::expr::{unit::Unit, val::Val, Expr, Op};

pub enum LaTeX {
    Text(String),
    Math(String),
}

pub trait ToLaTeX {
    fn to_latex(&self) -> LaTeX;
}

impl ToString for LaTeX {
    fn to_string(&self) -> String {
        match self {
            LaTeX::Text(t) => t.to_owned(),
            LaTeX::Math(m) => m.to_string(),
        }
    }
}

impl ToLaTeX for Expr {
    fn to_latex(&self) -> LaTeX {
        match self {
            Expr::Atom(v) => LaTeX::Math(v.to_string()),
            Expr::Ident(n) => LaTeX::Math(n.to_string()),
            Expr::Cons(op, e) => match (op, e.as_slice()) {
                (Op::Plus, [a, b, ..]) => LaTeX::Math(format!(
                    "{} + {}",
                    a.to_latex().to_string(),
                    b.to_latex().to_string()
                )),
                (Op::Minus, [a, b, ..]) => LaTeX::Math(format!(
                    "{} - {}",
                    a.to_latex().to_string(),
                    b.to_latex().to_string()
                )),
                (Op::Mul, [a, b, ..]) => LaTeX::Math(format!(
                    "{} \\times {}",
                    a.to_latex().to_string(),
                    b.to_latex().to_string()
                )),
                (Op::Div, [a, b, ..]) => LaTeX::Math(format!(
                    "\\frac{{{}}}{{{}}}",
                    a.to_latex().to_string(),
                    b.to_latex().to_string()
                )),
                _ => todo!(),
            },
        }
    }
}

impl ToLaTeX for Val {
    fn to_latex(&self) -> LaTeX {
        let unit_str = if (self.num.abs() - 1.0).abs() > f64::EPSILON && self.unit != Unit::empty()
        {
            format!("{}s", self.unit.to_string())
        } else {
            self.unit.to_string()
        };
        let out = format!("{} \\text{{ {}}}", self.num, unit_str);
        LaTeX::Math(format!("{}", out.trim()))
    }
}
