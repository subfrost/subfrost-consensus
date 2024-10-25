pub trait Token {
    fn name(&self) -> String;
    fn symbol(&self) -> String;
}
