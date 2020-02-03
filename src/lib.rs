mod conversionoverflow;
mod traits;

pub use crate::conversionoverflow::ConversionOverflow;
pub use crate::traits::bitwidth::BitWidth;

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
