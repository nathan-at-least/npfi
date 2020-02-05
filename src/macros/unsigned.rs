macro_rules! define_unsigned {
    ( $usertype:ident, $containertype:ident, $usersize:expr, #[$doc:meta] ) => {
        #[$doc]
        #[allow(non_camel_case_types)]
        #[derive(
            num_derive::Num,
            num_derive::NumCast,
            num_derive::NumOps,
            num_derive::One,
            num_derive::Zero,
            PartialEq,
            PartialOrd,
        )]
        pub struct $usertype($containertype);

        impl num_traits::Bounded for $usertype {
            fn min_value() -> Self {
                $usertype(0)
            }

            fn max_value() -> Self {
                $usertype(1 << $usersize - 1)
            }
        }

        impl crate::FixedUnsigned for $usertype {
            type PrimitiveContainer = $containertype;

            fn bit_width() -> usize {
                $usersize
            }

            fn into_primitive(self) -> Self::PrimitiveContainer {
                self.0
            }

            fn from_primitive(p: Self::PrimitiveContainer) -> Self {
                $usertype(p)
            }
        }
    };
}
