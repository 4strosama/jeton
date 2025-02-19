use std::collections::{HashMap, HashSet};
use std::hash::{Hash, Hasher};

use lazy_static::lazy_static;
use regex::Regex;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenExtension {
    pub content: String,
    pub is_single: bool,
    pub is_normalized: bool,
    pub is_special: bool,
    pub is_stripped: (bool, bool),
}

impl Default for TokenExtension {
    fn default() -> Self {
        Self {
            content: String::new(),
            is_single: false,
            is_normalized: false,
            is_special: false,
            is_stripped: (false, false),
        }
    }
}

impl TokenExtension {
    pub fn from<S: Into<String>>(content: S, special: bool) -> Self {
        Self {
            content: content.into(),
            is_normalized: !special,
            is_special: special,
            ..Default::default()
        }
    }

    #[must_use]
    pub fn set_single(mut self, single: bool) -> Self {
        self.is_single = single;
        self
    }

    #[must_use]
    pub fn set_normalized(mut self, normalized: bool) -> Self {
        self.is_normalized = normalized;
        self
    }

    #[must_use]
    pub fn is_special(mut self, special: bool) -> Self {
        self.is_special = special;
        self
    }

    #[must_use]
    pub fn is_stripped(mut self, left: bool, right: bool) -> Self {
        self.is_stripped = (left, right);
        self
    }
}

impl Hash for TokenExtension {
    fn hash<H: Hasher>(&self, state: &mut H) -> () {
        self.content.hash(state);
    }
}

lazy_static! {
    static ref STARTS_WITH_WORD: Regex = Regex::new(r"^\w").unwrap();
    static ref ENDS_WITH_WORD: Regex = Regex::new(r"\w$").unwrap();
    static ref LEFTMOST_SPACE_AT_END: Regex = Regex::new(r"\s*$").unwrap();
    static ref RIGHTMOST_SPACE_AT_START: Regex = Regex::new(r"^\s*").unwrap();
}

fn starts_with_word(sentence: &str) -> bool {
    STARTS_WITH_WORD.is_match(sentence)
}

fn ends_with_word(sentence: &str) -> bool {
    ENDS_WITH_WORD.is_match(sentence)
}

fn space_leftmost_at_end(sentence: &str) -> usize {
    if let Some(target) = LEFTMOST_SPACE_AT_END.find(sentence) {
        target.start()
    } else {
        sentence.len()
    }
}

fn space_rightmost_at_start(sentence: &str) -> usize {
    if let Some(target) = RIGHTMOST_SPACE_AT_START.find(sentence) {
        target.end()
    } else {
        0
    }
}

#[derive(Debug, Clone)]
pub struct VocabExtension {}
