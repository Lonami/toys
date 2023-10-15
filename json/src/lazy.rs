use std::borrow::Cow;

/// A lazily-parsed JSON-like text which assumes well-formed data.
///
/// Features such as arbitrary escape sequences are not supported.
///
/// Malformed data such as lists with invalid or empty items may still be parsed.
#[derive(Debug, PartialEq)]
pub struct LazyJson<'j> {
    json: &'j str,
}

#[derive(Debug)]
pub struct LazyJsonItems<'j> {
    json: &'j str,
    offset: usize,
}

#[derive(Debug)]
pub struct LazyJsonEntries<'j> {
    json: &'j str,
    offset: usize,
}

fn find_next_likely_json_token(json: &[u8]) -> Option<(usize, usize)> {
    for (i, b) in json.iter().copied().enumerate() {
        match b {
            b' ' | b'\t' | b'\r' | b'\n' => continue,
            b't' => {
                if json.get(i..i + 4) == Some(b"true") {
                    return Some((i, i + 4));
                } else {
                    return None;
                }
            }
            b'f' => {
                if json.get(i..i + 5) == Some(b"false") {
                    return Some((i, i + 5));
                } else {
                    return None;
                }
            }
            b'+' | b'-' | b'.' | b'0'..=b'9' => {
                return Some((
                    i,
                    i + 1
                        + json[i + 1..]
                            .iter()
                            .position(|c| !matches!(c, b'+' | b'-' | b'.' | b'0'..=b'9'))
                            .unwrap_or(json.len()),
                ));
            }
            b'"' => {
                let mut skip_next = false;
                for (j, c) in json[i + 1..].iter().copied().enumerate() {
                    if skip_next {
                        continue;
                    }
                    if c == b'"' {
                        return Some((i, i + j + 2));
                    }
                    skip_next = c == b'\\'
                }
                return None;
            }
            b',' | b':' | b'[' | b'{' | b']' | b'}' => return Some((i, i + 1)),
            _ => return None,
        }
    }
    None
}

fn find_next_likely_json_expr(json: &[u8]) -> Option<(usize, usize)> {
    let (nesting_start, mut bracket_depth, mut brace_depth) =
        match find_next_likely_json_token(json) {
            Some((start, end)) => match &json[start..end] {
                b"[" => (start, 1, 0),
                b"{" => (start, 0, 1),
                b"]" | b"}" => return None,
                _ => return Some((start, end)),
            },
            None => return None,
        };

    let mut offset = nesting_start + 1;
    while let Some((start, end)) = find_next_likely_json_token(&json[offset..]) {
        match &json[offset + start..offset + end] {
            b"[" => bracket_depth += 1,
            b"{" => brace_depth += 1,
            b"]" => {
                bracket_depth -= 1;
                if bracket_depth < 0 {
                    return None;
                }
                if bracket_depth == 0 {
                    if brace_depth == 0 {
                        return Some((nesting_start, offset + end));
                    } else {
                        return None;
                    }
                }
            }
            b"}" => {
                brace_depth -= 1;
                if brace_depth < 0 {
                    return None;
                }
                if brace_depth == 0 {
                    if bracket_depth == 0 {
                        return Some((nesting_start, offset + end));
                    } else {
                        return None;
                    }
                }
            }
            _ => {}
        }
        offset += end;
    }

    None
}

impl<'j> LazyJson<'j> {
    pub fn new(json: &'j str) -> Self {
        Self { json: json.trim() }
    }

    pub fn to_bool(&self) -> Option<bool> {
        if self.json == "true" {
            Some(true)
        } else if self.json == "false" {
            Some(false)
        } else {
            None
        }
    }

    pub fn to_f64(&self) -> Option<f64> {
        self.json.parse().ok()
    }

    pub fn to_string(&self) -> Option<Cow<'j, str>> {
        if !self.delimited_by(b'"', b'"') {
            return None;
        }

        let mut buffer = Vec::new();
        let mut expecting_escape = false;
        for (i, b) in self.json.as_bytes()[1..self.json.len() - 1]
            .iter()
            .copied()
            .enumerate()
        {
            if expecting_escape {
                if b != b'\\' && b != b'"' {
                    return None;
                }
                if buffer.is_empty() {
                    buffer.reserve(self.json.len() - 3);
                    buffer.extend(&self.json.as_bytes()[1..i]);
                }
                buffer.push(b);
                expecting_escape = false;
            } else if b == b'"' {
                return None;
            } else if b == b'\\' {
                expecting_escape = true;
            } else if !buffer.is_empty() {
                buffer.push(b);
            }
        }

