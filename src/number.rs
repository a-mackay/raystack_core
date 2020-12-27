use thiserror::Error;

/// A Haystack Number, encapsulating a scalar value and
/// an optional unit value. The unit is represented as a
/// string.
#[derive(Clone, Debug, PartialEq)]
pub struct Number {
    value: NumberValue,
    unit: Option<String>,
}

/// Represents the scalar portion of a Haystack Number.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NumberValue {
    Basic(f64),
    Exponent(f64, i32),
}

impl std::fmt::Display for NumberValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Basic(n) => write!(f, "{}", n),
            Self::Exponent(n, e) => write!(f, "{}e{}", n, e),
        }
    }
}

impl Number {
    /// Create a new `Number` with no exponent. If present, the unit should
    /// be a valid unit string from Project Haystack's
    /// unit database.
    pub fn new(value: f64, unit: Option<String>) -> Self {
        Self {
            value: NumberValue::Basic(value),
            unit,
        }
    }

    /// Create a new `Number` with an exponent. If present, the unit should
    /// be a valid unit string from Project Haystack's
    /// unit database.
    pub fn new_exponent(
        base: f64,
        exponent: i32,
        unit: Option<String>,
    ) -> Self {
        Self {
            value: NumberValue::Exponent(base, exponent),
            unit,
        }
    }

    /// Return the numeric component of this `Number`.
    pub fn value(&self) -> NumberValue {
        self.value
    }

    /// Return the unit component of this `Number`, if present.
    pub fn unit(&self) -> Option<&str> {
        self.unit.as_ref().map(|unit| unit.as_ref())
    }

    /// Parse a `Number` from a number encoded in a JSON string.
    /// # Example
    /// ```rust
    /// use raystack_core::Number;
    ///
    /// let n = Number::new(1.0, Some("pH".to_owned()));
    /// assert_eq!(Number::from_encoded_json_string("n:1.0 pH").unwrap(), n);
    /// ```
    pub fn from_encoded_json_string(
        json_string: &str,
    ) -> Result<Self, ParseNumberError> {
        let json_string = json_string.replacen("n:", "", 1);
        let mut split = json_string.trim().split(' ');
        let number_str = split
            .next()
            .ok_or_else(|| ParseNumberError::from_str(&json_string))?;
        let unit_str = split.next();
        let unit = unit_str.map(|unit_str| unit_str.trim().to_string());

        let mut split2 = number_str.trim().split('e');
        let base_num = split2
            .next()
            .ok_or_else(|| ParseNumberError::from_str(&json_string))?;
        let exp_num = split2.next();

        match exp_num {
            Some(exp_num) => {
                let base = base_num
                    .parse()
                    .map_err(|_| ParseNumberError::from_str(&json_string))?;
                let exp = exp_num
                    .parse()
                    .map_err(|_| ParseNumberError::from_str(&json_string))?;
                Ok(Number::new_exponent(base, exp, unit))
            }
            None => {
                let number = if number_str == "INF" {
                    std::f64::INFINITY
                } else if number_str == "-INF" {
                    std::f64::NEG_INFINITY
                } else {
                    number_str
                        .parse()
                        .map_err(|_| ParseNumberError::from_str(&json_string))?
                };
                Ok(Number::new(number, unit))
            }
        }

        // let number = if number_str == "INF" {
        //     std::f64::INFINITY
        // } else if number_str == "-INF" {
        //     std::f64::NEG_INFINITY
        // } else {
        //     number_str
        //         .parse()
        //         .map_err(|_| ParseNumberError::from_str(&json_string))?
        // };
        // let unit = unit_str.map(|unit_str| unit_str.trim().to_string());
        // Ok(Number::new(number, unit))
    }
}

impl std::fmt::Display for Number {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(unit) = self.unit() {
            write!(f, "{} {}", self.value(), unit)
        } else {
            write!(f, "{}", self.value().to_string())
        }
    }
}

/// Error denoting that a `Number` could not be parsed from a string.
#[derive(Clone, Debug, Eq, Error, PartialEq)]
#[error("Could not parse a Number from the string {unparsable_number}")]
pub struct ParseNumberError {
    unparsable_number: String,
}

