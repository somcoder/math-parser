use std::fmt::Display;
use std::ops::{Add, Div, Mul, Neg, Rem, Sub};

use std::{collections::HashMap, hash::Hash};

pub struct Converter<'a> {
    pub conversions: HashMap<(Unit, Unit), f64>,
    pub variables: HashMap<&'a str, Value>,
}

impl<'a> Converter<'a> {
    /// Initializes the converter.
    pub fn new(conversions: HashMap<(Unit, Unit), f64>) -> Self {
        let mut conversions = conversions;
        
        // Make the conversions more exaustive.
        Self::expand_conversions(&mut conversions);

        Self {
            conversions,
            variables: HashMap::new(),
        }
    }

    /// Addition.
    pub fn add(&self, left: Value, mut right: Value) -> Value {
        if left.unit != right.unit {
            right = self.convert(right, &left.unit.clone().unwrap_or_default());
        }

        self.percent(&left, &mut right);
        left + right
    }

    /// Subtraction
    pub fn sub(&self, left: Value, mut right: Value) -> Value {
        if left.unit != right.unit {
            right = self.convert(right, &left.unit.clone().unwrap_or_default());
        }

        self.percent(&left, &mut right);
        left - right
    }

    /// Multiplication
    pub fn mul(&self, left: Value, mut right: Value) -> Value {
        if left.unit != right.unit {
            right = self.convert(right, &left.unit.clone().unwrap_or_default());
        }

        self.percent(&left, &mut right);
        left * right
    }

    /// Division
    pub fn div(&self, left: Value, mut right: Value) -> Value {
        if left.unit != right.unit {
            right = self.convert(right, &left.unit.clone().unwrap_or_default());
        }

        self.percent(&left, &mut right);
        left / right
    }

    /// Modulo
    pub fn rem(&self, left: Value, mut right: Value) -> Value {
        if left.unit != right.unit {
            right = self.convert(right, &left.unit.clone().unwrap_or_default());
        }

        self.percent(&left, &mut right);
        left % right
    }

    fn percent(&self, left: &Value, right: &mut Value) {
        if right.unit == Some(Unit::new("%")) {
            right.number = left.number * right.number;
        }
    }

    pub fn convert(&self, from: Value, to_unit: &Unit) -> Value {
        let mut value = Value {
            number: from.number,
            unit: from.unit.clone(),
        };

        if from.unit.is_none() {
            return value;
        }

        let from_unit = from.unit.clone().unwrap();
        let to_unit = to_unit.clone();

        // Special case for percentages.
        if from_unit.0 == String::from("%") {
            return Value {
                number: from.number / 100.,
                unit: Some(Unit::new("%")),
            };
        }

        if from_unit == to_unit {
            return value;
        }

        let key = (from_unit, to_unit);
        if let Some(ratio) = self.conversions.get(&key) {
            value = Value {
                number: from.number * ratio,
                unit: Some(key.1),
            };
        }

        value
    }

    fn expand_conversions(conversions: &mut HashMap<(Unit, Unit), f64>) {
        let mut temp = HashMap::new();
        for ((from, to), ratio) in conversions.iter() {
            temp.insert((to.clone(), from.clone()), 1. / ratio);
        }

        conversions.extend(temp);

        let mut temp: HashMap<(Unit, Unit), f64> = HashMap::new();
        for ((left_from, left_to), left_ratio) in conversions.iter() {
            for ((right_from, right_to), right_ratio) in conversions.iter() {
                let key = (left_from.clone(), right_to.clone());
                if left_to == right_from && !conversions.contains_key(&key) {
                    if left_from == right_to {
                        continue;
                    }

                    temp.insert(key, left_ratio * right_ratio);
                }
            }
        }

        conversions.extend(temp);
    }
}

#[derive(Debug, Hash, Clone)]
pub struct Unit(pub String);

impl Unit {
    pub fn new(unit_str: &'static str) -> Unit {
        Self(String::from(unit_str))
    }
}

impl Display for Unit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.0.as_str())
    }
}

impl Default for Unit {
    fn default() -> Self {
        Self(String::default())
    }
}

impl PartialEq for Unit {
    fn eq(&self, other: &Self) -> bool {
        self.0.to_lowercase() == other.0.to_lowercase()
    }
}

impl Eq for Unit {}

pub struct Value {
    pub number: f64,
    pub unit: Option<Unit>,
}

impl Value {
    pub fn new(number: f64, unit: Unit) -> Value {
        Self {
            number,
            unit: Some(unit),
        }
    }

    pub fn simple(number: f64, unit_str: String) -> Self {
        Self {
            number,
            unit: Some(Unit(unit_str)),
        }
    }

    pub fn unitless(number: f64) -> Self {
        Self { number, unit: None }
    }

    pub fn power(self, pow: f64) -> Self {
        Self {
            number: self.number.powf(pow),
            unit: self.unit,
        }
    }
}

impl Add for Value {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            number: self.number + rhs.number,
            unit: self.unit,
        }
    }
}

impl Sub for Value {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            number: self.number - rhs.number,
            unit: self.unit,
        }
    }
}

impl Mul for Value {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            number: self.number * rhs.number,
            unit: self.unit,
        }
    }
}

impl Div for Value {
    type Output = Self;
    fn div(self, rhs: Self) -> Self::Output {
        Self {
            number: self.number / rhs.number,
            unit: self.unit,
        }
    }
}

impl Rem for Value {
    type Output = Self;
    fn rem(self, rhs: Self) -> Self::Output {
        Self {
            number: self.number % rhs.number,
            unit: self.unit,
        }
    }
}

// We may not need this, but let's see first! :)
impl Neg for Value {
    type Output = Self;
    fn neg(self) -> Self::Output {
        println!("Negating works: {}", self.number);
        Self {
            number: -self.number,
            unit: self.unit,
        }
    }
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let unit = self.unit.clone().unwrap_or_default();
        f.write_fmt(format_args!("{}{}", self.number, unit))
    }
}
