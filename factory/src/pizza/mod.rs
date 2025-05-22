pub mod chicago_style_cheese_pizza;
pub mod ny_style_cheese_pizza;
pub mod types;

pub trait Pizza {
    fn prepare(&self);
    fn bake(&self);
    fn cut(&self);
    fn box_pizza(&self);
    fn name(&self) -> &str {
        "Unknown Pizza"
    }
}
