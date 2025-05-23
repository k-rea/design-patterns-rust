use crate::pizza::types::PizzaType;
use crate::store::PizzaStore;
use crate::store::chicago_pizza_store::ChicagoPizzaStore;
use crate::store::ny_pizza_store::NYPizzaStore;

mod ingredient;
mod ingredient_factory;
mod pizza;
mod store;

pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    let ny_store = NYPizzaStore;
    let chicago_store = ChicagoPizzaStore;
    println!("\n======== NY Pizza Store ==========");
    ny_store.order_pizza(PizzaType::Cheese);
    ny_store.order_pizza(PizzaType::Clam);
    println!("\n======== Chicago Pizza Store ==========");
    chicago_store.order_pizza(PizzaType::Cheese);
    chicago_store.order_pizza(PizzaType::Clam);
    Ok(())
}
