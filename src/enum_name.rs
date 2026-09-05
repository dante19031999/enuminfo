pub trait EnumName {
    fn name(&self) -> &'static str;

    fn raw_name(&self) -> &'static str;
}
