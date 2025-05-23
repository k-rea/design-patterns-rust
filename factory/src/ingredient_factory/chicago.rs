use crate::ingredient::chicago::{FrozenClams, MozzarellaCheese, PlumTomatoSauce, ThickCrustDough};
use crate::ingredient::{Cheese, Clam, Dough, Sauce};
use crate::ingredient_factory::PizzaIngredientFactory;

pub struct ChicagoPizzaIngredientFactory;

impl PizzaIngredientFactory for ChicagoPizzaIngredientFactory {
    fn create_dough(&self) -> Box<dyn Dough> {
        Box::new(ThickCrustDough)
    }
    fn create_sauce(&self) -> Box<dyn Sauce> {
        Box::new(PlumTomatoSauce)
    }
    fn create_cheese(&self) -> Box<dyn Cheese> {
        Box::new(MozzarellaCheese)
    }

    fn create_clam(&self) -> Box<dyn Clam> {
        Box::new(FrozenClams)
    }
    fn name_prefix(&self) -> &str {
        "シカゴスタイル"
    }
}
