use smart_is_default::SmartIsDefault;

#[derive(SmartIsDefault, PartialEq, Debug)]
enum Setting {
    Named { count: i32 },
    Tuple(String),
    Off,
}

impl Default for Setting {
    fn default() -> Self {
        Setting::Named { count: 12 }
    }
}

#[test]
fn enum_is_default_true_for_default_variant_value() {
    assert!(Setting::default().is_default());
    assert!(Setting::Named { count: 12 }.is_default());
}

#[test]
fn enum_is_default_false_for_non_default_values() {
    assert!(!Setting::Named { count: 1 }.is_default());
    assert!(!Setting::Tuple("four".to_string()).is_default());
    assert!(!Setting::Off.is_default());
}

#[derive(SmartIsDefault, PartialEq, Debug)]
#[smart_is_default(no_is_default)]
enum ManualEnum {
    Default,
    Other,
}

impl Default for ManualEnum {
    fn default() -> Self {
        ManualEnum::Default
    }
}

impl ManualEnum {
    fn is_default(&self) -> bool {
        matches!(self, ManualEnum::Other)
    }
}

#[test]
fn enum_respects_no_is_default() {
    assert!(!ManualEnum::Default.is_default());
    assert!(ManualEnum::Other.is_default());
}
