use crate::EnumClass;

#[derive(Debug, Clone)]
struct FromEnumNameError {
    name: String,
}

impl std::fmt::Display for FromEnumNameError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Invalid enum name: {}", self.name)
    }
}

impl std::error::Error for FromEnumNameError {}

pub trait FromEnumName : EnumClass {
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
        write!(f, "Invalid enum name \"{}\" for enum `{}`", self.name, self.enum)
    }
}

impl std::error::Error for EnumFromNameError {}
