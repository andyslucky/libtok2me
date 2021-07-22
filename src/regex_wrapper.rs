use regex::*;
use serde::de::{Deserializer, Error as DeError, Visitor};
use serde::Deserialize;
use std::{fmt, str::FromStr};

#[derive(Debug,Clone)]
pub struct RegexWrapper(pub Regex);

impl From<String> for RegexWrapper {
    fn from(val : String) -> Self {
        return RegexWrapper(Regex::new(val.as_str()).unwrap());
    }
}

impl FromStr for RegexWrapper {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        return Ok(RegexWrapper(Regex::new(s).unwrap()));
    }
}

struct REVisitor;

impl REVisitor {
    fn wrap<T>(&self, string: T) -> String
    where
        T: AsRef<str>,
    {
        let mut temp = String::from("^");
        temp.push_str(string.as_ref());
        temp.push('$');
        return temp;
    }
}

impl<'de> Visitor<'de> for REVisitor {
    type Value = RegexWrapper;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a valid regex")
    }

    fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
    where
        E: DeError,
    {
        Ok(RegexWrapper(Regex::new(self.wrap(v).as_str()).unwrap()))
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: DeError,
    {
        Ok(RegexWrapper(Regex::new(self.wrap(v).as_str()).unwrap()))
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