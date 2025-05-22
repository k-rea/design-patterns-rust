pub mod chicago_pizza_store;
pub mod ny_pizza_store;

use crate::pizza::Pizza;
use crate::pizza::types::PizzaType;

pub trait PizzaStore {
    fn order_pizza(&self, pizza_type: PizzaType) -> Box<dyn Pizza> {
        let pizza = self.create_pizza(pizza_type);

        println!("--- {} が注文されました ---", pizza.name());
        pizza.prepare();
        pizza.bake();
        pizza.cut();
        pizza.box_pizza();
        pizza
    }

    fn create_pizza(&self, pizza_type: PizzaType) -> Box<dyn Pizza>; // Factory Method
}
