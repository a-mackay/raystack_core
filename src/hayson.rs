use crate::{Coord, Marker, Na, Number, RemoveMarker, Ref, Symbol};
use serde_json::json;
use serde_json::Value;

#[derive(Debug)]
pub struct FromHaysonError {
    message: String
}

const KIND: &str = "_kind";

impl FromHaysonError {
    pub fn new(message: String) -> Self {
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
pub trait Hayson: Sized {
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

                // Unit must be null or a string:
                let mut unit = obj.get("unit");
                if let Some(unit_val) = unit {
                    if unit_val.is_null() {
                        unit = None;
                    }
                }
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
                                let num = Number::new(f64::INFINITY, unit);
                                Ok(num)
                            },
                            "-INF" => {
                                let num = Number::new(f64::NEG_INFINITY, unit);
                                Ok(num)
                            },
                            "NaN" => {
                                let num = Number::new(f64::NAN, None);
                                Ok(num)
                            },
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
        let value = self.value();
        if value.is_nan() {
            json!({
                KIND: kind,
                "val": "NaN",
            })
        } else if value.is_infinite() && value.is_sign_positive() {
            json!({
                KIND: kind,
                "val": "INF",
                "unit": self.unit(),
            })
        } else if value.is_infinite() && value.is_sign_negative() {
            json!({
                KIND: kind,
                "val": "-INF",
                "unit": self.unit(),
            })
        } else {
                json!({
                    KIND: kind,
                    "val": value,
                    "unit": self.unit(),
                })
        }
    }
}

impl Hayson for Symbol {
    fn from_hayson(value: Value) -> Result<Self, FromHaysonError> {
        match &value {
            Value::Object(obj) => {
                if let Some(kind_err) = check_kind("symbol", &value) {
                    return Err(kind_err);
                }
                let val = obj.get("val");

                if val.is_none() {
                    return error("Symbol val is missing");
                }

                let val = val.unwrap().as_str();
                if val.is_none() {
                    return error("Symbol val is not a string");
                }

                let symbol_str = format!("^{}", val.unwrap());

                if !Symbol::is_valid_symbol(&symbol_str) {
                    return error(format!("Symbol val is not valid: {}", symbol_str));
                }

                Ok(Symbol::new(symbol_str).unwrap())
            },
            _ => error("Symbol JSON value must be an object")
        }
    }

    fn to_hayson(&self) -> Value {
        json!({
            KIND: "symbol",
            "val": self.to_axon_code().replacen("^", "", 1),
        })
    }
}

impl Hayson for Marker {
    fn from_hayson(value: Value) -> Result<Self, FromHaysonError> {
        match &value {
            Value::Object(_) => {
                match check_kind("marker", &value) {
                    Some(kind_err) => Err(kind_err),
                    None => Ok(Marker::new())
                }
            },
            _ => error("Marker JSON value must be an object")
        }
    }

    fn to_hayson(&self) -> Value {
        json!({
            KIND: "marker",
        })
    }
}

impl Hayson for RemoveMarker {
    fn from_hayson(value: Value) -> Result<Self, FromHaysonError> {
        match &value {
            Value::Object(_) => {
                match check_kind("remove", &value) {
                    Some(kind_err) => Err(kind_err),
                    None => Ok(RemoveMarker::new())
                }
            },
            _ => error("RemoveMarker JSON value must be an object")
        }
    }

    fn to_hayson(&self) -> Value {
        json!({
            KIND: "remove",
        })
    }
}

impl Hayson for Na {
    fn from_hayson(value: Value) -> Result<Self, FromHaysonError> {
        match &value {
            Value::Object(_) => {
                match check_kind("na", &value) {
                    Some(kind_err) => Err(kind_err),
                    None => Ok(Na::new())
                }
            },
            _ => error("NA JSON value must be an object")
        }
    }

    fn to_hayson(&self) -> Value {
        json!({
            KIND: "na",
        })
    }
}

#[cfg(test)]
mod test {
    use super::Hayson;
    use crate::{Coord, Marker, Na, Number, Ref, RemoveMarker, Symbol};

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

    #[test]
    fn serde_number_nan_works() {
        let num = Number::new(f64::NAN, None);
        let value = num.to_hayson();
        let deserialized = Number::from_hayson(value).unwrap();
        assert!(deserialized.value().is_nan());
        assert!(deserialized.unit().is_none())
    }

    #[test]
    fn serde_number_posinf_unitless_works() {
        let num = Number::new(f64::INFINITY, None);
        let value = num.to_hayson();
        let deserialized = Number::from_hayson(value).unwrap();
        assert_eq!(num, deserialized);
    }

    #[test]
    fn serde_number_posinf_units_works() {
        let num = Number::new(f64::INFINITY, Some("m/s".to_owned()));
        let value = num.to_hayson();
        let deserialized = Number::from_hayson(value).unwrap();
        assert_eq!(num, deserialized);
    }

    #[test]
    fn serde_number_neginf_unitless_works() {
        let num = Number::new(f64::NEG_INFINITY, None);
        let value = num.to_hayson();
        let deserialized = Number::from_hayson(value).unwrap();
        assert_eq!(num, deserialized);
    }

    #[test]
    fn serde_number_neginf_units_works() {
        let num = Number::new(f64::NEG_INFINITY, Some("m/s".to_owned()));
        let value = num.to_hayson();
        let deserialized = Number::from_hayson(value).unwrap();
        assert_eq!(num, deserialized);
    }

    #[test]
    fn serde_number_unitless_works() {
        let num = Number::new(1.23, None);
        let value = num.to_hayson();
        let deserialized = Number::from_hayson(value).unwrap();
        assert_eq!(num, deserialized);
    }

    #[test]
    fn serde_number_units_works() {
        let num = Number::new(1.23, Some("m/s".to_owned()));
        let value = num.to_hayson();
        let deserialized = Number::from_hayson(value).unwrap();
        assert_eq!(num, deserialized);
    }

    #[test]
    fn serde_symbol_works() {
        let sym = Symbol::new("^abc".to_owned()).unwrap();
        let value = sym.to_hayson();
        let deserialized = Symbol::from_hayson(value).unwrap();
        assert_eq!(sym, deserialized);
    }

    #[test]
    fn serde_marker_works() {
        let x = Marker::new();
        let value = x.to_hayson();
        let deserialized = Marker::from_hayson(value).unwrap();
        assert_eq!(x, deserialized);
    }

    #[test]
    fn serde_remove_marker_works() {
        let x = RemoveMarker::new();
        let value = x.to_hayson();
        let deserialized = RemoveMarker::from_hayson(value).unwrap();
        assert_eq!(x, deserialized);
    }

    #[test]
    fn serde_na_works() {
        let x = Na::new();
        let value = x.to_hayson();
        let deserialized = Na::from_hayson(value).unwrap();
        assert_eq!(x, deserialized);
    }
}