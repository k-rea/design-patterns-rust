use crate::pizza::Pizza;

pub struct NYStyleCheesePizza {}

impl Pizza for NYStyleCheesePizza {
    fn prepare(&self) {
        println!("NYStyleCheesePizza: 生地を準備しています");
    }
    fn bake(&self) {
        println!("NYStyleCheesePizza: 焼いています");
    }
    fn cut(&self) {
        println!("NYStyleCheesePizza: 三角にカットしています");
    }
    fn box_pizza(&self) {
        println!("NYStyleCheesePizza: 箱詰めしています");
    }
    fn name(&self) -> &str {
        "NYスタイルチーズピザ"
    }
}
