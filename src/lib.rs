mod coord;
mod hsref;
mod number;
mod qname;
mod symbol;
mod tag;

pub use coord::Coord;
pub use hsref::{ParseRefError, Ref};
pub use number::{Number, ParseNumberError};
pub use qname::Qname;
pub use symbol::{ParseSymbolError, Symbol};
pub use tag::{is_tag_name, ParseTagNameError, TagName};

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
