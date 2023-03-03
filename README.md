# binary-extract
Extract a value from a json string without parsing the whole thing.

[📖 Docs](https://docs.rs/binary-extract/latest/binary_extract/)

[📦 Crate](https://crates.io/crates/binary-extract)

## Installation

```bash
$ cargo add binary-extract
```

## Example

```rust
let value = binary_extract::extract(r#"{"foo": "bar"}"#, "foo").unwrap();
assert_eq!(value, "bar");
```

## Perf

With the object from [benches/json.rs](benches/json.rs), `extract()` is ~3x
faster than `json::parse`.

## License

MIT
