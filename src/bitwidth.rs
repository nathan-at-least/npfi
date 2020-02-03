pub trait BitWidth {
    type PrimitiveContainer;

    fn bit_width() -> usize;
    fn into_primitive(self) -> Self::PrimitiveContainer;
}
