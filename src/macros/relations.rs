macro_rules! define_relations {
    ( bigger newtype $big:ident, smaller newtype $small:ident ) => {
        impl From<$small> for $big {
            fn from(small: $small) -> Self {
                $big(small.0 as <$big as crate::BitWidth>::PrimitiveContainer)
            }
        }

        impl std::convert::TryFrom<$big> for $small {
            type Error = crate::ConversionOverflow<$big>;

            fn try_from(big: $big) -> Result<Self, Self::Error> {
                if big <= $big::from(<$small as num_traits::Bounded>::max_value()) {
                    Ok($small(
                        big.0 as <$small as crate::BitWidth>::PrimitiveContainer,
                    ))
                } else {
                    Err(crate::ConversionOverflow {
                        target_size: <$small as crate::BitWidth>::bit_width(),
                        source: big,
                    })
                }
            }
        }
    };

    ( bigger prim $big:ident, smaller newtype $small:ident ) => {};

    ( bigger newtype $big:ident, smaller prim $small:ident ) => {};

    ( bigger prim $big:ident, smaller prim $small:ident ) => {};
}
