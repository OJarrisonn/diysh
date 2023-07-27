use std::fmt::Display;

pub struct Prompt(pub String);
pub struct PromptHeader(pub String);

impl From<&str> for Prompt {
    fn from(value: &str) -> Self {
        Self(String::from(value))
    }
}

impl From<&str> for PromptHeader {
    fn from(value: &str) -> Self {
        Self(String::from(value))
    }
}

impl Display for Prompt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Display for PromptHeader {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}