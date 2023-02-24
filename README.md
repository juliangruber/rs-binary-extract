# binary-extract
Extract a value from a json blob without parsing the whole thing

## Example

```rs
use binary_extract;

let value = binary_extract::extract(r#"{"foo": "bar"}"#, "foo").unwrap();
assert_eq!(value, "bar");
```