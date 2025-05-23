use crate::ingredient_factory::chicago::ChicagoPizzaIngredientFactory;
use crate::pizza::Pizza;
use crate::pizza::cheese_pizza::CheesePizza;
use crate::pizza::clam_pizza::ClamPizza;
use crate::pizza::types::PizzaType;
use crate::store::PizzaStore;

pub struct ChicagoPizzaStore;

impl PizzaStore for ChicagoPizzaStore {
    fn create_pizza(&self, pizza_type: PizzaType) -> Box<dyn Pizza> {
        let ingredient_factory = Box::new(ChicagoPizzaIngredientFactory {});
        match pizza_type {
            PizzaType::Cheese => Box::new(CheesePizza::new(ingredient_factory)),
            PizzaType::Clam => Box::new(ClamPizza::new(ingredient_factory)),
        }
    }
}
