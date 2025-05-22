use crate::pizza::Pizza;

pub struct CheesePizza {}

impl CheesePizza {
    pub fn new() -> CheesePizza {
        CheesePizza {}
    }
}
impl Pizza for CheesePizza {
    fn prepare(&self) {
        println!("Preparing cheese pizza");
    }

    fn bake(&self) {
        println!("Baking cheese pizza");
    }

    fn cut(&self) {
        println!("Cutting cheese pizza");
    }

    fn box_pizza(&self) {
        println!("Boxing cheese pizza");
    }
}
