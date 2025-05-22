use crate::pizza::Pizza;
use crate::pizza::types::PizzaType;
use crate::simple_pizza_factory::SimplePizzaFactory;

pub struct PizzaStore {
    factory: SimplePizzaFactory,
}

impl PizzaStore {
    pub fn new(factory: SimplePizzaFactory) -> Self {
        PizzaStore { factory }
    }

    pub fn order_pizza(&self, pizza_type: PizzaType) -> Box<dyn Pizza> {
        let pizza = self.factory.create_pizza(pizza_type);

        pizza.prepare();
        pizza.bake();
        pizza.cut();
        pizza.box_pizza();
        pizza
    }
}
