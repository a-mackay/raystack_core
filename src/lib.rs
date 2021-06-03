mod coord;
#[cfg(feature = "json")]
mod hayson;
mod hsref;
mod marker;
mod na;
mod number;
mod qname;
mod symbol;
mod tag;
mod uri;
mod xstr;

pub use coord::Coord;
pub use hayson::{Hayson, FromHaysonError};
pub use hsref::{ParseRefError, Ref};
pub use marker::{Marker, RemoveMarker};
pub use na::Na;
pub use number::{Number};
pub use qname::Qname;
pub use symbol::{ParseSymbolError, Symbol};
pub use tag::{is_tag_name, ParseTagNameError, TagName};
pub use uri::Uri;
pub use xstr::Xstr;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
