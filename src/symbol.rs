use regex::Regex;
use thiserror::Error;

/// A Haystack Symbol.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Symbol(String);

impl Symbol {
    /// Create a new `Symbol`.
    ///
    /// # Example
    /// ```rust
    /// use raystack_core::Symbol;
    /// let my_symbol = Symbol::new("^steam-boiler".to_string()).unwrap();
    /// ```
    pub fn new(s: String) -> Result<Self, ParseSymbolError> {
        if Self::is_valid_symbol(&s) {
            Ok(Self(s))
        } else {
            Err(ParseSymbolError::from_string(s))
        }
    }
    /// Return a Symbol by decoding a symbol which was encoded in a JSON string. In
    /// raw JSON strings, symbols are formatted with a `y:` prefix instead of
    /// an `^` character.
    ///
    /// # Example
    /// ```rust
    /// use raystack_core::Symbol;
    /// let json_str = "y:steam-boiler"; // ^steam-boiler
    /// let my_symbol = Symbol::from_encoded_json_string(json_str).unwrap();
    /// ```
    pub fn from_encoded_json_string(
        json_string: &str,
    ) -> Result<Self, ParseSymbolError> {
        Self::new(json_string.replacen("y:", "^", 1))
    }

    /// Return a string containing this symbol, encoded with a `y:` prefix instead
    /// of with an `^` character. This representation for symbols is used in raw
    /// JSON strings sent to and from a Haystack server.
    pub fn to_encoded_json_string(&self) -> String {
        self.0.replacen('^', "y:", 1)
    }

    /// Convert this symbol into a string.
    pub fn into_string(self) -> String {
        self.0
    }

    /// Return this symbol as an Axon symbol literal.
    pub fn to_axon_code(&self) -> &str {
        self.as_ref()
    }

    /// Return true if the string can be parsed as a valid symbol.
    pub(crate) fn is_valid_symbol(s: &str) -> bool {
        if !s.starts_with('^') {
            return false;
        }

        let s = &s[1..];
        let sections = s.split(':').collect::<Vec<_>>();

        if sections.len() > 2 {
            return false;
        }

        let first_section = sections[0];
        let second_section = sections.get(1);

        // Match things like `steam`, `steam-boiler`, `steam-boiler-2`, etc.
        let re =
            Regex::new(r"[a-z][a-zA-Z0-9_]*(-[a-z][a-zA-Z0-9_])*").unwrap();

        if !re.is_match(first_section) {
            return false;
        };

        if let Some(second_section) = second_section {
            if !re.is_match(second_section) {
                return false;
            };
        };

        true
    }
}

impl std::str::FromStr for Symbol {
    type Err = ParseSymbolError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if Self::is_valid_symbol(s) {
            Ok(Self(s.to_owned()))
        } else {
            let unparsable_symbol = s.to_owned();
            Err(ParseSymbolError { unparsable_symbol })
        }
    }
}

impl std::fmt::Display for Symbol {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_axon_code())
    }
}

impl std::convert::AsRef<str> for Symbol {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

/// An error indicating that a `Symbol` could not be parsed.
#[derive(Clone, Debug, Eq, Error, PartialEq)]
#[error("Could not parse a Symbol from the string {unparsable_symbol}")]
pub struct ParseSymbolError {
    unparsable_symbol: String,
}

impl ParseSymbolError {
    pub(crate) fn from_string(s: String) -> Self {
        Self {
            unparsable_symbol: s,
        }
    }
}

#[cfg(test)]
mod test {
    use super::Symbol;
    #[test]
    fn parse_symbol() {
        assert_eq!(Symbol::is_valid_symbol("^steam"), true);
        assert_eq!(Symbol::is_valid_symbol("^steam-boiler"), true);
        assert_eq!(Symbol::is_valid_symbol("^azAZ09"), true);
        assert_eq!(Symbol::is_valid_symbol("^azAZ09-azAZ09"), true);
        assert_eq!(Symbol::is_valid_symbol("^"), false);
        assert_eq!(Symbol::is_valid_symbol(""), false);
        assert_eq!(Symbol::is_valid_symbol("^steam:boiler"), true);
        assert_eq!(Symbol::is_valid_symbol("^steam-boiler:boiler-steam"), true);
        assert_eq!(Symbol::is_valid_symbol("^az0-az0-az0:az0-az0"), true);
        assert_eq!(Symbol::is_valid_symbol("^steam:boiler:another"), false);
        assert_eq!(Symbol::is_valid_symbol("^steam_-__boil_er"), true);
    }

    #[test]
    fn to_json_works() {
        let sym = Symbol::new("^steam-boiler".to_owned()).unwrap();
        assert_eq!(sym.to_encoded_json_string(), "y:steam-boiler");
    }

    #[test]
    fn from_json_works() {
        let sym = Symbol::new("^steam-boiler".to_owned()).unwrap();
        assert_eq!(
            Symbol::from_encoded_json_string("y:steam-boiler").unwrap(),
            sym
        );
    }

    #[test]
    fn to_str_works() {
        let sym = Symbol::new("^steam-boiler".to_owned()).unwrap();
        assert_eq!(format!("{}", sym), "^steam-boiler");
    }
}
