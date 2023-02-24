use json::{self, JsonValue};

pub fn extract(s: &str, key: &str) -> Result<JsonValue, &'static str> {
    let mut in_string = false;
    let mut is_key = true;
    let mut level = 0;
    let mut skip_next = false;

    for i in 0..s.len() {
        let c = s.chars().nth(i).unwrap();
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
        let prev = s.chars().nth(i - 1).unwrap();
        let key_slice = &s[i..i + key.len()];
        let after = s.chars().nth(i + key.len()).unwrap();
        if prev == '\"' && key_slice == key && after == '\"' {
            let start = i + key.len() + 2;
            match find_end(&s, start) {
                Ok(end) => {
                    let parsed = json::parse(&s[start..end]).unwrap();
                    return Ok(parsed);
                }
                Err(err) => return Err(err),
            }
        }
    }

    Err("key not found")
}

fn find_end(buf: &str, start: usize) -> Result<usize, &'static str> {
    if buf.len() <= start {
        return Err("json too short");
    }

    let mut level = 0;
    let s = buf.chars().nth(start).unwrap();

    for i in start..(buf.len()) {
        let c = buf.chars().nth(i).unwrap();
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
            if s == '{' || s == '[' {
                return Ok(i + 1);
            } else {
                return Ok(i);
            }
        }
    }

    Err("missing end")
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
