pub trait EnumClass {
    fn class() -> &'static str
    where
        Self: Sized;
}
