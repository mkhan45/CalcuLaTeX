use num::rational::Ratio;
use num::{One, Zero};

use std::collections::BTreeMap;

enum UnitDim {
    Length,
    Mass,
    Time,
    Current,
    Temperature,
    Moles,
    Luminosity,
}

impl ToString for UnitDim {
    fn to_string(&self) -> String {
        match self {
            UnitDim::Length => "length",
            UnitDim::Mass => "mass",
            UnitDim::Time => "time",
            UnitDim::Current => "current",
            UnitDim::Temperature => "temperature",
            UnitDim::Moles => "moles",
            UnitDim::Luminosity => "luminosity",
        }
        .to_string()
    }
}

pub enum BaseUnit {
    Meter,
    Kilogram,
    Second,
    Ampere,
    Kelvin,
    Mole,
    Candela,
}

impl ToString for BaseUnit {
    fn to_string(&self) -> String {
        match self {
            BaseUnit::Meter => "meter",
            BaseUnit::Kilogram => "kilogram",
            BaseUnit::Second => "second",
            BaseUnit::Ampere => "ampere",
            BaseUnit::Kelvin => "kelvin",
            BaseUnit::Mole => "mole",
            BaseUnit::Candela => "candela",
        }
        .to_string()
    }
}

const BASE_UNITS: [BaseUnit; 7] = [
    BaseUnit::Meter,
    BaseUnit::Kilogram,
    BaseUnit::Second,
    BaseUnit::Ampere,
    BaseUnit::Kelvin,
    BaseUnit::Mole,
    BaseUnit::Candela,
];

#[derive(Debug, Clone)]
pub enum Unit {
    Base([Ratio<i8>; 7]),
    Custom(BTreeMap<String, Ratio<u8>>),
}

impl Unit {
    pub fn empty() -> Self {
        Unit::Base([Ratio::zero(); 7])
    }
}

impl From<BaseUnit> for Unit {
    fn from(b: BaseUnit) -> Self {
        let mut arr = [Ratio::zero(); 7];
        let index = match b {
            BaseUnit::Meter => 0,
            BaseUnit::Kilogram => 1,
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
            Unit::Custom(map) => {
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
        assert_eq!(format!("{}", u).as_str(), "meter");

        let u = Unit::Base([
            Ratio::one(),
            Ratio::one(),
            Ratio::zero(),
            Ratio::zero(),
            Ratio::zero(),
            Ratio::zero(),
            Ratio::zero(),
        ]);
        assert_eq!(format!("{}", u).as_str(), "meter kilogram");

        let u = Unit::Base([
            Ratio::one(),
            Ratio::one() * 2,
            -Ratio::one(),
            Ratio::zero(),
            Ratio::zero(),
            Ratio::zero(),
            Ratio::zero(),
        ]);
        assert_eq!(format!("{}", u).as_str(), "meter kilogram^2 second^-1");
    }
}
