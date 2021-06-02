/// A Haystack marker.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Marker;

impl Marker {
    pub fn new() -> Self {
        Self
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
