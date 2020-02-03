macro_rules! define_unsigned {
    ( $usertype:ident, $containertype:ident, $usersize:expr, #[$doc:meta] ) => {
        #[$doc]
        #[allow(non_camel_case_types)]
        #[derive(
            PartialOrd,
            PartialEq,
            num_derive::Num,
            num_derive::NumOps,
            num_derive::One,
            num_derive::Zero,
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

        impl crate::BitWidth for $usertype {
            type PrimitiveContainer = $containertype;

            fn bit_width() -> usize {
                $usersize
            }

            fn into_primitive(self) -> Self::PrimitiveContainer {
                self.0
            }
        }
    };
}
