#![allow(clippy::clippy::suspicious_arithmetic_impl)]

use num::rational::Ratio;
use num::{One, Zero};
use std::fmt::Debug;

use std::convert::TryFrom;
use std::convert::TryInto;

use std::collections::BTreeMap;

use bimap::BiMap;

use lazy_static::lazy_static;

lazy_static! {
    pub static ref UNIT_PREFIXES: BiMap<&'static str, i64> = {
        let mut m = BiMap::new();
        m.insert("centi", -2);
        m.insert("deci", -1);
        m.insert("deca", 1);
        m.insert("", 0);
        m.insert("hecto", 2);
        m.insert("kilo", 3);
        m
    };
    pub static ref UNIT_PREFIXES_ABBR: BiMap<&'static str, i64> = {
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

// The type of a Unit, e.g. grams or kilometers.
// --
// Base is an SI unit, derived or not.
// The array is a list of powers for each SI base unit.
// E.g. meters^2 / kelvin is [2, 0, 0, 0, -1, 0, 0]
// --
// Custom is not implemented yet, but I plan for users
// to be able to create custom units, in which case the
// map would just be [unit_name -> power]
#[derive(Clone, Debug)]
pub enum UnitDesc {
    Base([Ratio<i8>; 7]),
    Custom(BTreeMap<String, Ratio<u8>>),
}

impl PartialEq for UnitDesc {
    fn eq(&self, other: &Self) -> bool {
        match (&self, &other) {
            (UnitDesc::Base(a), UnitDesc::Base(b)) => a == b,
            _ => todo!(),
        }
    }
}

impl UnitDesc {
    pub fn is_empty(&self) -> bool {
        match self {
            UnitDesc::Base(a) => a == &[Ratio::zero(); 7],
            UnitDesc::Custom(_) => todo!(),
        }
    }
}

#[derive(Clone, PartialEq)]
pub struct Unit {
    pub desc: UnitDesc,
    pub exp: i64,
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

impl std::convert::TryFrom<&str> for Unit {
    type Error = &'static str;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        Ok({
            // Find if the unit starts with an SI prefix, in which case it should be
            // stripped and added to the exponent.
            // Unfortunately, some units start with an abbreviated SI prefix so they
            // have to be hardcoded out.
            let (stripped, exp) = if !matches!(s, "day" | "days" | "hours" | "hour") {
                UNIT_PREFIXES
                    .iter()
                    .chain(UNIT_PREFIXES_ABBR.iter())
                    .filter(|(p, _)| !p.is_empty())
                    .find_map(|(prefix, exp)| {
                        s.trim()
                            .strip_prefix(prefix)
                            .map(|stripped| (stripped, exp))
                    })
                    .unwrap_or((s.trim(), &0))
            } else {
                (s, &0)
            };

