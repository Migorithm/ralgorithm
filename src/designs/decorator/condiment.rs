use super::beverage::Beverage;

pub trait CondimentDecorator: Beverage {}

pub struct Mocha<T: Beverage>(pub T);

pub struct Milk<T: Beverage>(pub T);

pub struct Soy<T: Beverage>(pub T);
pub struct Whip<T: Beverage>(pub T);

impl<T> CondimentDecorator for Mocha<T> where T: Beverage {}

impl<T: Beverage> Beverage for Mocha<T> {
    fn cost(&self) -> f64 {
        self.0.cost() + 0.20
    }
    fn description(&self) -> String {
        self.0.description() + ", Mocha"
    }
}

impl<T> CondimentDecorator for Milk<T> where T: Beverage {}

impl<T: Beverage> Beverage for Milk<T> {
    fn cost(&self) -> f64 {
        self.0.cost() + 0.10
    }
    fn description(&self) -> String {
        self.0.description() + ", Milk"
    }
}

impl<T> CondimentDecorator for Soy<T> where T: Beverage {}

impl<T: Beverage> Beverage for Soy<T> {
    fn cost(&self) -> f64 {
        self.0.cost() + 0.15
    }
    fn description(&self) -> String {
        self.0.description() + ", Soy"
    }
}

impl<T> CondimentDecorator for Whip<T> where T: Beverage {}

impl<T: Beverage> Beverage for Whip<T> {
    fn cost(&self) -> f64 {
        self.0.cost() + 0.10
    }
    fn description(&self) -> String {
        self.0.description() + ", Whip"
    }
}
