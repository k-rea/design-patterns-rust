use crate::pizza::types::PizzaType;
use crate::simple_pizza_factory::SimplePizzaFactory;

mod pizza;
mod pizza_store;
mod simple_pizza_factory;

pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    let factory = SimplePizzaFactory {};
    let order_pizza = pizza_store::PizzaStore::new(factory);
    order_pizza.order_pizza(PizzaType::Cheese);
    println!("---------------------");
    order_pizza.order_pizza(PizzaType::Pepperoni);
    Ok(())
}
