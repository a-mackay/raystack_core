use crate::{Coord, Ref, NormalNumber, Number, NumberValue};
use serde_json::json;
use serde_json::Value;

#[derive(Debug)]
pub struct FromHaysonError {
    message: String
}

const KIND: &str = "_kind";

impl FromHaysonError {
    fn new(message: String) -> Self {
        Self {
            message,
        }
    }
}

fn error<T, M>(message: M) -> Result<T, FromHaysonError> where M: AsRef<str> {
    Err(FromHaysonError::new(message.as_ref().to_owned()))
}

fn error_opt<M>(message: M) -> Option<FromHaysonError> where M: AsRef<str> {
    Some(FromHaysonError::new(message.as_ref().to_owned()))
}

fn check_kind(target_kind: &str, value: &Value) -> Option<FromHaysonError> {
    match value.get(KIND) {
        Some(kind) => {
            match kind {
                Value::String(kind) => {
                    if kind == target_kind {
                        None
                    } else {
                        error_opt(format!("Expected '{}' = {} but found {}", KIND, kind, kind))
                    }
                },
                _ => error_opt(format!("'{}' key is not a string", KIND))
            }
        },
        None => error_opt(format!("Missing '{}' key", KIND)),
    }
}

/// Something which can be converted to and from Hayson
/// (the new JSON encoding used by Project Haystack).
trait Hayson: Sized {
    fn from_hayson(value: Value) -> Result<Self, FromHaysonError>;
    fn to_hayson(&self) -> Value;
}

impl Hayson for Coord {
    fn from_hayson(value: Value) -> Result<Self, FromHaysonError> {
        match &value {
            Value::Object(obj) => {
                if let Some(kind_err) = check_kind("coord", &value) {
                    return Err(kind_err);
                }
                let lat = obj.get("lat");
                let lng = obj.get("lng");

                if lat.is_none() {
                    return error("Coord lat is missing");
                }
                if lng.is_none() {
                    return error("Coord lng is missing");
                }

                let lat = lat.unwrap().as_f64();
                let lng = lng.unwrap().as_f64();

                if lat.is_none() {
                    return error("Coord lat is not a f64");
                }
                if lng.is_none() {
                    return error("Coord lng is not a f64");
                }

                let lat = lat.unwrap();
                let lng = lng.unwrap();

                Ok(Coord::new(lat, lng))
            },
            _ => error("Coord JSON value must be an object")
        }
    }

    fn to_hayson(&self) -> Value {
        json!({
            KIND: "coord",
            "lat": self.lat(),
            "lng": self.lng(),
        })
    }
}

impl Hayson for Ref {
    fn from_hayson(value: Value) -> Result<Self, FromHaysonError> {
        match &value {
            Value::Object(obj) => {
                if let Some(kind_err) = check_kind("ref", &value) {
                    return Err(kind_err);
                }
                // The ref string, without the preceding '@' sign:
                let val = obj.get("val");

                if val.is_none() {
                    return error("Ref val is missing");
                }

                let val = val.unwrap().as_str();
                if val.is_none() {
                    return error("Ref val is not a string");
                }

                let ref_str = format!("@{}", val.unwrap());

                if !Ref::is_valid_ref(&ref_str) {
                    return error(format!("Ref val is not valid: {}", ref_str));
                }

                Ok(Ref::new(ref_str).unwrap())
            },
            _ => error("Ref JSON value must be an object")
        }
    }

    fn to_hayson(&self) -> Value {
        json!({
            KIND: "ref",
            "val": self.to_axon_code().replacen("@", "", 1),
        })
    }
}

impl Hayson for Number {
    fn from_hayson(value: Value) -> Result<Self, FromHaysonError> {
        match &value {
            Value::Number(num) => {
                let float = num.as_f64();
                match float {
                    Some(float) => Ok(Number::new(float, None)),
                    None => error(format!("Number is not a f64: {}", num)),
                }
            },
            Value::Object(obj) => {
                if let Some(kind_err) = check_kind("number", &value) {
                    return Err(kind_err);
                }
                // The number, "INF", "-INF" or "NaN"
                let val = obj.get("val");
                if val.is_none() {
                    return error("Number val is missing");
                }
                let val = val.unwrap();

                let unit = obj.get("unit");
                if let Some(unit_val) = unit {
                    let unit_str = unit_val.as_str();
                    if unit_str.is_none() {
                        return error("Number unit is not a string");
                    }
                }
                let unit = unit.map(|unit_val| unit_val.as_str().unwrap().to_owned());

                match val {
                    Value::String(string) => {
                        match string.as_ref() {
                            "INF" => {
                                let num = NormalNumber::new(f64::INFINITY, unit);
                                Ok(Number::Normal(num))
                            },
                            "-INF" => {
                                let num = NormalNumber::new(f64::NEG_INFINITY, unit);
                                Ok(Number::Normal(num))
                            },
                            "NaN" => Ok(Number::Nan),
                            _ => error("Number val is a string but is not one of INF, -INF or NaN"),
                        }
                    },
                    Value::Number(num) => {
                        let float = num.as_f64();
                        match float {
                            Some(float) => Ok(Number::new(float, unit)),
                            None => error(format!("Number val is not a f64: {}", num)),
                        }
                    },
                    _ => error("Number val must be either a number or a string"),
                }
            },
            _ => error("Ref JSON value must be an object")
        }
    }

    fn to_hayson(&self) -> Value {
        let kind = "number";
        match self {
            Self::Normal(num) => {
                let unit = num.unit();
                match num.value() {
                    NumberValue::Basic(float) => {
                        if float.is_
                    },
                    NumberValue::Exponent(float, exp) => {
                        unimplemented!()
                    }
                }
            },
            Self::Nan => {
                json!({
                    KIND: kind,
                    "val": "NaN",
                })
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::Hayson;
    use crate::{Coord, Ref};

    #[test]
    fn serde_coord_works() {
        let coord = Coord::new(1.23, 4.56);
        let value = coord.to_hayson();
        let deserialized = Coord::from_hayson(value).unwrap();
        assert_eq!(coord, deserialized);
    }

    #[test]
    fn serde_ref_works() {
        let hsref = Ref::new("@abc".to_owned()).unwrap();
        let value = hsref.to_hayson();
        let deserialized = Ref::from_hayson(value).unwrap();
        assert_eq!(hsref, deserialized);
    }
}