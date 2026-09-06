/// Trait for constructing unit enum variants from string names.
///
/// This trait is automatically implemented when using `#[derive(EnumFromName)]`.
pub trait EnumFromName {
    /// Constructs an enum variant from its formatted name, respecting casing and rename overrides.
    ///
    /// Returns `None` if no matching variant is found or if the variant was marked with `#[enuminfo(ignore_from_name)]`.
    fn from_name(name: &str) -> Option<Self>
    where
        Self: Sized;

    /// Constructs an enum variant from its original identifier name as written in Rust code.
    ///
    /// Returns `None` if no matching variant is found.
    fn from_raw_name(name: &str) -> Option<Self>
    where
        Self: Sized;
}

/// Error returned when string parsing fails for an enum deriving [`EnumFromName`].
#[derive(Debug, Clone)]
pub struct EnumFromNameError {
    enum_class: String,
    enum_name: String,
}

impl EnumFromNameError {
    /// Creates a new `EnumFromNameError` with the enum type name and the invalid input name.
    pub fn new(enum_class: String, enum_name: String) -> EnumFromNameError {
        Self {
            enum_class,
            enum_name,
        }
    }
}

impl std::fmt::Display for EnumFromNameError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Invalid enum name \"{}\" for enum `{}`",
            self.enum_name, self.enum_class
        )
    }
}

impl std::error::Error for EnumFromNameError {}
