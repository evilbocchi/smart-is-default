use smart_default::SmartDefault;
use smart_is_default::SmartIsDefault;

#[derive(SmartDefault, SmartIsDefault, PartialEq, Debug)]
struct Item {
    #[default = 0]
    count: i32,

    #[smart_is_default(skip)]
    #[default = 0]
    skipped: i32,

    #[default = 1]
    other: i32,
}

#[test]
fn non_skipped_helpers_are_generated() {
    let def = Item::default();
    assert!(Item::is_default__count(&def.count));
    assert!(Item::is_default__other(&def.other));

    let other = Item {
        count: 5,
        skipped: 0,
        other: 1,
    };
    assert!(!Item::is_default__count(&other.count));
    assert!(Item::is_default__other(&other.other));
}

#[test]
fn whole_struct_is_default_is_still_generated() {
    assert!(Item::default().is_default());

    let other = Item {
        count: 5,
        skipped: 0,
        other: 1,
    };
    assert!(!other.is_default());
}

// Compile-time proof that the skipped helper is *not* generated.
// If `Item::is_default__skipped` existed, this trait impl would be a duplicate
// `is_default__skipped` definition (E0201) / conflicting impl. Instead we
// provide it ourselves here, which only compiles if the derive did NOT emit it.
impl Item {
    #[allow(non_snake_case)]
    fn is_default__skipped(v: &i32) -> bool {
        *v == 42
    }
}

#[test]
fn user_can_provide_own_helper_for_skipped_field() {
    // Our manual definition wins because the derive skipped this field.
    assert!(Item::is_default__skipped(&42));
    assert!(!Item::is_default__skipped(&0));
}
