use crate::pizza::cheese_pizza::CheesePizza;
use crate::pizza::greek_pizza::GreekPizza;
use crate::pizza::pepperoni_pizza::PepperoniPizza;
use crate::pizza::types::PizzaType;

pub struct SimplePizzaFactory;

impl SimplePizzaFactory {
    pub fn create_pizza(&self, pizza_type: PizzaType) -> Box<dyn crate::pizza::Pizza> {
        match pizza_type {
            PizzaType::Cheese => Box::new(CheesePizza::new()),
            PizzaType::Greek => Box::new(GreekPizza::new()),
            PizzaType::Pepperoni => Box::new(PepperoniPizza::new()),
        }
    }
}
