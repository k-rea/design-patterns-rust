use crate::ingredient::{Cheese, Clam, Dough, Sauce};
use crate::ingredient_factory::PizzaIngredientFactory;
use crate::pizza::Pizza;

pub struct ClamPizza {
    dough: Box<dyn Dough>,
    sauce: Box<dyn Sauce>,
    cheese: Box<dyn Cheese>,
    clam: Box<dyn Clam>,
    factory: Box<dyn PizzaIngredientFactory>,
}

impl ClamPizza {
    pub fn new(factory: Box<dyn PizzaIngredientFactory>) -> Self {
        Self {
            dough: factory.create_dough(),
            sauce: factory.create_sauce(),
            cheese: factory.create_cheese(),
            clam: factory.create_clam(),
            factory,
        }
    }
}

impl Pizza for ClamPizza {
    fn prepare(&self) {
        println!("{} を準備中…", self.name());
        println!("生地: {}", self.dough.describe());
        println!("ソース: {}", self.sauce.describe());
        println!("チーズ: {}", self.cheese.describe());
        println!("アサリ: {}", self.clam.describe());
    }
    fn name(&self) -> String {
        format!("{}アサリピザ", self.factory.name_prefix())

    }
}
