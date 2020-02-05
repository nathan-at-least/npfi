macro_rules! define_newtype_relations {
    ( bigger: $big:ident, smaller: $small:ident ) => {
        impl From<$small> for $big {
            fn from(small: $small) -> Self {
                $big::from_primitive(
                    small.into_primitive() as <$big as crate::FixedUnsigned>::PrimitiveContainer
                )
            }
        }

        impl std::convert::TryFrom<$big> for $small {
            type Error = crate::ConversionOverflow<$big>;

            fn try_from(big: $big) -> Result<Self, Self::Error> {
                if big <= $big::from(<$small as num_traits::Bounded>::max_value()) {
                    Ok($small::from_primitive(big.into_primitive()
                        as <$small as crate::FixedUnsigned>::PrimitiveContainer))
                } else {
                    Err(crate::ConversionOverflow {
                        target_size: <$small as crate::FixedUnsigned>::bit_width(),
                        source: big,
                    })
                }
            }
        }

        define_bitcontainerof_relation!(bigger: $big, smaller: $small);
    };
}

macro_rules! define_bitcontainerof_relation {
    ( bigger: $big:ident, smaller: $small:ident ) => {
        impl crate::BitContainerOf<$small> for $big {
            fn splice_in_bits_without_bounds_check(&mut self, field: $small, offset: usize) {
                self |= $big::from(field) << offset;
            }

            fn unsplice_bits_without_bounds_check(self, offset: usize) -> $small {
                let bigval = self >> offset & $big::from($small::max_value());
                $small::try_from(bigval)
                    .expect("impossible case: bigval must be small enough for small")
            }
        }
    };
}
