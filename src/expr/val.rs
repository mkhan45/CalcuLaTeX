use super::unit::Unit;

#[derive(Debug, Clone)]
pub struct Val {
    pub num: f64,
    pub unit: Unit,
}

impl std::fmt::Display for Val {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut out = format!("{} {}", self.num, self.unit.to_string());
        if (self.num.abs() - 1.0).abs() > f64::EPSILON && self.unit != Unit::empty() {
            out.push('s');
        }
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
            panic!("Can't add")
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
            panic!("Cannot add unit")
        }
    }
}
