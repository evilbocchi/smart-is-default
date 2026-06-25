use smart_default::SmartDefault;
use smart_is_default::SmartIsDefault;

#[derive(SmartDefault, SmartIsDefault, PartialEq, Debug)]
struct Config(
    #[default = 12] i32,
    #[default("four".to_string())] String,
    #[default = true] bool,
    #[default(None)] Option<String>,
    #[default(Vec::new())] Vec<i32>,
);

impl Config {
    fn all_non_default() -> Self {
        Config(
            1,
            "hi".to_string(),
            false,
            Some("x".to_string()),
            vec![1, 2],
        )
    }
}

#[test]
fn helpers_report_true_for_default_tuple_values() {
    let def = Config::default();
    assert!(Config::is_default__0(&def.0));
    assert!(Config::is_default__1(&def.1));
    assert!(Config::is_default__2(&def.2));
    assert!(Config::is_default__3(&def.3));
    assert!(Config::is_default__4(&def.4));
}

#[test]
fn helpers_report_false_for_non_default_tuple_values() {
    let other = Config::all_non_default();
    assert!(!Config::is_default__0(&other.0));
    assert!(!Config::is_default__1(&other.1));
    assert!(!Config::is_default__2(&other.2));
    assert!(!Config::is_default__3(&other.3));
    assert!(!Config::is_default__4(&other.4));
}

#[test]
fn helpers_compare_tuple_fields_by_value_not_identity() {
    assert!(Config::is_default__0(&12));
    assert!(Config::is_default__1(&"four".to_string()));
    assert!(Config::is_default__2(&true));
    assert!(Config::is_default__3(&None));
    assert!(Config::is_default__4(&Vec::<i32>::new()));
}

#[test]
fn is_default_works_for_tuple_structs() {
    assert!(Config::default().is_default());
    assert!(!Config::all_non_default().is_default());
    assert!(Config(12, "four".to_string(), true, None, Vec::new()).is_default());
}

#[derive(SmartDefault, SmartIsDefault, PartialEq, Debug)]
struct SkippedTuple(
    #[default = 0] i32,
    #[smart_is_default(skip)]
    #[default = 0]
    i32,
    #[default = 1] i32,
);

impl SkippedTuple {
    #[allow(non_snake_case)]
    fn is_default__1(v: &i32) -> bool {
        *v == 42
    }
}

#[test]
fn tuple_field_skip_suppresses_generated_helper() {
    let def = SkippedTuple::default();
    assert!(SkippedTuple::is_default__0(&def.0));
    assert!(SkippedTuple::is_default__2(&def.2));

    assert!(SkippedTuple::is_default__1(&42));
    assert!(!SkippedTuple::is_default__1(&0));
}
