use crate::pizza::types::PizzaType;

mod order_pizza;
mod pizza;

pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    let order_pizza = order_pizza::OrderPizza::new();
    order_pizza.order_pizza(PizzaType::Cheese);
    println!("---------------------");
    order_pizza.order_pizza(PizzaType::Pepperoni);
    Ok(())
}