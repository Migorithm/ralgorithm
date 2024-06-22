// Factory method pattern
enum PizzaType {
    Cheese,
    Veggie,
}

trait TPizzaStore {
    // Whole responsibility of creating pizza is delegated to the factory method
    fn create_pizza(&self, pizza_type: PizzaType) -> Box<dyn TPizza>;

    // Template method to order pizza
    fn order_pizza(&self, pizza_type: PizzaType) -> Box<dyn TPizza> {
        let mut pizza = self.create_pizza(pizza_type);
        pizza.prepare();
        pizza.bake();
        pizza.cut();
        pizza.boxed();
        pizza
    }
}

trait TPizza {
    fn prepare(&mut self);
    fn bake(&mut self);
    fn cut(&mut self);
    fn boxed(&mut self);
}

struct NYPizzaStore;

impl TPizzaStore for NYPizzaStore {
    fn create_pizza(&self, pizza_type: PizzaType) -> Box<dyn TPizza> {
        match pizza_type {
            PizzaType::Cheese => Box::new(NYCheesePizza),
            PizzaType::Veggie => Box::new(NYVeggiePizza),
        }
    }
}

struct NYCheesePizza;
impl TPizza for NYCheesePizza {
    fn prepare(&mut self) {
        println!("Preparing NY Cheese Pizza");
    }
    fn bake(&mut self) {
        println!("Baking NY Cheese Pizza");
    }
    fn cut(&mut self) {
        println!("Cutting NY Cheese Pizza");
    }
    fn boxed(&mut self) {
        println!("Boxing NY Cheese Pizza");
    }
}

struct NYVeggiePizza;
impl TPizza for NYVeggiePizza {
    fn prepare(&mut self) {
        println!("Preparing NY Veggie Pizza");
    }
    fn bake(&mut self) {
        println!("Baking NY Veggie Pizza");
    }
    fn cut(&mut self) {
        println!("Cutting NY Veggie Pizza");
    }
    fn boxed(&mut self) {
        println!("Boxing NY Veggie Pizza");
    }
}

#[test]
fn test_ny_pizza() {
    let ny_factory = NYPizzaStore;

    let _ny_veggie_pizza = ny_factory.order_pizza(PizzaType::Veggie);
}
