# smart-is-default

> A tiny `#[derive]` macro that generates per-field `is_default__*` helpers, designed to be paired with `smart-default` and `serde`'s `skip_serializing_if` so JSON output omits fields that match their custom defaults.

## Why

`serde` lets you skip serialization with `skip_serializing_if = "Type::is_default__field"`, but you have to write that helper yourself for every field, and it has to compare against the _actual_ `Default::default()` value of the type.

`smart-is-default` generates those helpers for you. It works best alongside [`smart-default`](https://crates.io/crates/smart-default), which is what produces the custom `Default::default()` in the first place. Together they form a one-liner: a default-bearing struct serializes to `{}` when fully default, and emits only the fields that actually differ from default.

## What it does

Given a named-field struct, `#[derive(SmartIsDefault)]` emits one associated function per field:

```rust
#[derive(SmartIsDefault)]
struct Config {
    a: i32,
    e: String,
}
```

…expands to:

```rust
impl Config {
    fn is_default__a(v: &i32) -> bool { v == &Self::default().a }
    fn is_default__e(v: &String) -> bool { v == &Self::default().e }
    fn is_default(v: &Self) -> bool { v == &Self::default() }
}
```

Add `#[smart_is_default(no_is_default)]` or `#[smart_is_default(skip)]` to suppress `is_default`.

```rust
#[derive(SmartDefault, SmartIsDefault, PartialEq, Debug)]
#[smart_is_default(no_is_default)]
struct Item {
    #[default = 0]
    count: i32,

    #[smart_is_default(skip)]
    #[default = 0]
    skipped: i32,
}

impl Item {
    fn is_default(&self) -> bool { // You can now implement your own `is_default`.
        self.count == 0
    }
}
```

## Supported types

| Type shape                           |      Support      |
| ------------------------------------ | :---------------: |
| Named fields (`struct Foo { x: T }`) |        yes        |
| Tuple structs (`struct Foo(T, U)`)   |   yes (indices)   |
| Unit structs                         | `is_default` only |
| Enums                                | `is_default` only |
