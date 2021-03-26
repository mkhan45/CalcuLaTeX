use crate::{error::CalcError, expr::val::Val, parser::fn_call::FnCall};
use crate::{expr::unit::Unit, expr::Expr, statement::Scope};

pub fn eval_fn_call(fc: &FnCall, scope: &Scope) -> Result<Val, CalcError> {
    let e = |a: &Expr| a.eval(scope);
    match (fc.name.as_str(), &fc.args.as_slice()) {
        ("sin", &[a]) => {
            let mut a = e(&a)?;
            if a.unit == Unit::empty() {
                a.num = a.num.sin();
                Ok(a)
            } else {
                return Err(CalcError::UnitError(
                    "Can't take sin of unit-ed value".to_string(),
                ));
            }
        }
        ("sin", _) => {
            return Err(CalcError::Other(
                "Incorrect number of arguments to function sin()".to_string(),
            ))
        }
        ("cos", &[a]) => {
            let mut a = e(&a)?;
            if a.unit == Unit::empty() {
                a.num = a.num.cos();
                Ok(a)
            } else {
                return Err(CalcError::UnitError(
                    "Can't take cos of unit-ed value".to_string(),
                ));
            }
        }
        ("cos", _) => {
            return Err(CalcError::Other(
                "Incorrect number of arguments to function cos()".to_string(),
            ))
        }
        ("tan", &[a]) => {
            let mut a = e(&a)?;
            if a.unit == Unit::empty() {
                a.num = a.num.tan();
                Ok(a)
            } else {
                return Err(CalcError::UnitError(
                    "Can't take tan of unit-ed value".to_string(),
                ));
            }
        }
        ("tan", _) => {
            return Err(CalcError::Other(
                "Incorrect number of arguments to function tan()".to_string(),
            ))
        }
        _ => return Err(CalcError::Other("Unknown Function".to_string())),
    }
}
