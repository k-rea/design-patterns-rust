use crate::ingredient::{Cheese, Clam, Dough, Sauce};

pub struct ThinCrustDough;
impl Dough for ThinCrustDough {
    fn describe(&self) -> &str {
        "薄いクリスピー生地"
    }
}

pub struct MarinaraSauce;
impl Sauce for MarinaraSauce {
    fn describe(&self) -> &str {
        "マリナラソース"
    }
}

pub struct ReggianoCheese;
impl Cheese for ReggianoCheese {
    fn describe(&self) -> &str {
        "レッジャーノチーズ"
    }
}

pub struct FreshClams;
impl Clam for FreshClams {
    fn describe(&self) -> &str {
        "新鮮なアサリ"
    }
}
