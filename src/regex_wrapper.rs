use regex::*;
use serde::de::{Deserializer, Error as DeError, Visitor};
use serde::Deserialize;
use std::{fmt};

#[derive(Debug, Clone)]
pub struct RegexWrapper(pub Regex);

impl From<String> for RegexWrapper {
    fn from(val: String) -> Self {
        return RegexWrapper(Regex::new(&wrap(val)).unwrap());
    }
}

impl From<&str> for RegexWrapper {
    fn from(v: &str) -> Self {
        return RegexWrapper(Regex::new(&wrap(v)).unwrap());
    }
}

fn wrap<T>(string: T) -> String
where
    T: AsRef<str>,
{
    let mut temp = String::from("^");
    temp.push_str(string.as_ref());
    temp.push('$');
    return temp;
}

struct REVisitor;

impl REVisitor {}

impl<'de> Visitor<'de> for REVisitor {
    type Value = RegexWrapper;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a valid regex")
    }

    fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
    where
        E: DeError,
    {
        Ok(RegexWrapper::from(v))
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: DeError,
    {
        Ok(RegexWrapper::from(v))
    }
}

impl<'de> Deserialize<'de> for RegexWrapper {
    fn deserialize<D>(deserializer: D) -> Result<RegexWrapper, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_any(REVisitor)
    }
}
