use std::convert::TryFrom;

use super::unit::Unit;

use rug::{self, ops::Pow};

#[derive(Debug, Clone)]
pub struct Val {
    pub num: rug::Rational,
    pub unit: Unit,
}

impl std::fmt::Display for Val {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let out = format!(
            "{} {}",
            self.num.to_f64().to_string(),
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
                num: self.num
                    + rhs.num * rhs.unit.mult * rug::Rational::from(10i32.pow(rhs.unit.exp as u32)),
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
                num: self.num - rhs.num * rhs.unit.mult,
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
    pub fn with_unit(&self, unit: &Unit) -> Val {
        if self.unit.desc.is_empty() {
            let new_unit = Unit {
                desc: unit.desc.clone(),
                ..Default::default()
            };
            Val {
                num: rug::Rational::from(&self.num * &unit.mult)
                    * rug::Rational::try_from(10f64.powi(unit.exp as i32)).unwrap(),
                unit: new_unit,
            }
        } else {
            self.clone()
                * Val {
                    unit: unit.clone(),
                    num: rug::Rational::from(1),
                }
        }
    }

    pub fn pow(&self, rhs: &Val) -> Val {
        let p = rhs.num.to_f64();
        if rhs.unit.desc.is_empty() || p.fract() == 0.0 {
            let pow = p as i8;
            let unit = self.unit.pow(pow);
            Val {
                num: rug::Rational::try_from((&self.num.to_f64()).pow(p as f64)).unwrap(),
                unit,
            }
        } else {
            panic!()
        }
    }
}
