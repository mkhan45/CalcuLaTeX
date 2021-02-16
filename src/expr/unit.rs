#![allow(clippy::clippy::suspicious_arithmetic_impl)]

use num::rational::Ratio;
use num::{One, Zero};
use std::fmt::Debug;

use std::collections::BTreeMap;

use bimap::BiMap;

use lazy_static::lazy_static;

lazy_static! {
    pub static ref UNIT_PREFIXES: BiMap<&'static str, i8> = {
        let mut m = BiMap::new();
        m.insert("centi", -2);
        m.insert("deci", -1);
        m.insert("deca", 1);
        m.insert("", 0);
        m.insert("hecto", 2);
        m.insert("kilo", 3);
        m
    };
    pub static ref UNIT_PREFIXES_ABBR: BiMap<&'static str, i8> = {
        let mut m = BiMap::new();
        m.insert("c", -2);
        m.insert("d", -1);
        m.insert("de", 1);
        m.insert("", 0);
        m.insert("h", 2);
        m.insert("k", 3);
        m
    };
}

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
            BaseUnit::Gram => "g",
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
pub enum UnitDesc {
    Base([Ratio<i8>; 7]),
    Custom(BTreeMap<String, Ratio<u8>>),
}

#[derive(Clone)]
pub struct Unit {
    pub desc: UnitDesc,
    pub exp: i8,
    pub mult: f64,
}

impl Default for Unit {
    fn default() -> Self {
        Unit {
            desc: UnitDesc::Base([Ratio::zero(); 7]),
            exp: 0,
            mult: 1.0,
        }
    }
}

impl Debug for Unit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

impl Unit {
    pub fn empty() -> Self {
        Unit {
            desc: UnitDesc::Base([Ratio::zero(); 7]),
            exp: 0,
            mult: 1.0,
        }
    }

    pub fn pow(&self, rhs: i8) -> Self {
        let mut ret = self.clone();
        (0..rhs - 1).for_each(|_| ret = ret.clone() * self.clone());
        ret
    }
}

impl PartialEq for Unit {
    fn eq(&self, other: &Self) -> bool {
        match (self.desc.clone(), other.desc.clone()) {
            (UnitDesc::Base(a), UnitDesc::Base(b)) => a == b,
            _ => todo!(),
        }
    }
}

impl std::convert::TryFrom<&str> for Unit {
    type Error = &'static str;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        Ok({
            let (stripped, exp) = UNIT_PREFIXES
                .iter()
                .chain(UNIT_PREFIXES_ABBR.iter())
                .filter(|(p, _)| !p.is_empty())
                .find_map(|(prefix, exp)| {
                    s.trim()
                        .strip_prefix(prefix)
                        .map(|stripped| (stripped, exp))
                })
                .unwrap_or((s.trim(), &0));

            let base = match stripped {
                "meters" | "meter" | "m" => BaseUnit::Meter.into(),
                "grams" | "gram" | "g" | "gm" => BaseUnit::Gram.into(),
                "second" | "seconds" | "s" => BaseUnit::Second.into(),
                "amp" | "amps" | "ampere" | "amperes" => BaseUnit::Ampere.into(),
                "kelvin" | "K" => BaseUnit::Kelvin.into(),
                "moles" | "mols" | "mol" | "mole" | "M" => BaseUnit::Mole.into(),
                "candela" => BaseUnit::Candela.into(),
                "J" | "joules" => Unit {
                    desc: UnitDesc::Base([
                        Ratio::from(2),
                        Ratio::one(),
                        Ratio::from(-2),
                        Ratio::zero(),
                        Ratio::zero(),
                        Ratio::zero(),
                        Ratio::zero(),
                    ]),
                    exp: 0,
                    mult: 1.0,
                },
                "N" | "newtons" => Unit {
                    desc: UnitDesc::Base([
                        Ratio::one(),
                        Ratio::one(),
                        Ratio::from(-2),
                        Ratio::zero(),
                        Ratio::zero(),
                        Ratio::zero(),
                        Ratio::zero(),
                    ]),
                    exp: 3,
                    mult: 1.0,
                },
                _ => {
                    dbg!(s);
                    return Err("Bad unit");
                }
            };

            Unit {
                desc: base.desc,
                exp: exp + base.exp,
                mult: base.mult,
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
        let desc = UnitDesc::Base(arr);
        Unit {
            desc,
            exp: 0,
            mult: 1.0,
        }
    }
}

impl std::fmt::Display for Unit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.desc.clone() {
            UnitDesc::Base(arr) => {
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
            UnitDesc::Custom(_map) => {
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

        let desc = UnitDesc::Base([
            Ratio::one(),
            Ratio::one(),
            Ratio::zero(),
            Ratio::zero(),
            Ratio::zero(),
            Ratio::zero(),
            Ratio::zero(),
        ]);
        let u = Unit {
            desc,
            exp: 0,
            mult: 1.0,
        };
        assert_eq!(format!("{}", u).as_str(), "m g");

        let desc = UnitDesc::Base([
            Ratio::one(),
            Ratio::one() * 2,
            -Ratio::one(),
            Ratio::zero(),
            Ratio::zero(),
            Ratio::zero(),
            Ratio::zero(),
        ]);
        let u = Unit {
            desc,
            exp: 0,
            mult: 1.0,
        };
        assert_eq!(format!("{}", u).as_str(), "m g^2 s^-1");
    }
}

impl std::ops::Mul for Unit {
    type Output = Unit;

    fn mul(self, rhs: Self) -> Self::Output {
        match (self.desc, rhs.desc) {
            (UnitDesc::Base(a), UnitDesc::Base(b)) => {
                let mut res = [Ratio::zero(); 7];
                res.iter_mut()
                    .zip(a.iter().zip(b.iter()))
                    .for_each(|(r, (a, b))| {
                        *r = a + b;
                    });
                Unit {
                    desc: UnitDesc::Base(res),
                    exp: self.exp + rhs.exp,
                    mult: self.mult * rhs.mult,
                }
            }
            _ => todo!(),
        }
    }
}

impl std::ops::Div for Unit {
    type Output = Unit;

    fn div(self, rhs: Self) -> Self::Output {
        match (self.desc, rhs.desc) {
            (UnitDesc::Base(a), UnitDesc::Base(b)) => {
                let mut res = [Ratio::zero(); 7];
                res.iter_mut()
                    .zip(a.iter().zip(b.iter()))
                    .for_each(|(r, (a, b))| {
                        *r = a - b;
                    });
                Unit {
                    desc: UnitDesc::Base(res),
                    exp: self.exp - rhs.exp,
                    mult: self.mult / rhs.mult,
                }
            }
            _ => todo!(),
        }
    }
}
