#![allow(clippy::clippy::suspicious_arithmetic_impl)]

use num::rational::Ratio;
use num::{One, Zero};
use std::fmt::Debug;

use std::collections::BTreeMap;

pub const UNIT_PREFIXES: [(&str, i8); 5] = [
    ("centi", -2),
    ("deci", -1),
    ("deca", 1),
    ("hecto", 2),
    ("kilo", 3),
];

// enum UnitType {
//     Length,
//     Mass,
//     Time,
//     Current,
//     Temperature,
//     Moles,
//     Luminosity,
// }

// impl ToString for UnitType {
//     fn to_string(&self) -> String {
//         match self {
//             UnitType::Length => "length",
//             UnitType::Mass => "mass",
//             UnitType::Time => "time",
//             UnitType::Current => "current",
//             UnitType::Temperature => "temperature",
//             UnitType::Moles => "moles",
//             UnitType::Luminosity => "luminosity",
//         }
//         .to_string()
//     }
// }

pub enum BaseUnit {
    Meter,
    Gram,
    Second,
    Ampere,
    Kelvin,
    Mole,
    Candela,
}

impl ToString for BaseUnit {
    fn to_string(&self) -> String {
        match self {
            BaseUnit::Meter => "m",
            BaseUnit::Gram => "gm",
            BaseUnit::Second => "s",
            BaseUnit::Ampere => "A",
            BaseUnit::Kelvin => "K",
            BaseUnit::Mole => "M",
            BaseUnit::Candela => "cd",
        }
        .to_string()
    }
}

pub const BASE_UNITS: [BaseUnit; 7] = [
    BaseUnit::Meter,
    BaseUnit::Gram,
    BaseUnit::Second,
    BaseUnit::Ampere,
    BaseUnit::Kelvin,
    BaseUnit::Mole,
    BaseUnit::Candela,
];

#[derive(Clone)]
pub enum Unit {
    Base([Ratio<i8>; 7]),
    Custom(BTreeMap<String, Ratio<u8>>),
}

impl Debug for Unit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

impl Unit {
    pub fn empty() -> Self {
        Unit::Base([Ratio::zero(); 7])
    }
}

impl PartialEq for Unit {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Unit::Base(a), Unit::Base(b)) => a == b,
            _ => todo!(),
        }
    }
}

impl std::convert::TryFrom<&str> for Unit {
    type Error = &'static str;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        Ok(match s.trim() {
            "meters" | "meter" | "m" => BaseUnit::Meter.into(),
            "grams" | "gram" | "gm" => BaseUnit::Gram.into(),
            "second" | "seconds" | "s" => BaseUnit::Second.into(),
            "amp" | "amps" | "ampere" | "amperes" => BaseUnit::Ampere.into(),
            "kelvin" | "K" => BaseUnit::Kelvin.into(),
            "moles" | "mols" | "mol" | "mole" | "M" => BaseUnit::Mole.into(),
            "candela" => BaseUnit::Candela.into(),
            "J" | "joules" => Unit::Base([
                Ratio::from(2),
                Ratio::one(),
                Ratio::from(-2),
                Ratio::zero(),
                Ratio::zero(),
                Ratio::zero(),
                Ratio::zero(),
            ]),
            _ => {
                dbg!(s);
                return Err("Bad unit");
            }
        })
    }
}

impl From<BaseUnit> for Unit {
    fn from(b: BaseUnit) -> Self {
        let mut arr = [Ratio::zero(); 7];
        let index = match b {
            BaseUnit::Meter => 0,
            BaseUnit::Gram => 1,
            BaseUnit::Second => 2,
            BaseUnit::Ampere => 3,
            BaseUnit::Kelvin => 4,
            BaseUnit::Mole => 5,
            BaseUnit::Candela => 6,
        };
        arr[index] = Ratio::one();
        Unit::Base(arr)
    }
}

impl std::fmt::Display for Unit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Unit::Base(arr) => {
                let res =
                    arr.iter()
                        .zip(BASE_UNITS.iter())
                        .fold("".to_string(), |acc, (pow, unit)| match pow {
                            r if r == &Ratio::zero() => acc,
                            r if r == &Ratio::one() => format!("{} {}", acc, unit.to_string()),
                            _ => format!("{} {}^{}", acc, unit.to_string(), pow),
                        });
                write!(f, "{}", res.trim())
            }
            Unit::Custom(_map) => {
                todo!()
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_disp() {
        let u: Unit = BaseUnit::Meter.into();
        assert_eq!(format!("{}", u).as_str(), "m");

        let u = Unit::Base([
            Ratio::one(),
            Ratio::one(),
            Ratio::zero(),
            Ratio::zero(),
            Ratio::zero(),
            Ratio::zero(),
            Ratio::zero(),
        ]);
        assert_eq!(format!("{}", u).as_str(), "m gm");

        let u = Unit::Base([
            Ratio::one(),
            Ratio::one() * 2,
            -Ratio::one(),
            Ratio::zero(),
            Ratio::zero(),
            Ratio::zero(),
            Ratio::zero(),
        ]);
        assert_eq!(format!("{}", u).as_str(), "m gm^2 s^-1");
    }
}

impl std::ops::Mul for Unit {
    type Output = Unit;

    fn mul(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Unit::Base(a), Unit::Base(b)) => {
                let mut res = [Ratio::zero(); 7];
                res.iter_mut()
                    .zip(a.iter().zip(b.iter()))
                    .for_each(|(r, (a, b))| {
                        *r = a + b;
                    });
                Unit::Base(res)
            }
            _ => todo!(),
        }
    }
}

impl std::ops::Div for Unit {
    type Output = Unit;

    fn div(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Unit::Base(a), Unit::Base(b)) => {
                let mut res = [Ratio::zero(); 7];
                res.iter_mut()
                    .zip(a.iter().zip(b.iter()))
                    .for_each(|(r, (a, b))| {
                        *r = a - b;
                    });
                Unit::Base(res)
            }
            _ => todo!(),
        }
    }
}
