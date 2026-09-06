use std::vec::IntoIter;

pub trait EnumVariants {
    fn variants() -> &'static [Self]
    where
        Self: Sized;

    fn variant_count() -> usize
    where
        Self: Sized;
}
