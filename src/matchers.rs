use regex::Regex;
use std::collections::HashSet;

enum Matcher {
    Exact { name: String },
    Set { names: HashSet<String> },
    Regex { regex: Regex },
}

impl Matcher {
    pub fn matches(&self, query: String) -> bool {
        match self {
            Self::Exact { name } => name == &query,
            Self::Set { names } => names.contains(&query),
            Self::Regex { regex } => regex.is_match(&query),
        }
    }
}
