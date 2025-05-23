use crate::ingredient_factory::ny::NYPizzaIngredientFactory;
use crate::pizza::Pizza;
use crate::pizza::cheese_pizza::CheesePizza;
use crate::pizza::clam_pizza::ClamPizza;
use crate::pizza::types::PizzaType;
use crate::store::PizzaStore;

pub struct NYPizzaStore;

impl PizzaStore for NYPizzaStore {
    fn create_pizza(&self, pizza_type: PizzaType) -> Box<dyn Pizza> {
        let ingredient_factory = Box::new(NYPizzaIngredientFactory {});
        match pizza_type {
            PizzaType::Cheese => Box::new(CheesePizza::new(ingredient_factory)),
            PizzaType::Clam => Box::new(ClamPizza::new(ingredient_factory)),
        }
    }
}
