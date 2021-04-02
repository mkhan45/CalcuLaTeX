use crate::{error::CalcError, expr::val::Val, parser::fn_call::FnCall};
use crate::{expr::unit::Unit, expr::Expr, statement::Scope};
use std::ops::RangeInclusive;

type FunctionArgsRange = (RangeInclusive<usize>, fn(&[f64]) -> f64, UnitBehavior);

enum UnitBehavior {
    PreserveUnit,
    NoUnit,
    // Map(Box<dyn Fn(Unit) -> Unit>), // can be used when a function changes the unit
}

macro_rules! match_unary_fn {
    ($f:expr, $u:expr, [$($name:ident),* $(,)?]) => {
        {
            let res: Option<FunctionArgsRange> = match $f {
                $(stringify!($name) => Some((1..=1, |x: &[f64]| f64::$name(x[0]), $u)),)*
                _ => None,
            };
            res
        }
    };
}

fn assert_units_match(args: &Vec<Val>) -> Result<(), CalcError> {
    let first_unit = &args[0].unit.desc;
    if args[1..].iter().any(|v| &v.unit.desc != first_unit) {
        Err(CalcError::Other(format!(
            "Functions require all arguments to have the same unit"
        )))
    } else {
        Ok(())
    }
}

pub fn eval_fn_call(fc: &FnCall, scope: &Scope) -> Result<Val, CalcError> {
    let e = |a: &Expr| a.eval(scope);

    let name = fc.name.as_str();
    let args_len = fc.args.len();

    let fn_args_range = match_unary_fn!(
        name,
        UnitBehavior::NoUnit,
        [
            acos, acosh, asin, asinh, atan, atanh, cbrt, cos, cosh, exp, ln, log10, log2, sin,
            sinh, sqrt, tan, tanh
        ]
    )
    .or_else(|| match_unary_fn!(name, UnitBehavior::PreserveUnit, [abs, ceil, floor, round]))
    .or_else(|| match name {
        "atan2" => Some((
            2..=2,
            |x: &[f64]| f64::atan2(x[0], x[1]),
            UnitBehavior::NoUnit,
        )),
        "min" => Some((
            1..=usize::MAX,
            |x: &[f64]| x.iter().cloned().reduce(f64::min).unwrap(),
            UnitBehavior::PreserveUnit,
        )),
        "max" => Some((
            1..=usize::MAX,
            |x: &[f64]| x.iter().cloned().reduce(f64::max).unwrap(),
            UnitBehavior::PreserveUnit,
        )),
        _ => None,
    });

    // TODO: Handle values with units
    if let Some((args_range, calc, unit_behavior)) = fn_args_range {
        if args_range.contains(&args_len) {
            let evaled_args: Result<Vec<Val>, CalcError> = fc.args.iter().map(|a| e(&a)).collect();
            let evaled_args = evaled_args?;
            let args: Result<Vec<f64>, CalcError> = evaled_args
                .iter()
                .map(|a| {
                    if !a.unit.desc.is_empty() && matches!(unit_behavior, UnitBehavior::NoUnit) {
                        Err(CalcError::UnitError(format!(
                            "Can't take {} of unit-ed value",
                            fc.name
                        )))
                    } else {
                        Ok(a.num * a.unit.mult * 10f64.powi(a.unit.exp as i32))
                    }
                })
                .collect();
            let args = args?;

            let unit: Result<Unit, CalcError> = match unit_behavior {
                UnitBehavior::NoUnit => Ok(Unit::empty()),
                UnitBehavior::PreserveUnit => {
                    assert_units_match(&evaled_args)?;
                    let mut u = evaled_args[0].unit.clone();
                    u.exp = 0;
                    Ok(u)
                } // UnitBehavior::Map(_) => todo!(),
            };

            let res: Val = (calc(args.as_slice()), unit?).into();
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
