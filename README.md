# smart-is-default

> A tiny `#[derive]` macro that generates per-field `is_default__*` helpers, designed to be paired with `smart-default` and `serde`'s `skip_serializing_if` so JSON output omits fields that match their custom defaults.

## Why

`serde` lets you skip serialization with `skip_serializing_if = "Type::is_default__field"`, but you have to write that helper yourself for every field, and it has to compare against the _actual_ `Default::default()` value of the type.

`smart-is-default` generates those helpers for you. It works best alongside [`smart-default`](https://crates.io/crates/smart-default), which is what produces the custom `Default::default()` in the first place. Together they form a one-liner: a default-bearing struct serializes to `{}` when fully default, and emits only the fields that actually differ from default.

## What it does

Given a struct, `#[derive(SmartIsDefault)]` emits one associated function per named field:

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
}
```

## Supported types

| Struct shape                         | Support |
| ------------------------------------ | :-----: |
| Named fields (`struct Foo { x: T }`) |   yes   |
| Tuple structs (`struct Foo(T, U)`)   |   no    |
| Unit structs                         |   no    |
| Enums (variants)                     |   no    |

For generic structs the helpers are emitted inside a `impl<T: ...>` block, so the same `Pair<T>::is_default__first` works for any `T` whose default is comparable:

```rust
#[derive(SmartDefault, SmartIsDefault, Serialize, Deserialize)]
struct Pair<T: Default + PartialEq> {
    #[default(T::default())]
    #[serde(skip_serializing_if = "Pair::is_default__first", default)]
    first: T,
    // …
}
```
