use num_traits::PrimInt;

pub trait FixedUnsigned: PrimInt {
    type PrimitiveContainer;

    fn bit_width() -> usize;
    fn into_primitive(self) -> Self::PrimitiveContainer;
    /// from_primitive *assumes* the parameter to be less than Self::max_value()
    fn from_primitive(p: Self::PrimitiveContainer) -> Self;
}
