use crate::pizza::cheese_pizza::CheesePizza;
use crate::pizza::greek_pizza::GreekPizza;
use crate::pizza::pepperoni_pizza::PepperoniPizza;
use crate::pizza::Pizza;
use crate::pizza::types::PizzaType;

pub struct OrderPizza {}

impl OrderPizza {
    pub fn new() -> Self {
        OrderPizza {}
    }

    pub fn order_pizza(&self, pizza_type: PizzaType)-> Box<dyn Pizza> {
        let pizza: Box<dyn Pizza> = match pizza_type {
            PizzaType::Cheese => Box::new(CheesePizza::new()),
            PizzaType::Pepperoni => Box::new(PepperoniPizza::new()),
            PizzaType::Greek => Box::new(GreekPizza::new()),
        };

        pizza.prepare();
        pizza.bake();
        pizza.cut();
        pizza.box_pizza();
        pizza
    }
}