use crate::pizza::Pizza;
use crate::pizza::chicago_style_cheese_pizza::ChicagoStyleCheesePizza;
use crate::pizza::types::PizzaType;
use crate::store::PizzaStore;

pub struct ChicagoPizzaStore;

impl PizzaStore for ChicagoPizzaStore {
    fn create_pizza(&self, pizza_type: PizzaType) -> Box<dyn Pizza> {
        match pizza_type {
            PizzaType::Cheese => Box::new(ChicagoStyleCheesePizza {}),
        }
    }
}
