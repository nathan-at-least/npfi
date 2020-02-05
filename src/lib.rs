mod conversionoverflow;
mod traits;

pub use crate::conversionoverflow::ConversionOverflow;
pub use crate::traits::bitsplice::{BitContainerOf, SpliceOutOfBounds, UnspliceOutOfBounds};
pub use crate::traits::fixedunsigned::FixedUnsigned;

#[macro_use]
mod macros;

include!(concat!(env!("OUT_DIR"), "/macrocalls.rs"));

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
