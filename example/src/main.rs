use serde::{Deserialize, Serialize};
use smart_default::SmartDefault;
use smart_is_default::SmartIsDefault;

#[derive(Debug, SmartDefault, SmartIsDefault, Deserialize, Serialize, PartialEq)]
#[serde(default)]
struct Config {
    #[default = 12]
    #[serde(skip_serializing_if = "Config::is_default__a")]
    a: i32,

    #[default("four".to_string())]
    #[serde(skip_serializing_if = "Config::is_default__e")]
    e: String,
}

fn main() {
    let serialized = serde_json::to_string(&Config::default()).unwrap();

    println!(
        "Nothing is emitted for smart default values: {}",
        serialized
    );

    let deserialized: Config = serde_json::from_str(&serialized).unwrap();
    println!("Deserialized: {:?}", deserialized);
}
