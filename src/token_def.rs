use crate::regex_wrapper::*;
use serde::Deserialize;

/// Token has a type and a value
#[derive(Debug, Clone)]
pub struct Token {
    pub token_type: Option<String>,
    pub token_value: String,
}

impl AsRef<Token> for Token {
    fn as_ref(&self) -> &Token {
        return self;
    }
}

///
#[derive(Debug, Deserialize, Clone)]
pub struct TokenDef {
    pub token_type: String,
    pub exprs: Vec<RegexWrapper>,
}
