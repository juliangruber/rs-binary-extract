use json::{self, JsonValue};

#[derive(Debug)]
pub enum ExtractError {
    JsonError(json::Error),
    KeyNotFound(),
    JsonTooShort(),
    MissingEnd(),
}

impl From<json::Error> for ExtractError {
    fn from(err: json::Error) -> Self {
        ExtractError::JsonError(err)
    }
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
    let key_decorated = format!("\"{key}\"");
    let mut it = s.chars().enumerate();

    while let Some((i, c)) = it.next()  {
        match c {
            '\\' => {
                it.nth(0);
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
                ':' => is_key = false,
                ',' => is_key = true,
                '{' => level = level + 1,
                '}' => level = level - 1,
                _ => (),
            }
        }
        if is_key && level == 1 && i > 0 {
            if let Some(sub) = s.get(i - 1..i + key.len() + 1) {
                if sub == key_decorated {
                    let start = i + key.len() + 2;
                    let end = find_end(&s, start)?;
                    return Ok(json::parse(&s[start..end])?);
                }
            }
        }

    }

    Err(ExtractError::KeyNotFound())
}

fn find_end(s: &str, start: usize) -> Result<usize, ExtractError> {
    if s.len() <= start {
        return Err(ExtractError::JsonTooShort());
    }

    let mut level = 0;
    let mut first_char: Option<char> = Default::default();
    let mut chars = s.chars();
    chars.nth(start - 1);

    for i in start..(s.len()) {
        let c = chars.next().unwrap();
        if let None = first_char {
            first_char = Some(c);
        }
        match c {
            '{' | '[' => {
                level = level + 1;
                continue;
            }
            '}' | ']' => {
                level = level - 1;
                if level > 0 {
                    continue;
                }
            }
            _ => ()
        }
        if level < 0 || level == 0 && (c == ',' || c == '}' || c == ']') {
            return match first_char {
                Some('{') | Some('[') => Ok(i + 1),
                _ => Ok(i),
            };
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
