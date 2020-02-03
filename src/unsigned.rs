macro_rules! define_unsigned {
    ( $usertype:ident, $containertype:ident, $usersize:expr, #[$doc:meta] ) => {
        #[$doc]
        #[allow(non_camel_case_types)]
        pub struct $usertype($containertype);

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
