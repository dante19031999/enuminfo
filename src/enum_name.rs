/// Trait for inspecting formatted and raw string names of enum variants.
///
/// This trait is automatically implemented when using `#[derive(EnumName)]`.
pub trait EnumName {
    /// Returns the formatted name of this variant, taking into account `rename` and `rename_all` attributes.
    fn name(&self) -> &'static str;

    /// Returns the exact identifier name of this variant as written in Rust code.
    fn raw_name(&self) -> &'static str;
}
