/// A Haystack number.
#[derive(Clone, Debug, PartialEq)]
pub enum Number {
    Basic(BasicNumber),
    Scientific(ScientificNumber),
}

impl std::fmt::Display for Number {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Basic(num) => write!(f, "{}", num),
            Self::Scientific(ex) => write!(f, "{}", ex),
        }
    }
}

impl Number {
    /// Create a new `Number`. If present, the unit should
    /// be a valid unit string from Project Haystack's
    /// unit database.
    pub fn new(value: f64, unit: Option<String>) -> Self {
        Number::Basic(BasicNumber::new(value, unit))
    }

    /// Create a new `Number` and no unit.
    pub fn new_unitless(value: f64) -> Self {
        Self::new(value, None)
    }

    /// Create a new scientific notation `Number`. If present, the unit should
    /// be a valid unit string from Project Haystack's
    /// unit database.
    pub fn new_scientific(
        significand: f64,
        exponent: i32,
        unit: Option<String>,
    ) -> Option<Self> {
        Some(Number::Scientific(ScientificNumber::new(
            significand,
            exponent,
            unit,
        )?))
    }

    /// Create a new scientific notation `Number` and no unit.
    pub fn new_scientific_unitless(
        significand: f64,
        exponent: i32,
    ) -> Option<Self> {
        Self::new_scientific(significand, exponent, None)
    }

    /// If this represents a non-scientific notation number, return the number.
    pub fn as_number(&self) -> Option<&BasicNumber> {
        match self {
            Self::Basic(number) => Some(number),
            _ => None,
        }
    }

    /// If this represents a number in scientific notation,
    /// return the scientific notation number.
    pub fn as_scientific_number(&self) -> Option<&ScientificNumber> {
        match self {
            Self::Scientific(ex) => Some(ex),
            _ => None,
        }
    }

    /// Return the unit component of this `Number`, if present.
    pub fn unit(&self) -> Option<&str> {
        match self {
            Self::Basic(num) => num.unit(),
            Self::Scientific(ex) => ex.unit(),
        }
    }

    /// Return a string containing Axon code representing this number.
    pub fn to_axon_code(&self) -> String {
        match self {
            Self::Basic(num) => num.to_axon_code(),
            Self::Scientific(ex) => ex.to_axon_code(),
        }
    }
}

/// A Haystack Number, encapsulating a scalar value and
/// an optional unit value. The unit is represented as a
/// string. This does not represent Haystack scientific notation numbers.
#[derive(Clone, Debug, PartialEq)]
pub struct BasicNumber {
    value: f64,
    unit: Option<String>,
}

impl BasicNumber {
    /// Create a new `BasicNumber`. If present, the unit should
    /// be a valid unit string from Project Haystack's
    /// unit database.
    pub fn new(value: f64, unit: Option<String>) -> Self {
        Self { value, unit }
    }

    /// Create a new `BasicNumber` with no unit.
    pub fn new_unitless(value: f64) -> Self {
        Self::new(value, None)
    }

    /// Return the numeric component of this number.
    pub fn value(&self) -> f64 {
        self.value
    }

    /// Return the unit component of this number, if present.
    pub fn unit(&self) -> Option<&str> {
        self.unit.as_ref().map(|unit| unit.as_ref())
    }

    /// Return a string containing Axon code representing this number.
    pub fn to_axon_code(&self) -> String {
        let value = self.value();
        if let Some(unit) = self.unit() {
            if value.is_nan() {
                format!("nan().as(\"{}\")", unit)
            } else if value.is_infinite() && value.is_sign_positive() {
                format!("posInf().as(\"{}\")", unit)
            } else if value.is_infinite() && value.is_sign_negative() {
                format!("negInf().as(\"{}\")", unit)
            } else {
                format!("{}{}", value, unit)
            }
        } else {
            if value.is_nan() {
                "nan()".to_owned()
            } else if value.is_infinite() && value.is_sign_positive() {
                "posInf()".to_owned()
            } else if value.is_infinite() && value.is_sign_negative() {
                "negInf()".to_owned()
            } else {
                format!("{}", value)
            }
        }
    }
}

impl std::fmt::Display for BasicNumber {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = self.value();
        if value.is_nan() {
            if let Some(unit) = self.unit() {
                write!(f, "NaN {}", unit)
            } else {
                write!(f, "NaN")
            }
        } else if value.is_infinite() && value.is_sign_positive() {
            if let Some(unit) = self.unit() {
                write!(f, "INF {}", unit)
            } else {
                write!(f, "INF")
            }
        } else if value.is_infinite() && value.is_sign_negative() {
            if let Some(unit) = self.unit() {
                write!(f, "-INF {}", unit)
            } else {
                write!(f, "-INF")
            }
        } else if let Some(unit) = self.unit() {
            write!(f, "{} {}", value, unit)
        } else {
            write!(f, "{}", value)
        }
    }
}

/// A Haystack scientific notation Number, encapsulating a value and
/// an optional unit value. The unit is represented as a
/// string.
#[derive(Clone, Debug, PartialEq)]
pub struct ScientificNumber {
    significand: f64,
    exponent: i32,
    unit: Option<String>,
}

impl ScientificNumber {
    /// Create a new `ScientificNumber`. If present, the unit should
    /// be a valid unit string from Project Haystack's
    /// unit database. The significand must be a finite number which is
    /// not NaN.
    pub fn new(
        significand: f64,
        exponent: i32,
        unit: Option<String>,
    ) -> Option<Self> {
        if significand.is_nan() || significand.is_infinite() {
            None
        } else {
            Some(Self {
                significand,
                exponent,
                unit,
            })
        }
    }

    /// Create a new `ScientificNumber` with no unit. The significand must
    /// be a finite number which is not NaN.
    pub fn new_unitless(significand: f64, exponent: i32) -> Option<Self> {
        Self::new(significand, exponent, None)
    }

    /// Return the numeric significand component of this number.
    pub fn significand(&self) -> f64 {
        self.significand
    }

    /// Return the numeric exponent component of this number.
    pub fn exponent(&self) -> i32 {
        self.exponent
    }

    /// Return the unit component of this number, if present.
    pub fn unit(&self) -> Option<&str> {
        self.unit.as_ref().map(|unit| unit.as_ref())
    }

    /// Return a string containing Axon code representing this number.
    pub fn to_axon_code(&self) -> String {
        let exp = self.exponent();
        let sig = self.significand();
        if let Some(unit) = self.unit() {
            format!("{}e{}{}", sig, exp, unit)
        } else {
            format!("{}e{}", sig, exp)
        }
    }
}

impl std::fmt::Display for ScientificNumber {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let exp = self.exponent();
        let sig = self.significand();
        if let Some(unit) = self.unit() {
            write!(f, "{}e{} {}", sig, exp, unit)
        } else {
            write!(f, "{}e{}", sig, exp)
        }
    }
}
