use num::traits::Pow;

use crate::error::CalcError;

use super::unit::Unit;

use std::fmt::{self, Debug, Display, Formatter};

#[derive(Clone)]
pub struct Val {
    pub num: f64,
    pub unit: Unit,
}

impl std::ops::Neg for Val {
    type Output = Val;

    fn neg(self) -> Self::Output {
        Val {
            num: self.num * -1.0,
            ..self
        }
        .clamp_num()
    }
}

impl PartialEq for Val {
    fn eq(&self, other: &Self) -> bool {
        self.num == other.num && self.unit == other.unit
    }
}

impl Debug for Val {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

impl Display for Val {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let out = format!(
            "{} {}",
            // lossy conversions since Val's Display isn't actually used
            self.unit.mult * self.num * 10f64.powi(self.unit.exp as i32),
            self.unit.to_string()
        );
        write!(f, "{}", out.trim())
    }
}

impl std::ops::Add<Val> for Val {
    type Output = Result<Val, CalcError>;

    fn add(self, rhs: Val) -> Self::Output {
        if self.unit.desc == rhs.unit.desc {
            let (larger_exp, smaller_exp) = if rhs.unit.exp.abs() > self.unit.exp.abs() {
                (&rhs, &self)
            } else {
                (&self, &rhs)
            };

            let add_exp = larger_exp.unit.exp - smaller_exp.unit.exp;
            let mult_factor = smaller_exp.unit.mult / larger_exp.unit.mult;

            let mut num =
                larger_exp.num + (smaller_exp.num / 10f64.powi(add_exp as i32) * mult_factor);
            let mut exp = larger_exp.unit.exp;
            if num.abs() >= 10f64 {
                exp += num.log10() as i64;
                num /= 10f64.powi(num.log10() as i32);
            }

            Ok(Val {
                num,
                unit: Unit {
                    exp,
                    mult: larger_exp.unit.mult,
                    ..self.unit
                },
            }
            .clamp_num())
        } else {
            Err(CalcError::Other(format!(
                "Can't sub values with units {} and {}",
                self.unit.to_string(),
                rhs.unit.to_string()
            )))
        }
    }
}

impl std::ops::Sub<Val> for Val {
    type Output = Result<Val, CalcError>;

    fn sub(self, rhs: Val) -> Self::Output {
        if self.unit.desc == rhs.unit.desc {
            let larger_exp = if rhs.unit.exp.abs() > self.unit.exp.abs() {
                &rhs
            } else {
                &self
            };

            #[rustfmt::skip]
            let mut num =
                self.num / 10f64.powi((larger_exp.unit.exp - self.unit.exp) as i32) * self.unit.mult / larger_exp.unit.mult
                - rhs.num / 10f64.powi((larger_exp.unit.exp - rhs.unit.exp) as i32) * rhs.unit.mult / larger_exp.unit.mult;

            let mut exp = larger_exp.unit.exp;
            if num.abs() >= 10f64 {
                exp += num.log10() as i64;
                num /= 10f64.powi(num.log10() as i32);
            }

            Ok(Val {
                num,
                unit: Unit {
                    exp,
                    mult: larger_exp.unit.mult,
                    ..self.unit
                },
            }
            .clamp_num())
        } else {
            Err(CalcError::Other(format!(
                "Can't sub values with units {} and {}",
                self.unit.to_string(),
                rhs.unit.to_string()
            )))
        }
    }
}

impl std::ops::Mul<Val> for Val {
    type Output = Val;

    fn mul(self, rhs: Val) -> Self::Output {
        let mut new_num = self.num * rhs.num;
        let mut new_unit = self.unit * rhs.unit;

        if new_num.abs() >= 10f64 {
            new_unit.exp += new_num.log10() as i64;
            new_num = new_num.signum() * (new_num / 10f64.powi(new_num.log10() as i32)).abs();
        }

        Val {
            num: new_num,
            unit: new_unit,
        }
        .clamp_num()
    }
}

impl std::ops::Div<Val> for Val {
    type Output = Val;

    fn div(self, rhs: Val) -> Self::Output {
        let mut new_num = self.num / rhs.num;
        let mut new_unit = self.unit / rhs.unit;

        if new_num.abs() >= 10f64 {
            new_unit.exp += new_num.log10() as i64;
            new_num = new_num.signum() * new_num / 10f64.powi(new_num.log10() as i32);
        }

        Val {
            num: new_num,
            unit: new_unit,
        }
        .clamp_num()
    }
}

impl Val {
    pub fn empty(val: f64) -> Self {
        Self {
            unit: Unit::empty(),
            num: val,
        }
    }

    pub fn with_unit(&self, unit: &Unit) -> Val {
        Val {
            num: self.num,
            unit: self.unit.clone() * unit.clone(),
        }
    }

