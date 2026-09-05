pub trait EnumVariants {
    fn variants(&self) -> &'static [Self]
    where
        Self: Sized;

    fn variant_count(&self) -> usize;
}
