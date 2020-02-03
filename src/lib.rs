mod bitwidth;

pub use crate::bitwidth::BitWidth;

#[macro_use]
mod unsigned;

#[macro_use]
mod prim;

include!(concat!(env!("OUT_DIR"), "/macrocalls.rs"));

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
