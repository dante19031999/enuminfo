/// Trait for accessing all variants of a unit enum as a static slice.
///
/// This trait is automatically implemented when using `#[derive(EnumVariants)]`.
pub trait EnumVariants {
    /// Returns a static slice of all variants of this enum (excluding variants marked with `#[enuminfo(ignore_variants)]`).
    fn variants() -> &'static [Self]
    where
        Self: Sized;

    /// Returns the total count of variants in [`EnumVariants::variants()`].
    fn variant_count() -> usize
    where
        Self: Sized;
}
