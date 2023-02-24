use json::{self, JsonValue};

/// Extract a value from a json string without parsing the whole thing
///
/// # Examples
///
/// ```
/// let value = binary_extract::extract(r#"{"foo": "bar"}"#, "foo").unwrap();
/// assert_eq!(value, "bar");
/// ```
pub fn extract(s: &str, key: &str) -> Result<JsonValue, &'static str> {
    let mut in_string = false;
    let mut is_key = true;
    let mut level = 0;
    let mut skip_next = false;
    let key_decorated = format!("\"{key}\"");

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
        if &s[i - 1..i + key.len() + 1] == key_decorated {
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

    // #[bench]
    // fn bench_binary_extract(b: &mut Bencher) {
    //     let json = "{\"properties\":{\"selected\":\"2\",\"lastName\":\"\",\"username\":\"someone\",\"category\":\"Wedding Venues\",\"firstName\":\"\",\"product\":\"planner\",\"location\":\"\",\"platform\":\"ios\",\"email\":\"someone@yahoo.com\",\"member_id\":\"12312313123123\",\"filtered\":\"false\",\"viewed\":3},\"projectId\":\"foobarbaz\",\"userId\":\"123123123123123\",\"sessionId\":\"FF8D19D8-123123-449E-A0B9-2181C4886020\",\"requestId\":\"F3C49DEB-123123-4A54-BB72-D4BE591E4B29\",\"action\":\"Track\",\"event\":\"Vendor Category Viewed\",\"timestamp\":\"2014-04-23T20:55:19.000Z\",\"context\":{\"providers\":{\"Crittercism\":false,\"Amplitude\":false,\"Mixpanel\":false,\"Countly\":false,\"Localytics\":false,\"Google Analytics\":false,\"Flurry\":false,\"Tapstream\":false,\"Bugsnag\":false},\"appReleaseVersion\":\"2.3.1\",\"osVersion\":\"7.1\",\"os\":\"iPhone OS\",\"appVersion\":\"690\",\"screenHeight\":480,\"library-version\":\"0.10.3\",\"traits\":{\"lastName\":\"\",\"product\":\"planner\",\"member_id\":\"123123123123123\",\"firstName\":\"\",\"email\":\"someone@yahoo.com\",\"platform\":\"ios\",\"username\":\"someone\"},\"screenWidth\":320,\"deviceManufacturer\":\"Apple\",\"library\":\"analytics-ios\",\"idForAdvertiser\":\"1323232-A0ED-47AB-BE4F-274F2252E4B4\",\"deviceModel\":\"iPad3,4\"},\"requestTime\":\"2014-04-23T20:55:44.211Z\",\"version\":1,\"channel\":\"server\"}";
    //     b.iter(|| extract(json, "projectId"));
    // }
}