        if expecting_escape {
            return None;
        }

        Some(if buffer.is_empty() {
            Cow::Borrowed(&self.json[1..self.json.len() - 1])
        } else {
            Cow::Owned(String::from_utf8(buffer).unwrap())
        })
    }

    pub fn items(&self) -> Option<LazyJsonItems> {
        if self.delimited_by(b'[', b']') {
            Some(LazyJsonItems {
                json: self.json,
                offset: 1,
            })
        } else {
            None
        }
    }

    pub fn entries(&self) -> Option<LazyJsonEntries> {
        if self.delimited_by(b'{', b'}') {
            Some(LazyJsonEntries {
                json: self.json,
                offset: 1,
            })
        } else {
            None
        }
    }

    fn delimited_by(&self, left: u8, right: u8) -> bool {
        !self.json.is_empty()
            && self.json.as_bytes()[0] == left
            && self.json.as_bytes()[self.json.len() - 1] == right
    }
}

impl<'j> Iterator for LazyJsonItems<'j> {
    type Item = LazyJson<'j>;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some((start, end)) =
            find_next_likely_json_expr(&self.json[self.offset..].as_bytes())
        {
            let token = &self.json[self.offset + start..self.offset + end];
            self.offset += end;
            if token != "," {
                return Some(LazyJson { json: token });
            }
        }
        self.offset = self.json.len();
        None
    }
}

impl<'j> Iterator for LazyJsonEntries<'j> {
    type Item = (Cow<'j, str>, LazyJson<'j>);

