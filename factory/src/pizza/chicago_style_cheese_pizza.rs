use crate::pizza::Pizza;

pub struct ChicagoStyleCheesePizza;
impl Pizza for ChicagoStyleCheesePizza {
    fn prepare(&self) {
        println!("ChicagoStyleCheesePizza: 厚い生地を準備しています");
    }
    fn bake(&self) {
        println!("ChicagoStyleCheesePizza: 焼いています");
    }
    fn cut(&self) {
        println!("ChicagoStyleCheesePizza: 四角にカットしています");
    }
    fn box_pizza(&self) {
        println!("ChicagoStyleCheesePizza: 箱詰めしています");
    }
    fn name(&self) -> &str {
        "シカゴスタイルチーズピザ"
    }
}
