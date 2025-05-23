pub mod cheese_pizza;
pub mod clam_pizza;
pub mod types;

pub trait Pizza {
    fn prepare(&self);
    fn bake(&self) {
        println!("180度で25分間焼く");
    }
    fn cut(&self) {
        println!("ピザを扇形にカットする");
    }
    fn box_pizza(&self) {
        println!("ピザを箱に入れる");
    }
    fn name(&self) -> String {
        "Unknown Pizza".to_string()
    }
}
