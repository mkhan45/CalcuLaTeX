use super::unit::Unit;

#[derive(Debug, Clone)]
pub struct Val {
    pub num: f64,
    pub unit: Unit,
}

impl std::fmt::Display for Val {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let out = format!("{} {}", self.num, self.unit.to_string());
        write!(f, "{}", out.trim())
    }
}

impl std::ops::Add<Val> for Val {
    type Output = Val;

    fn add(self, rhs: Val) -> Self::Output {
        if self.unit == rhs.unit {
            Val {
                num: self.num + rhs.num,
                unit: self.unit,
            }
        } else {
            panic!("Can't add")
        }
    }
}

impl std::ops::Sub<Val> for Val {
    type Output = Val;

    fn sub(self, rhs: Val) -> Self::Output {
        if self.unit == rhs.unit {
            Val {
                num: self.num - rhs.num,
                unit: self.unit,
            }
        } else {
            panic!(format!("Can't sub {} from {}", rhs, self))
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
        if self.unit == Unit::empty() {
            Val {
                num: self.num,
                unit: unit.clone(),
            }
        } else {
            Val {
                num: self.num,
                unit: self.unit.clone() * unit.clone(),
            }
        }
    }

    pub fn pow(&self, rhs: &Val) -> Val {
        if rhs.unit == Unit::empty() && rhs.num.fract() == 0.0 {
            let pow = rhs.num as i8;
            let unit = self.unit.pow(pow);
            Val {
                num: self.num.powi(rhs.num as i32),
                unit,
            }
        } else {
            panic!()
        }
    }
}