    fn next(&mut self) -> Option<Self::Item> {
        dbg!(&self.json[self.offset..]);
        let mut key = None;
        let mut colon = false;
        while let Some((start, end)) =
            find_next_likely_json_expr(&self.json[self.offset..].as_bytes())
        {
            let token = &self.json[self.offset + start..self.offset + end];
            self.offset += end;
            if token == "," {
                continue;
            }
            if key.is_none() {
                match (LazyJson { json: token }).to_string() {
                    Some(k) => key = Some(k),
                    None => break,
                }
            } else if !colon {
                if token == ":" {
                    colon = true;
                } else {
                    break;
                }
            } else {
                return Some((key.unwrap(), LazyJson { json: token }));
            }
        }
        self.offset = self.json.len();
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bools() {
        let json = LazyJson::new("true");
        assert_eq!(json.to_bool(), Some(true));
        let json = LazyJson::new("false");
        assert_eq!(json.to_bool(), Some(false));
    }

    #[test]
    fn bad_bools() {
        let json = LazyJson::new("True");
        assert_eq!(json.to_bool(), None);
        let json = LazyJson::new("False");
        assert_eq!(json.to_bool(), None);
    }

    #[test]
    fn integers() {
        let json = LazyJson::new("42");
        assert_eq!(json.to_f64(), Some(42.0));
    }

    #[test]
    fn bad_integers() {
        let json = LazyJson::new("4d2");
        assert_eq!(json.to_f64(), None, "non-digit character");
    }

    #[test]
    fn decimals() {
        let json = LazyJson::new("4.25");
        assert_eq!(json.to_f64(), Some(4.25));
    }

    #[test]
    fn bad_decimals() {
        let json = LazyJson::new("4,25");
        assert_eq!(json.to_f64(), None, "incorrect decimal separator");
    }

    #[test]
    fn strings() {
        let json = LazyJson::new("\"blogen\"");
        assert_eq!(json.to_string(), Some(Cow::Borrowed("blogen")), "simple");
        let json = LazyJson::new("\"\\\\blogen\\\"\"");
        assert_eq!(
            json.to_string(),
            Some(Cow::Borrowed("\\blogen\"")),
            "escape backslash first at start"
        );
        let json = LazyJson::new("\"\\\"blogen\\\\\"");
        assert_eq!(
            json.to_string(),
            Some(Cow::Borrowed("\"blogen\\")),
            "escape quote first at start"
        );
        let json = LazyJson::new("\"b\\\\loge\\\"n\"");
        assert_eq!(
            json.to_string(),
            Some(Cow::Borrowed("b\\loge\"n")),
            "escape backslash first after start"
        );
        let json = LazyJson::new("\"b\\\"loge\\\\n\"");
        assert_eq!(
            json.to_string(),
            Some(Cow::Borrowed("b\"loge\\n")),
            "escape quote first after start"
        );
    }

    #[test]
    fn bad_strings() {
        let json = LazyJson::new("'blogen'");
        assert_eq!(json.to_string(), None, "incorrect quotes");
        let json = LazyJson::new("\"blogen");
        assert_eq!(json.to_string(), None, "missing end quote");
        let json = LazyJson::new("blogen\"");
        assert_eq!(json.to_string(), None, "missing start quote");
        let json = LazyJson::new("\"blo\\gen\"");
        assert_eq!(json.to_string(), None, "unrecognised escape");
        let json = LazyJson::new("\"blogen\\\"");
        assert_eq!(json.to_string(), None, "escaped end quote");
    }

    #[test]
    fn lists() {
        let json = LazyJson::new("[]");
        let mut items = json.items().expect("empty");
        assert_eq!(items.next(), None);

        let json = LazyJson::new("[true,false]");
        let mut items = json.items().expect("booleans");
        assert_eq!(items.next().unwrap().to_bool().unwrap(), true);
        assert_eq!(items.next().unwrap().to_bool().unwrap(), false);
        assert_eq!(items.next(), None);

        let json = LazyJson::new("[ 1  ,  2 ]");
        let mut items = json.items().expect("numbers with spaces");
        assert_eq!(items.next().unwrap().to_f64().unwrap(), 1f64);
        assert_eq!(items.next().unwrap().to_f64().unwrap(), 2f64);
        assert_eq!(items.next(), None);

        let json = LazyJson::new("[\"[blogen]\"]");
        let mut items = json.items().expect("string with brackets");
        assert_eq!(
            items.next().unwrap().to_string().unwrap(),
            Cow::Borrowed("[blogen]")
        );
        assert_eq!(items.next(), None);

        let json = LazyJson::new("[1, \"blogen\", true]");
        let mut items = json.items().expect("mixed");
        assert_eq!(items.next().unwrap().to_f64().unwrap(), 1f64);
        assert_eq!(
            items.next().unwrap().to_string().unwrap(),
            Cow::Borrowed("blogen")
        );
        assert_eq!(items.next().unwrap().to_bool().unwrap(), true);
        assert_eq!(items.next(), None);

        let json = LazyJson::new("[1, [2, 3], 4]");
        let mut items = json.items().expect("nested");
        assert_eq!(items.next().unwrap().to_f64().unwrap(), 1f64);
        let nested = items.next().unwrap();
        assert_eq!(items.next().unwrap().to_f64().unwrap(), 4f64);
        assert_eq!(items.next(), None);
        let mut items = nested.items().expect("nested");
        assert_eq!(items.next().unwrap().to_f64().unwrap(), 2f64);
        assert_eq!(items.next().unwrap().to_f64().unwrap(), 3f64);
        assert_eq!(items.next(), None);

        let json = LazyJson::new("[1, }, 2]");
        let mut items = json.items().expect("numbers malformed");
        assert_eq!(items.next().unwrap().to_f64().unwrap(), 1f64);
        assert_eq!(items.next(), None);
    }

    #[test]
    fn objects() {
        let json = LazyJson::new("{}");
        let mut entries = json.entries().expect("empty");
        assert_eq!(entries.next(), None);

        let json = LazyJson::new("{\"blogen\":true}");
        let mut entries = json.entries().expect("booleans");
        let (key, item) = entries.next().unwrap();
        assert_eq!(key, "blogen");
        assert_eq!(item.to_bool().unwrap(), true);
        assert_eq!(entries.next(), None);

        let json = LazyJson::new("{\"blogen\": {\"blo\": 1, \"gen\": 2}}");
        let mut entries = json.entries().expect("nested");
        let (key, nested) = entries.next().unwrap();
        assert_eq!(key, "blogen");
        assert_eq!(entries.next(), None);
        let mut entries = nested.entries().expect("nested");
        let (key, item) = entries.next().unwrap();
        assert_eq!(key, "blo");
        assert_eq!(item.to_f64().unwrap(), 1f64);
        let (key, item) = entries.next().unwrap();
        assert_eq!(key, "gen");
        assert_eq!(item.to_f64().unwrap(), 2f64);
        assert_eq!(entries.next(), None);
    }
}
