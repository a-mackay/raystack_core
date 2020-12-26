#[derive(Clone, Debug, PartialEq)]
pub struct Qname(String);

/// A qualified name, like `core::parseNumber`.
impl Qname {
    pub fn new(s: String) -> Self {
        Self(s)
    }

    /// Return a string slice containing the contents of this `Qname`.
    pub fn qname(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Display for Qname {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.qname())
    }
}
