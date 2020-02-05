macro_rules! define_prim_wrappers {
    ( $t:ident, $bits:expr ) => {
        impl crate::FixedUnsigned for $t {
            type PrimitiveContainer = $t;

            fn bit_width() -> usize {
                $bits
            }

            fn into_primitive(self) -> Self::PrimitiveContainer {
                self
            }

            fn from_primitive(p: Self::PrimitiveContainer) -> Self {
                p
            }
        }
    };
}
