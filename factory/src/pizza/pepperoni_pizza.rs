use crate::pizza::Pizza;

pub struct PepperoniPizza {}

impl PepperoniPizza {
    pub fn new() -> Self {
        PepperoniPizza {}
    }
}

impl Pizza for PepperoniPizza {
    fn prepare(&self) {
        println!("Preparing pepperoni pizza");
    }

    fn bake(&self) {
        println!("Baking pepperoni pizza");
    }

    fn cut(&self) {
        println!("Cutting pepperoni pizza");
    }

    fn box_pizza(&self) {
        println!("Boxing pepperoni pizza");
    }
}
