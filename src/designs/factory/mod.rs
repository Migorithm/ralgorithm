// Factory method pattern
enum KimbabType {
    Tuna,
    Veggie,
}

// Factory to create kimbab
trait TKimbabStore {
    // Whole responsibility of creating kimbab is delegated to the factory method
    fn make_kimbab(&self, kimbab_type: KimbabType) -> Box<dyn TKimbab>;

    // Template method to order kimbab
    fn order_kimbab(&self, kimbab_type: KimbabType) -> Box<dyn TKimbab> {
        let mut kimbab = self.make_kimbab(kimbab_type);
        kimbab.prepare();
        kimbab.cut();
        kimbab.boxed();
        kimbab
    }
}

// Product sold by the store
trait TKimbab {
    fn prepare(&mut self);
    fn cut(&mut self);
    fn boxed(&mut self);
}

struct NYKimbabStore;

impl TKimbabStore for NYKimbabStore {
    fn make_kimbab(&self, kimbab_type: KimbabType) -> Box<dyn TKimbab> {
        match kimbab_type {
            KimbabType::Tuna => Box::new(NYTunaKimbab),
            KimbabType::Veggie => Box::new(NYVeggieKimbab),
        }
    }
}

struct NYTunaKimbab;
impl TKimbab for NYTunaKimbab {
    fn prepare(&mut self) {
        println!("Preparing NY Tuna Kimbab");
    }

    fn cut(&mut self) {
        println!("Cutting NY Tuna Kimbab");
    }
    fn boxed(&mut self) {
        println!("Boxing NY Tuna Kimbab");
    }
}

struct NYVeggieKimbab;
impl TKimbab for NYVeggieKimbab {
    fn prepare(&mut self) {
        println!("Preparing NY Veggie Kimbab");
    }
    fn cut(&mut self) {
        println!("Cutting NY Veggie Kimbab");
    }
    fn boxed(&mut self) {
        println!("Boxing NY Veggie Kimbab");
    }
}

#[test]
fn test_ny_kimbab() {
    let ny_factory = NYKimbabStore;

    let _ny_veggie_imbab = ny_factory.order_kimbab(KimbabType::Veggie);
}
