pub trait Beverage {
    fn description(&self) -> String;
    fn cost(&self) -> f64;
}
pub struct HouseBlend;
pub struct DardRoast;
pub struct Espresso;
pub struct Decaf;

impl Beverage for HouseBlend {
    fn cost(&self) -> f64 {
        0.89
    }
    fn description(&self) -> String {
        "HouseBlend".to_string()
    }
}
impl Beverage for DardRoast {
    fn cost(&self) -> f64 {
        0.99
    }
    fn description(&self) -> String {
        "DardRoast".to_string()
    }
}
impl Beverage for Espresso {
    fn cost(&self) -> f64 {
        1.99
    }
    fn description(&self) -> String {
        "Espresso".to_string()
    }
}
impl Beverage for Decaf {
    fn cost(&self) -> f64 {
        1.05
    }
    fn description(&self) -> String {
        "Decaf".to_string()
    }
}
