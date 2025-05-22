pub mod cheese_pizza;
pub mod greek_pizza;
pub mod pepperoni_pizza;
pub mod types;

pub trait Pizza {
    fn prepare(&self);
    fn bake(&self);
    fn cut(&self);
    fn box_pizza(&self);
}
