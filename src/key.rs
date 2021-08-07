use crate::matcher;

#[derive(Clone, Default)]
pub struct Key {
    pub ctrl: bool,
    pub alt: bool,
    pub shift: bool,

    pub word: &'static str,
}

impl Key {
    pub fn matches_pattern(&self, pattern: &str) -> bool {
        matcher::pattern_matches_key(pattern, &self)
    }
}
