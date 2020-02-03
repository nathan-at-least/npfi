mod bitwidth;
mod conversionoverflow;

pub use crate::bitwidth::BitWidth;
pub use crate::conversionoverflow::ConversionOverflow;

#[macro_use]
mod unsigned;

#[macro_use]
mod prim;

#[macro_use]
mod relations;

include!(concat!(env!("OUT_DIR"), "/macrocalls.rs"));

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
