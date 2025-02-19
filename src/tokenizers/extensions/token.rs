use std::hash::{Hash, Hasher};

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

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenExtensionID {
    pub id: u32,
    #[serde(flatten)]
    pub token: TokenExtension,
}