impl ParseNumberError {
    pub(crate) fn from_str(s: &str) -> Self {
        Self {
            unparsable_number: s.to_string(),
        }
    }
}

#[cfg(test)]
mod test {
    use super::{Number, NumberValue};

    #[test]
    fn from_encoded_json_string() {
        let unitless = "n:45.5";
        assert_eq!(
            Number::from_encoded_json_string(unitless).unwrap().value(),
            NumberValue::Basic(45.5)
        );

        let unit = "n:73.2 °F";
        let number_with_unit = Number::from_encoded_json_string(unit).unwrap();
        assert_eq!(number_with_unit.value(), NumberValue::Basic(73.2));
        assert_eq!(number_with_unit.unit(), Some("°F"))
    }

    #[test]
    fn from_encoded_json_string_exponent() {
        let s = "n:1.23e+47";
        assert_eq!(
            Number::from_encoded_json_string(s).unwrap().value(),
            NumberValue::Exponent(1.23, 47)
        )
    }

    #[test]
    fn from_encoded_json_string_negative_exponent() {
        let s = "n:-1.23e-43";
        assert_eq!(
            Number::from_encoded_json_string(s).unwrap().value(),
            NumberValue::Exponent(-1.23, -43)
        )
    }

    #[test]
    fn from_encoded_json_string_exponent_with_unit() {
        let s = "n:1.23e+47 min";
        let n = Number::from_encoded_json_string(s).unwrap();
        assert_eq!(n.value(), NumberValue::Exponent(1.23, 47));
        assert_eq!(n.unit(), Some("min"))
    }

    #[test]
    fn from_encoded_json_string_negative_exponent_with_unit() {
        let s = "n:9e-45 min";
        let n = Number::from_encoded_json_string(s).unwrap();
        assert_eq!(n.value(), NumberValue::Exponent(9.0, -45));
        assert_eq!(n.unit(), Some("min"))
    }

    #[test]
    fn from_encoded_json_string_infinity() {
        let unitless = "n:INF";
        assert_eq!(
            Number::from_encoded_json_string(unitless).unwrap().value(),
            NumberValue::Basic(std::f64::INFINITY),
        );

        let unit = "n:INF °F";
        let number_with_unit = Number::from_encoded_json_string(unit).unwrap();
        assert_eq!(
            number_with_unit.value(),
            NumberValue::Basic(std::f64::INFINITY)
        );
        assert_eq!(number_with_unit.unit(), Some("°F"))
    }

    #[test]
    fn from_encoded_json_string_neg_infinity() {
        let unitless = "n:-INF";
        assert_eq!(
            Number::from_encoded_json_string(unitless).unwrap().value(),
            NumberValue::Basic(std::f64::NEG_INFINITY),
        );

        let unit = "n:-INF °F";
        let number_with_unit = Number::from_encoded_json_string(unit).unwrap();
        assert_eq!(
            number_with_unit.value(),
            NumberValue::Basic(std::f64::NEG_INFINITY)
        );
        assert_eq!(number_with_unit.unit(), Some("°F"))
    }

    fn number_to_f64(number: &Number) -> f64 {
        let val = number.value();
        match val {
            NumberValue::Basic(f) => f,
            _ => panic!("Expected a NumberValue::Basic"),
        }
    }

    #[test]
    fn from_encoded_json_string_signless_nan() {
        let unitless = "n:NaN";
        let float1 =
            number_to_f64(&Number::from_encoded_json_string(unitless).unwrap());
        assert!(float1.is_nan());

        let unit = "n:NaN °F";
        let number_with_unit = Number::from_encoded_json_string(unit).unwrap();
        let float2 = number_to_f64(&number_with_unit);
        assert!(float2.is_nan());
        assert_eq!(number_with_unit.unit(), Some("°F"))
    }

    #[test]
    fn from_encoded_json_string_signed_nan() {
        let unitless = "n:-NaN";
        let float1 =
            number_to_f64(&Number::from_encoded_json_string(unitless).unwrap());
        assert!(float1.is_nan());

        let unit = "n:+NaN °F";
        let number_with_unit = Number::from_encoded_json_string(unit).unwrap();
        let float2 = number_to_f64(&number_with_unit);
        assert!(float2.is_nan());
        assert_eq!(number_with_unit.unit(), Some("°F"))
    }
}
