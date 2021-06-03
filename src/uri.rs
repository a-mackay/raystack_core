/// A Haystack Uri.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Uri(String);

impl Uri {
    /// Create a new `Uri`.
    ///
    /// # Example
    /// ```rust
    /// use raystack_core::Uri;
    /// let my_uri = Uri::new("http://skyspark.company.com".to_string());
    /// ```
    pub fn new(s: String) -> Self {
        Uri(s)
    }


    /// Convert this Uri into a string.
    pub fn into_string(self) -> String {
        self.0
    }

    /// Return this uri as an Axon Uri literal.
    pub fn to_axon_code(&self) -> String {
        format!("`{}`", self.0)
    }
}

impl std::fmt::Display for Uri {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_axon_code())
    }
}

impl std::convert::AsRef<str> for Uri {
    fn as_ref(&self) -> &str {
        &self.0
    }
}