            let base = match stripped {
                "meters" | "meter" | "m" => BaseUnit::Meter.into(),
                "grams" | "gram" | "g" | "gm" => BaseUnit::Gram.into(),
                "second" | "seconds" | "s" => BaseUnit::Second.into(),
                "amp" | "amps" | "ampere" | "amperes" => BaseUnit::Ampere.into(),
                "kelvin" | "K" => BaseUnit::Kelvin.into(),
                "moles" | "mols" | "mol" | "mole" | "M" => BaseUnit::Mole.into(),
                "candela" => BaseUnit::Candela.into(),
                "J" | "joule" => Unit {
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
                "N" | "newton" => Unit {
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
                "minute" | "min" => Unit {
                    mult: 60.0,
                    ..BaseUnit::Second.into()
                },
                "hour" | "hours" => Unit {
                    mult: 60.0 * 60.0,
                    ..BaseUnit::Second.into()
                },
                "day" | "days" => Unit {
                    mult: 60.0 * 60.0 * 24.0,
                    ..BaseUnit::Second.into()
                },
                "year" | "years" => Unit {
                    mult: 60.0 * 60.0 * 24.0 * 365.0,
                    ..BaseUnit::Second.into()
                },
                "amu" => Unit {
                    mult: 1.6603145,
                    exp: -24,
                    ..BaseUnit::Gram.into()
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

impl TryFrom<String> for Unit {
    type Error = &'static str;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        s.as_str().try_into()
    }
}

impl TryFrom<&String> for Unit {
    type Error = &'static str;

    fn try_from(s: &String) -> Result<Self, Self::Error> {
        s.as_str().try_into()
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

// This is only used internally for testing,
// ToLatex handles the proper formatting
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::convert::TryFrom;

    #[test]
    fn to_string_base_unit() {
        assert_eq!(&BaseUnit::Meter.to_string(), "m");
        assert_eq!(&BaseUnit::Gram.to_string(), "g");
        assert_eq!(&BaseUnit::Second.to_string(), "s");
        assert_eq!(&BaseUnit::Ampere.to_string(), "A");
        assert_eq!(&BaseUnit::Kelvin.to_string(), "K");
        assert_eq!(&BaseUnit::Mole.to_string(), "M");
        assert_eq!(&BaseUnit::Candela.to_string(), "cd");
    }

    #[test]
    fn empty_unit() {
        let unit = Unit::empty();
        assert_eq!(unit.to_string(), "");
    }

    #[test]
    fn pow_unit_meter() {
        let unit = Unit::from(BaseUnit::Meter).pow(3);
        assert_eq!(unit.to_string(), "m^3");
    }

    #[test]
    fn pow_unit_gram() {
        let unit = Unit::from(BaseUnit::Gram).pow(3);
        assert_eq!(unit.to_string(), "g^3");
    }

    #[test]
    fn pow_unit_second() {
        let unit = Unit::from(BaseUnit::Second).pow(3);
        assert_eq!(unit.to_string(), "s^3");
    }

    #[test]
    fn pow_unit_ampere() {
        let unit = Unit::from(BaseUnit::Ampere).pow(3);
        assert_eq!(unit.to_string(), "A^3");
    }

    #[test]
    fn pow_unit_kelvin() {
        let unit = Unit::from(BaseUnit::Kelvin).pow(3);
        assert_eq!(unit.to_string(), "K^3");
    }

    #[test]
    fn pow_unit_mole() {
        let unit = Unit::from(BaseUnit::Mole).pow(3);
        assert_eq!(unit.to_string(), "M^3");
    }

    #[test]
    fn pow_unit_candela() {
        let unit = Unit::from(BaseUnit::Candela).pow(3);
        assert_eq!(unit.to_string(), "cd^3");
    }

    #[test]
    fn partial_eq_unit_base_success() {
        let unit1 = Unit::from(BaseUnit::Meter);
        let unit2 = Unit::from(BaseUnit::Meter);
        assert_eq!(unit1, unit2);
    }

    #[test]
    fn partial_eq_unit_base_failure() {
        let unit1 = Unit::from(BaseUnit::Meter);
        let unit2 = Unit::from(BaseUnit::Ampere);
        assert_ne!(unit1, unit2);
    }

    #[test]
    fn partial_eq_unit_pow_success() {
        let unit1 = Unit::from(BaseUnit::Meter).pow(3);
        let unit2 = Unit::from(BaseUnit::Meter).pow(3);
        assert_eq!(unit1, unit2);
    }

    #[test]
    fn partial_eq_unit_pow_failure() {
        let unit1 = Unit::from(BaseUnit::Meter).pow(2);
        let unit2 = Unit::from(BaseUnit::Meter).pow(3);
        assert_ne!(unit1, unit2);
    }

    #[test]
    fn try_from_unit_meter() {
        let unit = Unit::try_from("meter").unwrap();
        assert_eq!(unit.to_string(), "m");
    }

    #[test]
    fn try_from_unit_gram() {
        let unit = Unit::try_from("gram").unwrap();
        assert_eq!(unit.to_string(), "g");
    }

    #[test]
    fn try_from_unit_second() {
        let unit = Unit::try_from("second").unwrap();
        assert_eq!(unit.to_string(), "s");
    }

    #[test]
    fn try_from_unit_ampere() {
        let unit = Unit::try_from("ampere").unwrap();
        assert_eq!(unit.to_string(), "A");
    }

    #[test]
    fn try_from_unit_mole() {
        let unit = Unit::try_from("mole").unwrap();
        assert_eq!(unit.to_string(), "M");
    }

    #[test]
    fn try_from_unit_joule() {
        let unit1 = Unit::try_from("joule").unwrap();
        assert_eq!(unit1.to_string(), "m^2 g s^-2");

        let unit2 = Unit::try_from("J").unwrap();
        assert_eq!(unit2.to_string(), "m^2 g s^-2");
    }

    #[test]
    fn try_from_unit_newton() {
        let unit1 = Unit::try_from("newton").unwrap();
        assert_eq!(unit1.to_string(), "m g s^-2");

        let unit2 = Unit::try_from("N").unwrap();
        assert_eq!(unit2.to_string(), "m g s^-2");
    }

    #[test]
    fn try_from_unit_minute() {
        let unit1 = Unit::try_from("minute").unwrap();
        assert_eq!(unit1.to_string(), "s");

        let unit2 = Unit::try_from("min").unwrap();
        assert_eq!(unit2.to_string(), "s");
    }

    #[test]
    fn try_from_unit_hour() {
        let unit1 = Unit::try_from("hour").unwrap();
        assert_eq!(unit1.to_string(), "s");

        let unit2 = Unit::try_from("hours").unwrap();
        assert_eq!(unit2.to_string(), "s");
    }

    #[test]
    fn try_from_unit_day() {
        let unit1 = Unit::try_from("day").unwrap();
        assert_eq!(unit1.to_string(), "s");

        let unit2 = Unit::try_from("days").unwrap();
        assert_eq!(unit2.to_string(), "s");
    }

    #[test]
    fn try_from_unit_year() {
        let unit1 = Unit::try_from("year").unwrap();
        assert_eq!(unit1.to_string(), "s");

        let unit2 = Unit::try_from("years").unwrap();
        assert_eq!(unit2.to_string(), "s");
    }

    #[test]
    fn mult_meter() {
        let unit = Unit::try_from("meters").unwrap();
        let out = (unit.clone() * unit).to_string();
        assert_eq!(out, "m^2");
    }

    #[test]
    fn mult_gram() {
        let unit = Unit::try_from("grams").unwrap();
        let out = (unit.clone() * unit).to_string();
        assert_eq!(out, "g^2");
    }

    #[test]
    fn mult_meters_grams() {
        let meters = Unit::try_from("meters").unwrap();
        let grams = Unit::try_from("grams").unwrap();
        let result = (meters * grams).to_string();
        assert_eq!(result, "m g");
    }

    #[test]
    fn mult_second() {
        let unit = Unit::try_from("seconds").unwrap();
        let out = (unit.clone() * unit).to_string();
        assert_eq!(out, "s^2");
    }

    #[test]
    fn mult_ampere() {
        let unit = Unit::try_from("amperes").unwrap();
        let out = (unit.clone() * unit).to_string();
        assert_eq!(out, "A^2");
    }

    #[test]
    fn mult_ampere_second() {
        let amperes = Unit::try_from("amperes").unwrap();
        let second = Unit::try_from("second").unwrap();
        let result = (amperes * second).to_string();
        assert_eq!(result, "s A");
    }

    #[test]
    fn mult_mole() {
        let unit = Unit::try_from("moles").unwrap();
        let out = (unit.clone() * unit).to_string();
        assert_eq!(out, "M^2");
    }

    #[test]
    fn mult_joule() {
        let unit = Unit::try_from("joule").unwrap();
        let out = (unit.clone() * unit).to_string();
        assert_eq!(out, "m^4 g^2 s^-4");
    }

    #[test]
    fn mult_joule_second() {
        let joules = Unit::try_from("joule").unwrap();
        let second = Unit::try_from("second").unwrap();
        let result = (joules * second).to_string();
        assert_eq!(result, "m^2 g s^-1");
    }

    #[test]
    fn mult_newton() {
        let unit = Unit::try_from("newton").unwrap();
        let out = (unit.clone() * unit).to_string();
        assert_eq!(out, "m^2 g^2 s^-4");
    }

    #[test]
    fn div_meter_gram() {
        let mut unit1 = Unit::try_from("meters").unwrap();
        let mut unit2 = Unit::try_from("grams").unwrap();
        assert_eq!((unit1 / unit2).to_string(), "m g^-1");
    }

    #[test]
    fn div_joule_meter() {
        let mut unit1 = Unit::try_from("joule").unwrap();
        let mut unit2 = Unit::try_from("meter").unwrap();
        assert_eq!((unit1 / unit2).to_string(), "m g s^-2");
    }

    #[test]
    fn div_joule_second() {
        let mut unit1 = Unit::try_from("joule").unwrap();
        let mut unit2 = Unit::try_from("second").unwrap();
        assert_eq!((unit1 / unit2).to_string(), "m^2 g s^-3");
    }

    #[test]
    fn div_ampere_second() {
        let mut unit1 = Unit::try_from("ampere").unwrap();
        let mut unit2 = Unit::try_from("second").unwrap();
        assert_eq!((unit1 / unit2).to_string(), "s^-1 A");
    }

    #[test]
    fn div_ampere_ampere() {
        let mut unit1 = Unit::try_from("ampere").unwrap();
        let mut unit2 = Unit::try_from("ampere").unwrap();
        assert_eq!((unit1 / unit2).to_string(), "");
    }

    #[test]
    fn div_meter_meter() {
        let mut unit1 = Unit::try_from("meters").unwrap();
        let mut unit2 = Unit::try_from("meters").unwrap();
        assert_eq!((unit1 / unit2).to_string(), "");
    }

    #[test]
    fn div_gram_gram() {
        let mut unit1 = Unit::try_from("grams").unwrap();
        let mut unit2 = Unit::try_from("grams").unwrap();
        assert_eq!((unit1 / unit2).to_string(), "");
    }
}
