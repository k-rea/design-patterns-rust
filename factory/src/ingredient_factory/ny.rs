use crate::ingredient::ny::{FreshClams, MarinaraSauce, ReggianoCheese, ThinCrustDough};
use crate::ingredient::{Cheese, Clam, Dough, Sauce};
use crate::ingredient_factory::PizzaIngredientFactory;

pub struct NYPizzaIngredientFactory;

impl PizzaIngredientFactory for NYPizzaIngredientFactory {
    fn create_dough(&self) -> Box<dyn Dough> {
        Box::new(ThinCrustDough)
    }
    fn create_sauce(&self) -> Box<dyn Sauce> {
        Box::new(MarinaraSauce)
    }
    fn create_cheese(&self) -> Box<dyn Cheese> {
        Box::new(ReggianoCheese)
    }

    fn create_clam(&self) -> Box<dyn Clam> {
        Box::new(FreshClams)
    }
    fn name_prefix(&self) -> &str {
        "NYスタイル"
    }
}
