pub mod chicago;
pub mod ny;

use crate::ingredient::{Cheese, Clam, Dough, Sauce};

pub trait PizzaIngredientFactory {
    fn create_dough(&self) -> Box<dyn Dough>;
    fn create_sauce(&self) -> Box<dyn Sauce>;
    fn create_cheese(&self) -> Box<dyn Cheese>;
    fn create_clam(&self) -> Box<dyn Clam>;
    fn name_prefix(&self) -> &str;
}
