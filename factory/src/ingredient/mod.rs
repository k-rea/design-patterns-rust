pub mod chicago;
pub mod ny;

pub trait Dough {
    fn describe(&self) -> &str;
}
pub trait Sauce {
    fn describe(&self) -> &str;
}
pub trait Cheese {
    fn describe(&self) -> &str;
}
pub trait Clam {
    fn describe(&self) -> &str;
}
