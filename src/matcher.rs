use regex::Regex;
use std::collections::HashSet;

pub enum Matcher {
    Exact { name: String },
    Wildcard { pattern: String },
    Set { names: HashSet<String> },
    Regex { regex: Regex },
}

impl Matcher {
    pub fn matches(&self, query: String) -> bool {
        match self {
            Self::Exact { name } => name == &query,
            Self::Wildcard { pattern } => unimplemented!(), // FIXME
            Self::Set { names } => names.contains(&query),
            Self::Regex { regex } => regex.is_match(&query),
        }
    }
}
