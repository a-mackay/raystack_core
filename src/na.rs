/// A Haystack NA (not available).
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Na;

impl Na {
    pub fn new() -> Self {
        Self
    }
}

impl std::default::Default for Na {
    fn default() -> Self {
        Self::new()
    }
}
