pub mod beverage;
pub mod condiment;

#[test]
fn test_beverage() {
    use beverage::*;
    use condiment::*;

    let espresso_with_double_mocka_soy_milk_whip = Whip(Milk(Soy(Mocha(Mocha(Espresso)))));

    assert_eq!(espresso_with_double_mocka_soy_milk_whip.cost(), 2.74);
    assert_eq!(
        espresso_with_double_mocka_soy_milk_whip.description(),
        "Espresso, Mocha, Mocha, Soy, Milk, Whip"
    );
}
