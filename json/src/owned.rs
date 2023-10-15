use std::collections::HashMap;
use std::fmt::{self, Write as _};

/// An owned JSON value.
///
/// This can later be serialized to a JSON string by using its `Display` implementation.
#[derive(Debug, PartialEq)]
pub enum JsonValue {
    Boolean(bool),
    Number(f64),
    String(String),
    List(Vec<JsonValue>),
    Object(HashMap<String, JsonValue>),
}

impl From<bool> for JsonValue {
    fn from(value: bool) -> Self {
        Self::Boolean(value)
    }
}

macro_rules! impl_from_number {
    ($($ty:ident)+) => {
        $(
            impl From<$ty> for JsonValue {
                fn from(value: $ty) -> Self {
                    Self::Number(value.into())
                }
            }
        )+
    };
}

impl_from_number![i8 u8 i16 u16 i32 u32 f32 f64];

impl From<&str> for JsonValue {
    fn from(value: &str) -> Self {
        Self::String(value.into())
    }
}

impl From<String> for JsonValue {
    fn from(value: String) -> Self {
        Self::String(value)
    }
}

impl<J: Into<JsonValue>> From<Vec<J>> for JsonValue {
    fn from(value: Vec<J>) -> Self {
        Self::List(value.into_iter().map(|v| v.into()).collect())
    }
}

impl From<HashMap<&str, JsonValue>> for JsonValue {
    fn from(value: HashMap<&str, JsonValue>) -> Self {
        Self::Object(value.into_iter().map(|(k, v)| (k.into(), v)).collect())
    }
}

impl From<HashMap<String, JsonValue>> for JsonValue {
    fn from(value: HashMap<String, JsonValue>) -> Self {
        Self::Object(value)
    }
}

fn write_str(f: &mut fmt::Formatter, string: &str) -> fmt::Result {
    f.write_char('"')?;
    for c in string.chars() {
        if c == '"' || c == '\\' {
            f.write_char('\\')?;
        }
        f.write_char(c)?;
    }
    f.write_char('"')
}

impl fmt::Display for JsonValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Boolean(x) => write!(f, "{}", if *x { "true" } else { "false" }),
            Self::Number(x) => write!(f, "{}", x),
            Self::String(x) => write_str(f, x),
            Self::List(x) => {
                let mut pre = "[";
                for v in x {
                    f.write_str(pre)?;
                    v.fmt(f)?;
                    pre = ", ";
                }
                f.write_char(']')
            }
            Self::Object(x) => {
                let mut pre = "{";
                for (k, v) in x {
                    f.write_str(pre)?;
                    write_str(f, k)?;
                    f.write_str(": ")?;
                    v.fmt(f)?;
                    pre = ", ";
                }
                f.write_char('}')
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn boolean() {
        let value = JsonValue::from(true);
        assert_eq!(value.to_string(), "true");
        let value = JsonValue::from(false);
        assert_eq!(value.to_string(), "false");
    }

    #[test]
    fn number() {
        let value = JsonValue::from(42);
        assert_eq!(value.to_string(), "42");
        let value = JsonValue::from(4.25);
        assert_eq!(value.to_string(), "4.25");
    }

    #[test]
    fn string() {
        let value = JsonValue::from("blogen");
        assert_eq!(value.to_string(), "\"blogen\"");
        let value = JsonValue::from("\"blogen\\");
        assert_eq!(value.to_string(), "\"\\\"blogen\\\\\"");
    }

    #[test]
    fn list() {
        let value = JsonValue::from(vec![1, 2, 3]);
        assert_eq!(value.to_string(), "[1, 2, 3]");
        let value = JsonValue::from(vec![vec![1], vec![2]]);
        assert_eq!(value.to_string(), "[[1], [2]]");
    }

    #[test]
    fn object() {
        let mut value = HashMap::new();
        value.insert("blo", JsonValue::from(vec![1, 2]));
        value.insert("gen", JsonValue::from(true));
        let value = JsonValue::from(value);
        assert_eq!(value.to_string(), "{\"blo\": [1, 2], \"gen\": true}");
    }
}
