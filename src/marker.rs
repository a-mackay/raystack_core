/// A Haystack marker.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Marker;

impl Marker {
    pub fn new() -> Self {
        Self
    }
}

impl std::default::Default for Marker {
    fn default() -> Self {
        Self::new()
    }
}

/// A Haystack remove marker.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RemoveMarker;

impl RemoveMarker {
    pub fn new() -> Self {
        Self
    }
}

impl std::default::Default for RemoveMarker {
    fn default() -> Self {
        Self::new()
    }
}
