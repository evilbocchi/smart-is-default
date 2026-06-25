use smart_default::SmartDefault;
use smart_is_default::SmartIsDefault;

#[derive(SmartDefault, SmartIsDefault, PartialEq, Debug)]
#[smart_is_default(no_is_default)]
struct Item {
    #[default = 0]
    count: i32,
}

impl Item {
    fn is_default(&self) -> bool {
        self.count == 1
    }
}

#[test]
fn user_declared_is_default_is_used() {
    let item = Item::default();
    assert!(!item.is_default());

    let other = Item { count: 1 };
    assert!(other.is_default());

    let item = Item::default();
    assert!(Item::is_default__count(&item.count));

    let other = Item { count: 5 };
    assert!(!Item::is_default__count(&other.count));
}
