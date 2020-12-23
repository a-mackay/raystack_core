mod coord;
mod hsref;
mod number;
mod tag;

pub use coord::Coord;
pub use hsref::{ParseRefError, Ref};
pub use number::{ParseNumberError, Number};
pub use tag::{is_tag_name, ParseTagNameError, TagName};

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
