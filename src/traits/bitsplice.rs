use crate::FixedUnsigned;

pub struct SpliceOutOfBounds<C, F>
where
    C: BitContainerOf<F>,
    F: FixedUnsigned,
{
    container: C,
    field: F,
    offset: usize,
}

pub struct UnspliceOutOfBounds<C>
where
    C: FixedUnsigned,
{
    container: C,
    field_width: usize,
    offset: usize,
}

pub trait BitContainerOf<F>: FixedUnsigned
where
    F: FixedUnsigned,
{
    fn splice_in_bits_without_bounds_check(&mut self, field: F, offset: usize);

    fn splice_in_bits(
        &mut self,
        field: F,
        offset: usize,
    ) -> Result<(), SpliceOutOfBounds<Self, F>> {
        if self.bit_width() < field.bit_width() + offset {
            Err(SpliceOutOfBounds {
                container: self,
                field: field,
                offset: offset,
            })
        } else {
            self.splice_in_bits_without_bounds_check(field, offset);
            Ok(())
        }
    }

    fn splice_bits(mut self, field: F, offset: usize) -> Result<Self, SpliceOutOfBounds<Self, F>> {
        self.splice_in_bits(field, offset)?;
        Ok(self)
    }

    fn unsplice_bits_without_bounds_check(self, offset: usize) -> F;

    fn unsplice_bits(self, offset: usize) -> Result<F, UnspliceOutOfBounds<Self>> {
        if self.bit_width() < F::bit_width() + offset {
            Err(UnspliceOutOfBounds {
                container: self,
                field_width: F::bit_width(),
                offset: offset,
            })
        } else {
            Ok(self.unsplice_bits_without_bounds_check(offset))
        }
    }
}