    pub fn pow(&self, rhs: &Val) -> Val {
        if rhs.unit.desc.is_empty() || rhs.num.fract() == 0.0 {
            let p = rhs.num * 10f64.powi(rhs.unit.exp as i32);
            // dbg!(self.num);
            // dbg!(self.unit.exp);
            // dbg!(self.unit.mult);
            // dbg!(p);
            // let unit = self.unit.pow(p as i64);
            // dbg!(self.num.pow(p));
            // dbg!(unit.exp);
            // dbg!(unit.mult);
            // Val {
            //     num: self.num.pow(p),
            //     unit,
            // }

            // lazy but simple
            // Avoids a bug with exponentiating small values.
            let scaled_num = self.num * 10f64.powi(self.unit.exp as i32) * self.unit.mult;
            Val {
                num: scaled_num.pow(p),
                unit: Unit {
                    desc: self.unit.pow(p as i64).desc,
                    ..Unit::default()
                },
            }
            .clamp_num()
        } else {
            panic!()
        }
    }

    pub fn clamp_num(&self) -> Val {
        if self.num == 0.0 {
            return self.clone();
        }

        let num_log10 = self.num.log10() as i64;
        let mult_log10 = self.unit.mult.log10() as i64;

        let mut res = Val {
            num: self.num / 10f64.powi(num_log10 as i32),
            unit: Unit {
                mult: self.unit.mult / 10f64.powi(mult_log10 as i32),
                exp: self.unit.exp + num_log10 + mult_log10,
                desc: self.unit.desc.clone(),
            },
        };

        if res.num.abs() < 1.0 {
            let n = (res.num.signum() * 1.0f64 / res.num).floor();
            res.unit.exp -= 1 + n.log10() as i64;
            res.num *= 10f64.powi(n.log10() as i32 + 1);
        }

        if res.unit.mult.is_sign_negative() {
            res.num *= -1.0;
            res.unit.mult *= -1.0;
        }

        res
    }
}

impl<V, U> From<(V, U)> for Val
where
    V: Into<f64>,
    U: Into<Unit>,
{
    fn from((v, u): (V, U)) -> Val {
        Self {
            unit: u.into(),
            num: v.into(),
        }
    }
}

#[cfg(test)]
mod test {

    use super::*;
    use crate::expr::BaseUnit;

    #[test]
    fn create_val() {
        let val: Val = (2.5, BaseUnit::Meter).into();
        assert_eq!(val.to_string(), "2.5 m");
    }

    #[test]
    fn add_val_success() {
        let val1: Val = (0.9, BaseUnit::Meter).into();
        let val2: Val = (0.1, BaseUnit::Meter).into();
        assert_eq!((val1 + val2).unwrap().to_string(), "1 m");
    }

    #[test]
    fn add_val_failure() {
        let val1: Val = (0.9, BaseUnit::Meter).into();
        let val2: Val = (0.1, BaseUnit::Gram).into();
        assert!((val1 + val2).is_err());
    }

    #[test]
    fn sub_val_success() {
        let val1: Val = (0.9, BaseUnit::Meter).into();
        let val2: Val = (0.1, BaseUnit::Meter).into();
        assert_eq!((val1 - val2).unwrap().to_string(), "0.8 m");
    }

    #[test]
    fn sub_val_failure() {
        let val1: Val = (0.9, BaseUnit::Meter).into();
        let val2: Val = (0.1, BaseUnit::Gram).into();
        assert!((val1 - val2).is_err());
    }

    #[test]
    fn mult_val_m_m_success() {
        let val1: Val = (1.5, BaseUnit::Meter).into();
        let val2: Val = (2, BaseUnit::Meter).into();
        assert_eq!((val1 * val2).to_string(), "3 m^2");
    }

    #[test]
    fn mult_val_m_g_success() {
        let val1: Val = (1.5, BaseUnit::Meter).into();
        let val2: Val = (2.0, BaseUnit::Gram).into();
        assert_eq!((val1 * val2).to_string(), "3 m g");
    }

    #[test]
    fn mult_val_m_s_success() {
        let val1: Val = (1.5, BaseUnit::Meter).into();
        let val2: Val = (2.0, BaseUnit::Second).into();
        assert_eq!((val1 * val2).to_string(), "3 m s");
    }

    #[test]
    fn mult_val_m_A_success() {
        let val1: Val = (1.5, BaseUnit::Meter).into();
        let val2: Val = (2.0, BaseUnit::Ampere).into();
        assert_eq!((val1 * val2).to_string(), "3 m A");
    }

    #[test]
    fn mult_val_pow_success() {
        let val1: Val = (1.5, BaseUnit::Meter).into();
        let val2: Val = (2.0, BaseUnit::Ampere).into();
        assert_eq!(val1.pow(&val2), "2.25 m^2");
    }
}
