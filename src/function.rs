use crate::{error::CalcError, expr::val::Val, parser::fn_call::FnCall};
use crate::{expr::unit::Unit, expr::Expr, statement::Scope};
use std::ops::RangeInclusive;

type FunctionArgsRange = (RangeInclusive<usize>, fn(&[f64]) -> f64);

macro_rules! match_unary_fn {
    ($f:expr, $($name:ident),* $(,)?) => {
        {
            let res: Option<FunctionArgsRange> = match $f {
                $(stringify!($name) => Some((1..=1, |x: &[f64]| f64::$name(x[0]))),)*
                _ => None,
            };
            res
        }
    };
}

pub fn eval_fn_call(fc: &FnCall, scope: &Scope) -> Result<Val, CalcError> {
    let e = |a: &Expr| a.eval(scope);

    let name = fc.name.as_str();
    let args_len = fc.args.len();

    let fn_args_range = match_unary_fn!(
        name, abs, acos, acosh, asin, asinh, atan, atanh, cbrt, ceil, cos, cosh, exp, floor, ln,
        log10, log2, round, sin, sinh, sqrt, tan, tanh,
    )
    .or_else(|| match name {
        "atan2" => Some((2..=2, |x: &[f64]| f64::atan2(x[0], x[1]))),
        "min" => Some((1..=usize::MAX, |x: &[f64]| {
            x.iter().cloned().reduce(f64::min).unwrap()
        })),
        "max" => Some((1..=usize::MAX, |x: &[f64]| {
            x.iter().cloned().reduce(f64::max).unwrap()
        })),
        _ => None,
    });

    // TODO: Handle values with units
    if let Some((args_range, calc)) = fn_args_range {
        if args_range.contains(&args_len) {
            let args: Result<Vec<f64>, CalcError> = fc
                .args
                .iter()
                .map(|a| {
                    let a = e(&a)?;
                    if a.unit.desc.is_empty() {
                        Ok(a.num * a.unit.mult * 10f64.powi(a.unit.exp as i32))
                    } else {
                        Err(CalcError::UnitError(format!(
                            "Can't take {} of unit-ed value",
                            fc.name
                        )))
                    }
                })
                .collect();
            let args = args?;

            let res: Val = (calc(args.as_slice()), Unit::empty()).into();
            Ok(res.clamp_num())
        } else {
            Err(CalcError::Other(format!(
                "Incorrect number of arguments to function {}, expected {:?} but got {}",
                name, args_range, args_len
            )))
        }
    } else {
        Err(CalcError::Other(format!("Unknown function {}", name)))
    }
}
