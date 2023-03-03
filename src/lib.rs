use json::{self, JsonValue};

#[derive(Debug)]
pub enum ExtractError {
    JsonError(json::Error),
    KeyNotFound(),
    JsonTooShort(),
    MissingEnd(),
}

/// Extract a value from a json string without parsing the whole thing.
///
/// With the case from [benches/json.rs](benches/json.rs), this is ~3x
/// faster than using the `json` crate directly.
///
/// # Examples
///
/// ```
/// let value = binary_extract::extract(r#"{"foo": "bar"}"#, "foo").unwrap();
/// assert_eq!(value, "bar");
/// ```
pub fn extract(s: &str, key: &str) -> Result<JsonValue, ExtractError> {
    let mut in_string = false;
    let mut is_key = true;
    let mut level = 0;
    let mut skip_next = false;
    let key_decorated = format!("\"{key}\"");
    let mut chars = s.chars();

    for i in 0..s.len() {
        let c = chars.next().unwrap();
        if skip_next {
            skip_next = false;
            continue;
        }
        match c {
            '\\' => {
                skip_next = true;
                continue;
            }
            '"' => {
                in_string = !in_string;
                continue;
            }
            _ => (),
        }
        if !in_string {
            match c {
                ':' => {
                    is_key = false;
                }
                ',' => {
                    is_key = true;
                }
                '{' => {
                    level = level + 1;
                }
                '}' => {
                    level = level - 1;
                }
                _ => (),
            }
        }
        if !is_key || level > 1 || i == 0 {
            continue;
        }
        if &s[i - 1..i + key.len() + 1] == key_decorated {
            let start = i + key.len() + 2;
            match find_end(&s, start) {
                Ok(end) => {
                    match json::parse(&s[start..end]) {
                        Ok(parsed) => return Ok(parsed),
                        Err(err) => return Err(ExtractError::JsonError(err)),
                    };
                }
                Err(err) => return Err(err),
            }
        }
    }

    Err(ExtractError::KeyNotFound())
}

fn find_end(buf: &str, start: usize) -> Result<usize, ExtractError> {
    if buf.len() <= start {
        return Err(ExtractError::JsonTooShort());
    }

    let mut level = 0;
    let mut s: Option<char> = Default::default();
    let mut buf_chars = buf.chars();
    buf_chars.nth(start - 1);

    for i in start..(buf.len()) {
        let c = buf_chars.next().unwrap();
        if let None = s {
            s = Some(c);
        }
        if c == '{' || c == '[' {
            level = level + 1;
            continue;
        } else if c == '}' || c == ']' {
            level = level - 1;
            if level > 0 {
                continue;
            }
        }
        if level < 0 || level == 0 && (c == ',' || c == '}' || c == ']') {
            if let Some('{') = s {
                return Ok(i + 1);
            } else if let Some('[') = s {
                return Ok(i + 1);
            } else {
                return Ok(i);
            }
        }
    }

    Err(ExtractError::MissingEnd())
}

#[cfg(test)]
mod tests {
    use super::*;
    use json::{array, object};

    #[test]
    fn test() {
        let value = extract(r#"{"foo": "bar"}"#, "foo").unwrap();
        assert_eq!(value, "bar");

        let value = extract(r#"{"foo": "bar","bar":"baz"}"#, "foo").unwrap();
        assert_eq!(value, "bar");

        let value = extract(r#"{"foo": "bar","bar":"baz"}"#, "bar").unwrap();
        assert_eq!(value, "baz");

        let value = extract(r#"{"foo":{"beep":"boop","bar":"oops"},"bar":"baz"}"#, "bar").unwrap();
        assert_eq!(value, "baz");

        let value = extract(r#"{"foo":[{"bar":"oops"}],"bar":"baz"}"#, "bar").unwrap();
        assert_eq!(value, "baz");

        let value = extract(r#"{"foo":{"bar":"baz"}}"#, "foo").unwrap();
        assert_eq!(
            value,
            object! {
                bar: "baz"
            }
        );

        let value = extract(r#"{"foo":["bar","baz"]}"#, "foo").unwrap();
        assert_eq!(
            value,
            array! {
                "bar",
                "baz"
            }
        );

        let value = extract(r#"{"foo": "bar"}"#, "foo").unwrap();
        assert_eq!(value, "bar");

        let value = extract(r#"{"beep":"\\","foo":"bar"}"#, "foo").unwrap();
        assert_eq!(value, "bar");

        let value = extract(r#"{"foo":"bar\"baz"}"#, "foo").unwrap();
        assert_eq!(value, "bar\"baz");

        let value = extract(r#"{"_a":0,"a_":1,"_a_":2,"a":3}"#, "a").unwrap();
        assert_eq!(value, 3);

        extract(r#"{"foo"}"#, "foo").unwrap_err();
        extract(r#"{"foo":"bar"}"#, "bar").unwrap_err();

        let value = extract(r#"{"foo":{"bar":{"baz":"beep"}}}"#, "foo").unwrap();
        assert_eq!(
            value,
            object! {
                bar: {
                    baz: "beep"
                }
            }
        );
    }
}
