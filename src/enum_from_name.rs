pub trait EnumFromName {
    fn from_name(name: &str) -> Option<Self>
    where
        Self: Sized;

    fn from_raw_name(name: &str) -> Option<Self>
    where
        Self: Sized;
}

#[derive(Debug, Clone)]
pub struct EnumFromNameError {
    enum_class: String,
    enum_name: String,
}

impl EnumFromNameError {
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
            &self.enum_name, &self.enum_class
        )
    }
}

impl std::error::Error for EnumFromNameError {}
