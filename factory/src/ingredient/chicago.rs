use crate::ingredient::{Cheese, Clam, Dough, Sauce};

pub struct ThickCrustDough;
impl Dough for ThickCrustDough {
    fn describe(&self) -> &str {
        "厚いクリスピー生地"
    }
}

pub struct PlumTomatoSauce;
impl Sauce for PlumTomatoSauce {
    fn describe(&self) -> &str {
        "プラムトマトソース"
    }
}

pub struct MozzarellaCheese;
impl Cheese for MozzarellaCheese {
    fn describe(&self) -> &str {
        "モッツァレラチーズ"
    }
}

pub struct FrozenClams;
impl Clam for FrozenClams {
    fn describe(&self) -> &str {
        "冷凍アサリ"
    }
}
