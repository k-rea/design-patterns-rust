use crate::pizza::Pizza;
use crate::pizza::ny_style_cheese_pizza::NYStyleCheesePizza;
use crate::pizza::types::PizzaType;
use crate::store::PizzaStore;

pub struct NYPizzaStore;

impl PizzaStore for NYPizzaStore {
    fn create_pizza(&self, pizza_type: PizzaType) -> Box<dyn Pizza> {
        match pizza_type {
            PizzaType::Cheese => Box::new(NYStyleCheesePizza {}),
        }
    }
}
