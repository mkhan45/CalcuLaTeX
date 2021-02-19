use std::convert::TryFrom;
use rug::{self, ops::Pow};

use super::unit::Unit;

use std::fmt::{
    self, Display, Debug, Formatter
};

#[derive(Clone)]
pub struct Val {
    pub num: f64,
    pub unit: Unit,
}

impl PartialEq for Val {
    fn eq(&self, other: &Self) -> bool {
        self.num  == other.num &&
        self.unit == other.unit
    }
}

impl PartialEq<&str> for Val {
    fn eq(&self, other: &&str) -> bool {
        self.to_string() == other.to_string()
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
            self.unit.mult.to_f64() * self.num * 10f64.powi(self.unit.exp as i32),
            self.unit.to_string()
        );
        write!(f, "{}", out.trim())
    }
}

impl std::ops::Add<Val> for Val {
    type Output = Val;

    fn add(self, rhs: Val) -> Self::Output {
        if self.unit.desc == rhs.unit.desc {
            Val {
                num: self.num + rhs.num * rhs.unit.mult.to_f64() * 10i32.pow(rhs.unit.exp as u32) as f64,
                unit: Unit {
                    exp: self.unit.exp + rhs.unit.exp,
                    ..self.unit
                },
            }
        } else {
            panic!("Can't add")
        }
    }
}

impl std::ops::Sub<Val> for Val {
    type Output = Val;

    fn sub(self, rhs: Val) -> Self::Output {
        if self.unit.desc == rhs.unit.desc {
            Val {
                num: self.num - rhs.num * rhs.unit.mult.to_f64(),
                unit: Unit {
                    exp: self.unit.exp - rhs.unit.exp,
                    ..self.unit
                },
            }
        } else {
            std::panic::panic_any(format!("Can't sub {} from {}", rhs, self));
        }
    }
}

impl std::ops::Mul<Val> for Val {
    type Output = Val;

    fn mul(self, rhs: Val) -> Self::Output {
        Val {
            num: self.num * rhs.num,
            unit: self.unit * rhs.unit,
        }
    }
}

impl std::ops::Div<Val> for Val {
    type Output = Val;

    fn div(self, rhs: Val) -> Self::Output {
        Val {
            num: self.num / rhs.num,
            unit: self.unit / rhs.unit,
        }
    }
}

impl Val {

    pub fn empty(val: f64) -> Self {
        Self {
            unit: Unit::empty(),
            num:  val,
        }
    }

    pub fn with_unit(&self, unit: &Unit) -> Val {
        Val {
            num: self.num.clone(),
            unit: self.unit.clone() * unit.clone(),
        }
    }

    pub fn pow(&self, rhs: &Val) -> Val {
        let p = rhs.num;
        if rhs.unit.desc.is_empty() || p.fract() == 0.0 {
            let pow = p as i8;
            let unit = self.unit.pow(pow);
            Val {
                num: self.num.pow(p as f64),
                unit,
            }
        } else {
            panic!()
        }
    }
}

impl<V,U> TryFrom<(V,U)> for Val
    where V: Into<f64>,
          U: Into<Unit>
{
    type Error = rug::rational::TryFromFloatError;

    fn try_from((v,u): (V,U)) -> Result<Self,Self::Error> {
        Ok(Val {
            num:  v.into(),
            unit: u.into()
        })
    }
}

#[cfg(test)]
mod test {

    use super::*;
    use crate::expr::{Unit,BaseUnit};
    use std::convert::TryInto;

    #[test]
    fn create_val(){
        let val: Val = (2.5,BaseUnit::Meter).try_into().unwrap();
        assert_eq!(val.to_string(),"2.5 m");
    }

    #[test]
    fn add_val_success(){
        let val1: Val = (0.9,BaseUnit::Meter).try_into().unwrap();
        let val2: Val = (0.1,BaseUnit::Meter).try_into().unwrap();
        assert_eq!((val1 + val2).to_string(),"1 m");
    }

    // #[test]
    // fn add_val_failure(){
    //     let val1: Val = (0.9,BaseUnit::Meter).try_into().unwrap();
    //     let val2: Val = (0.1,BaseUnit::Gram).try_into().unwrap();
    //     assert_eq!((val1 + val2).to_string(),"");
    // }

}