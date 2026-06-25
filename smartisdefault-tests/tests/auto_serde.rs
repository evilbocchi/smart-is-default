//! Tests for the `#[smart_is_default(serde)]` flag, which auto-generates a
//! `serde::Serialize` impl that skips fields whose value equals
//! `Self::default()` — the proc-macro equivalent of writing
//! `#[serde(skip_serializing_if = "StructName::is_default__<field>")]` on
//! every field.

use serde_json;
use smart_default::SmartDefault;
use smart_is_default::SmartIsDefault;

// Named struct: the main use case.

#[derive(SmartDefault, SmartIsDefault, PartialEq, Debug)]
#[smart_is_default(serde)]
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

#[test]
fn fully_default_struct_serializes_to_empty_object() {
    let cfg = Config::default();
    let json = serde_json::to_string(&cfg).unwrap();
    assert_eq!(json, "{}");
}

#[test]
fn only_non_default_fields_are_emitted() {
    let cfg = Config {
        a: 99,
        ..Config::default()
    };
    let json = serde_json::to_string(&cfg).unwrap();
    assert_eq!(json, r#"{"a":99}"#);
}

#[test]
fn value_that_compares_equal_to_default_is_still_skipped() {
    // `a` is set to 12, which is the default value, so it must be skipped.
    let cfg = Config {
        a: 12,
        ..Config::default()
    };
    let json = serde_json::to_string(&cfg).unwrap();
    assert_eq!(json, "{}");
}

#[test]
fn all_fields_are_emitted_when_none_match_default() {
    let cfg = Config {
        a: 1,
        e: "hi".to_string(),
        flag: false,
        opt: Some("x".to_string()),
        list: vec![1, 2],
    };
    let json = serde_json::to_string(&cfg).unwrap();
    assert_eq!(
        json,
        r#"{"a":1,"e":"hi","flag":false,"opt":"x","list":[1,2]}"#
    );
}

#[test]
fn fields_appear_in_declared_order() {
    let cfg = Config {
        a: 1,
        e: "x".to_string(),
        flag: false,
        opt: Some("y".to_string()),
        list: vec![9],
    };
    let json = serde_json::to_string(&cfg).unwrap();
    assert_eq!(json, r#"{"a":1,"e":"x","flag":false,"opt":"y","list":[9]}"#);
}

// Interaction with `#[smart_is_default(skip)]`.
// Skipped fields have no helper, so the auto-serialize impl always emits
// them (no skip check is performed).

#[derive(SmartDefault, SmartIsDefault, PartialEq, Debug)]
#[smart_is_default(serde)]
struct WithSkipped {
    #[default = 0]
    count: i32,

    #[smart_is_default(skip)]
    #[default = 0]
    always_emitted: i32,
}

#[test]
fn skipped_field_is_always_serialized() {
    let v = WithSkipped::default();
    // `count` matches the default and is skipped, but `always_emitted` is
    // always emitted because its helper was suppressed.
    let json = serde_json::to_string(&v).unwrap();
    assert_eq!(json, r#"{"always_emitted":0}"#);
}

#[test]
fn skipped_field_appears_alongside_non_default_field() {
    let v = WithSkipped {
        count: 5,
        always_emitted: 7,
    };
    let json = serde_json::to_string(&v).unwrap();
    assert_eq!(json, r#"{"count":5,"always_emitted":7}"#);
}

// Tuple struct: the flag is a no-op for skipping (skipping tuple elements
// would break deserialization), but the user still gets a `Serialize` impl.

#[derive(SmartDefault, SmartIsDefault, PartialEq, Debug)]
#[smart_is_default(serde)]
struct Tuple(i32, String, bool);

#[test]
fn tuple_struct_serializes_all_fields() {
    let v = Tuple(1, "hi".to_string(), false);
    let json = serde_json::to_string(&v).unwrap();
    assert_eq!(json, r#"[1,"hi",false]"#);
}

#[test]
fn tuple_struct_serializes_default_value() {
    let v = Tuple::default();
    let json = serde_json::to_string(&v).unwrap();
    assert_eq!(json, r#"[0,"",false]"#);
}

// Without the flag, no `Serialize` impl is generated. `Config` is checked
// at compile time above; this struct deliberately *doesn't* have the flag
// and is not used as Serialize, so the test is that this whole file
// compiles. (A separate `cargo test` will catch any unintended impl
// collisions.)

#[derive(SmartDefault, SmartIsDefault, PartialEq, Debug)]
struct NoFlag {
    #[default = 0]
    a: i32,
}

#[test]
fn no_serde_flag_means_helpers_only() {
    // Helper still works; this just proves the flag is opt-in.
    let v = NoFlag::default();
    assert!(NoFlag::is_default__a(&v.a));
}
