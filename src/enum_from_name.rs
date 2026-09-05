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

pub trait FromEnumName {
    fn from_name(name: &str) -> Option<Self>
    where
        Self: Sized;

    fn from_raw_name(name: &str) -> Option<Self>
    where
        Self: Sized;
}
