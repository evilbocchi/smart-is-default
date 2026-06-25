use serde::{Deserialize, Serialize};
use smart_default::SmartDefault;
use smart_is_default::SmartIsDefault;

#[derive(SmartDefault, SmartIsDefault, Serialize, Deserialize, PartialEq, Debug)]
struct Inner {
    #[default = 5]
    x: i32,
    #[default("inner".to_string())]
    y: String,
}

#[derive(SmartDefault, SmartIsDefault, Serialize, Deserialize, PartialEq, Debug)]
struct Outer {
    #[default(Inner::default())]
    #[serde(skip_serializing_if = "Outer::is_default__inner", default)]
    inner: Inner,

    #[default = 0]
    #[serde(skip_serializing_if = "Outer::is_default__count", default)]
    count: i32,
}

#[test]
fn nested_default_struct_serializes_to_empty_object() {
    let o = Outer::default();
    assert_eq!(serde_json::to_string(&o).unwrap(), "{}");
}

#[test]
fn nested_non_default_inner_is_serialized() {
    let o = Outer {
        inner: Inner {
            x: 99,
            y: "other".to_string(),
        },
        count: 0,
    };
    assert_eq!(
        serde_json::to_string(&o).unwrap(),
        r#"{"inner":{"x":99,"y":"other"}}"#
    );
}

#[test]
fn nested_non_default_count_is_serialized() {
    let o = Outer {
        inner: Inner::default(),
        count: 7,
    };
    assert_eq!(serde_json::to_string(&o).unwrap(), r#"{"count":7}"#);
}

#[test]
fn nested_roundtrip_preserves_values() {
    let o = Outer {
        inner: Inner {
            x: 42,
            y: "z".to_string(),
        },
        count: 3,
    };
    let json = serde_json::to_string(&o).unwrap();
    let back: Outer = serde_json::from_str(&json).unwrap();
    assert_eq!(o, back);
}

#[test]
fn nested_is_default_helper_is_value_based() {
    assert!(Outer::is_default__inner(&Inner::default()));
    assert!(!Outer::is_default__inner(&Inner {
        x: 1,
        y: "a".to_string(),
    }));
}

#[test]
fn nested_partial_deserialize_fills_defaults() {
    let back: Outer = serde_json::from_str(r#"{"count":2}"#).unwrap();
    assert_eq!(back.inner, Inner::default());
    assert_eq!(back.count, 2);
}
