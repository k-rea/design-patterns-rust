use crate::ingredient::{Cheese, Dough, Sauce};
use crate::ingredient_factory::PizzaIngredientFactory;
use crate::pizza::Pizza;

pub struct CheesePizza {
    factory: Box<dyn PizzaIngredientFactory>,
    dough: Box<dyn Dough>,
    sauce: Box<dyn Sauce>,
    cheese: Box<dyn Cheese>,
}

impl CheesePizza {
    pub fn new(factory: Box<dyn PizzaIngredientFactory>) -> Self {
        Self {
            dough: factory.create_dough(),
            sauce: factory.create_sauce(),
            cheese: factory.create_cheese(),
            factory,
        }
    }
}

impl Pizza for CheesePizza {
    fn prepare(&self) {
        println!("{} を準備中…", self.name());
        println!("生地: {}", self.dough.describe());
        println!("ソース: {}", self.sauce.describe());
        println!("チーズ: {}", self.cheese.describe());
    }
    fn name(&self) -> String {
        format!("{}チーズピザ", self.factory.name_prefix())
    }
}
