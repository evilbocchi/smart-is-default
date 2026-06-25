use smart_default::SmartDefault;
use smart_is_default::SmartIsDefault;

#[derive(SmartDefault, SmartIsDefault, PartialEq, Debug)]
struct Config {
    #[default = 12]
    a: i32,

    #[default("four".to_string())]
    e: String,

    #[default = true]
    flag: bool,

    #[default(None)]
    opt: Option<String>,

    #[default(Vec::new())]
    list: Vec<i32>,
}

impl Config {
    fn all_non_default() -> Self {
        Config {
            a: 1,
            e: "hi".to_string(),
            flag: false,
            opt: Some("x".to_string()),
            list: vec![1, 2],
        }
    }
}

#[test]
fn helpers_report_true_for_default_values() {
    let def = Config::default();
    assert!(Config::is_default__a(&def.a));
    assert!(Config::is_default__e(&def.e));
    assert!(Config::is_default__flag(&def.flag));
    assert!(Config::is_default__opt(&def.opt));
    assert!(Config::is_default__list(&def.list));
}

#[test]
fn helpers_report_false_for_non_default_values() {
    let other = Config::all_non_default();
    assert!(!Config::is_default__a(&other.a));
    assert!(!Config::is_default__e(&other.e));
    // default `flag` is `true`, so `false` is non-default.
    assert!(!Config::is_default__flag(&other.flag));
    assert!(!Config::is_default__opt(&other.opt));
    assert!(!Config::is_default__list(&other.list));
}

#[test]
fn helpers_compare_by_value_not_identity() {
    assert!(Config::is_default__a(&12));
    assert!(Config::is_default__e(&"four".to_string()));
    assert!(Config::is_default__opt(&None));
    assert!(Config::is_default__list(&Vec::<i32>::new()));
}

#[test]
fn smart_default_values_are_as_expected() {
    let def = Config::default();
    assert_eq!(def.a, 12);
    assert_eq!(def.e, "four");
    assert!(def.flag);
    assert!(def.opt.is_none());
    assert!(def.list.is_empty());
}

#[test]
fn is_default_true_for_default_instance() {
    assert!(Config::default().is_default());
}

#[test]
fn is_default_false_for_all_non_default_instance() {
    assert!(!Config::all_non_default().is_default());
}

#[test]
fn is_default_false_when_single_field_differs() {
    let cfg = Config {
        a: 99,
        ..Config::default()
    };
    assert!(!cfg.is_default());
}

#[test]
fn is_default_true_for_value_equal_to_default() {
    // A fresh instance with values that compare equal to the smart defaults
    // (not the same object as `Config::default()`) must still report `true`.
    let cfg = Config {
        a: 12,
        e: "four".to_string(),
        flag: true,
        opt: None,
        list: Vec::new(),
    };
    assert!(cfg.is_default());
}
