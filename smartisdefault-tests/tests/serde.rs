use serde::{Deserialize, Serialize};
use smart_default::SmartDefault;
use smart_is_default::SmartIsDefault;

#[derive(SmartDefault, SmartIsDefault, Serialize, Deserialize, PartialEq, Debug)]
#[serde(default)]
struct Config {
    #[default = 12]
    #[serde(skip_serializing_if = "Config::is_default__a")]
    a: i32,

    #[default("four".to_string())]
    #[serde(skip_serializing_if = "Config::is_default__e")]
    e: String,

    #[default = true]
    #[serde(skip_serializing_if = "Config::is_default__flag")]
    flag: bool,

    #[default(None)]
    #[serde(skip_serializing_if = "Config::is_default__opt")]
    opt: Option<String>,

    #[default(Vec::new())]
    #[serde(skip_serializing_if = "Config::is_default__list")]
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

#[derive(SmartDefault, SmartIsDefault, Serialize, Deserialize, PartialEq, Debug)]
struct PerFieldDefaults {
    #[default = 12]
    #[serde(skip_serializing_if = "PerFieldDefaults::is_default__a", default)]
    a: i32,

    #[default = true]
    #[serde(skip_serializing_if = "PerFieldDefaults::is_default__flag", default)]
    flag: bool,
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
fn all_fields_are_emitted_when_none_match_default() {
    let cfg = Config::all_non_default();
    let json = serde_json::to_string(&cfg).unwrap();
    assert_eq!(
        json,
        r#"{"a":1,"e":"hi","flag":false,"opt":"x","list":[1,2]}"#
    );
}

#[test]
fn a_value_that_compares_equal_to_default_is_still_skipped() {
    let cfg = Config {
        a: 12,
        ..Config::default()
    };
    assert_eq!(serde_json::to_string(&cfg).unwrap(), "{}");
}

#[test]
fn roundtrip_preserves_all_values() {
    let cfg = Config::all_non_default();
    let json = serde_json::to_string(&cfg).unwrap();
    let back: Config = serde_json::from_str(&json).unwrap();
    assert_eq!(cfg, back);
}

#[test]
fn deserializing_empty_object_yields_smart_default() {
    let back: Config = serde_json::from_str("{}").unwrap();
    assert_eq!(back, Config::default());
}

#[test]
fn deserializing_partial_object_fills_missing_with_smart_defaults() {
    let back: Config = serde_json::from_str(r#"{"a":100}"#).unwrap();
    assert_eq!(back.a, 100);
    assert_eq!(back.e, "four");
    assert!(back.flag);
    assert!(back.opt.is_none());
    assert!(back.list.is_empty());
}

#[test]
fn skip_then_deserialize_roundtrips_to_default() {
    let json = serde_json::to_string(&Config::default()).unwrap();
    let back: Config = serde_json::from_str(&json).unwrap();
    assert_eq!(back, Config::default());
}

#[test]
fn to_json_value_then_back_preserves_semantics() {
    let cfg = Config {
        a: 7,
        e: "hello".to_string(),
        ..Config::default()
    };
    let value = serde_json::to_value(&cfg).unwrap();
    // Only `a` and `e` differ from default, so only those survive.
    assert_eq!(value, serde_json::json!({ "a": 7, "e": "hello" }));
    let back: Config = serde_json::from_value(value).unwrap();
    assert_eq!(back, cfg);
}

#[test]
fn per_field_default_uses_field_types_std_default_not_smart_default() {
    let back: PerFieldDefaults = serde_json::from_str("{}").unwrap();
    assert_eq!(back, PerFieldDefaults { a: 0, flag: false });
    assert_ne!(back, PerFieldDefaults::default());
}

#[test]
fn per_field_default_skip_then_deserialize_is_not_symmetric() {
    let json = serde_json::to_string(&PerFieldDefaults::default()).unwrap();
    assert_eq!(json, "{}");
    let back: PerFieldDefaults = serde_json::from_str(&json).unwrap();
    assert_ne!(back, PerFieldDefaults::default());
}
