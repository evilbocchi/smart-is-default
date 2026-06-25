use serde::{Deserialize, Serialize};
use smart_default::SmartDefault;
use smart_is_default::SmartIsDefault;

#[derive(SmartDefault, SmartIsDefault, Serialize, Deserialize, PartialEq, Debug)]
struct Pair<T: Default + PartialEq> {
    #[default(T::default())]
    #[serde(skip_serializing_if = "Pair::is_default__first", default)]
    first: T,

    #[default(T::default())]
    #[serde(skip_serializing_if = "Pair::is_default__second", default)]
    second: T,
}

#[test]
fn generic_default_struct_serializes_to_empty_object() {
    let p: Pair<i32> = Pair::default();
    assert_eq!(serde_json::to_string(&p).unwrap(), "{}");
}

#[test]
fn generic_only_non_default_field_is_emitted() {
    let p: Pair<i32> = Pair {
        first: 1,
        second: 0,
    };
    assert_eq!(serde_json::to_string(&p).unwrap(), r#"{"first":1}"#);
}

#[test]
fn generic_roundtrip_with_i32() {
    let p: Pair<i32> = Pair {
        first: 10,
        second: 20,
    };
    let json = serde_json::to_string(&p).unwrap();
    let back: Pair<i32> = serde_json::from_str(&json).unwrap();
    assert_eq!(p, back);
}

#[test]
fn generic_works_with_string() {
    let p: Pair<String> = Pair {
        first: "hi".to_string(),
        second: String::new(),
    };
    assert_eq!(serde_json::to_string(&p).unwrap(), r#"{"first":"hi"}"#);
}

#[test]
fn generic_is_default_helpers() {
    let p: Pair<i32> = Pair::default();
    assert!(Pair::is_default__first(&p.first));
    assert!(Pair::is_default__second(&p.second));

    let q: Pair<i32> = Pair {
        first: 5,
        second: 0,
    };
    assert!(!Pair::is_default__first(&q.first));
    assert!(Pair::is_default__second(&q.second));
}

#[test]
fn generic_is_default_whole_struct() {
    assert!(Pair::<i32>::default().is_default());

    let q: Pair<i32> = Pair {
        first: 5,
        second: 0,
    };
    assert!(!q.is_default());

    assert!(Pair::<String>::default().is_default());
}
