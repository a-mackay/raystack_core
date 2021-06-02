use thiserror::Error;

/// A Haystack Number, encapsulating a scalar value and
/// an optional unit value. The unit is represented as a
/// string.
#[derive(Clone, Debug, PartialEq)]
pub struct Number {
    value: f64,
    unit: Option<String>,
}

impl Number {
    /// Create a new `Number`. If present, the unit should
    /// be a valid unit string from Project Haystack's
    /// unit database.
    pub fn new(value: f64, unit: Option<String>) -> Self {
        Self {
            value,
            unit,
        }
    }

    /// Return the numeric component of this `Number`.
    pub fn value(&self) -> f64 {
        self.value
    }

    /// Return the unit component of this `Number`, if present.
    pub fn unit(&self) -> Option<&str> {
         self.unit.as_ref().map(|unit| unit.as_ref())
    }
}

impl std::fmt::Display for Number {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = self.value();
        if value.is_nan() {
            write!(f, "NaN")
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
        } else {
            if let Some(unit) = self.unit() {
                write!(f, "{} {}", value, unit)
            } else {
                write!(f, "{}", value)
            }
        }
    }
}

// /// Error denoting that a `Number` could not be parsed from a string.
// #[derive(Clone, Debug, Eq, Error, PartialEq)]
// #[error("Could not parse a Number from the string {unparsable_number}")]
// pub struct ParseNumberError {
//     unparsable_number: String,
// }

// impl ParseNumberError {
//     pub(crate) fn from_str(s: &str) -> Self {
//         Self {
//             unparsable_number: s.to_string(),
//         }
//     }
// }