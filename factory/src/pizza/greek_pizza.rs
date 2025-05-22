use crate::pizza::Pizza;

pub struct GreekPizza {}
impl GreekPizza {
    pub fn new() -> Self {
        GreekPizza {}
    }
}
impl Pizza for GreekPizza {
    fn prepare(&self) {
        println!("Preparing Greek pizza");
    }

    fn bake(&self) {
        println!("Baking Greek pizza");
    }

    fn cut(&self) {
        println!("Cutting Greek pizza");
    }

    fn box_pizza(&self) {
        println!("Boxing Greek pizza");
    }
}
