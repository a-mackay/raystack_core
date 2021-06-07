/// A Haystack XStr.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Xstr {
    type_name: String,
    value: String,
}

impl Xstr {
    /// Create a new `Xstr`.
    ///
    /// # Example
    /// ```rust
    /// use raystack_core::Xstr;
    /// let my_xstr = Xstr::new("Color".to_string(), "red".to_string());
    /// ```
    pub fn new(type_name: String, value: String) -> Self {
        Self { type_name, value }
    }

    pub fn type_name(&self) -> &str {
        &self.type_name
    }

    pub fn value(&self) -> &str {
        &self.value
    }

    /// Return this `Xstr` as Axon code.
    pub fn to_axon_code(&self) -> String {
        format!("xstr(\"{}\", \"{}\")", self.type_name(), self.value())
    }
}

impl std::fmt::Display for Xstr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}(\"{}\")", self.type_name(), self.value())
    }
}
