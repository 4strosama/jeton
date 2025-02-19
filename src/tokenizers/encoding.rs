use std::collections::HashMap;
use std::ops::Range;

use serde::{Deserialize, Serialize};

use super::Offset;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub struct Encoding {
    id: Vec<u32>,
    id_type: Vec<u32>,
    tokens: Vec<String>,
    words: Vec<Option<u32>>,
    offsets: Vec<Offset>,
    special_tokens_mask: Vec<u32>,
    attention_mask: Vec<u32>,
    overflowing: Vec<Encoding>,
    sequence_ranges: HashMap<usize, Range<usize>>,
}

impl Encoding {
    pub fn new(
        id: Vec<u32>,
        id_type: Vec<u32>,
        tokens: Vec<String>,
        words: Vec<Option<u32>>,
        offsets: Vec<Offset>,
        special_tokens_mask: Vec<u32>,
        attention_mask: Vec<u32>,
        overflowing: Vec<Encoding>,
        sequence_ranges: HashMap<usize, Range<usize>>,
    ) -> Self {
        Self {
            id,
            id_type,
            tokens,
            words,
            offsets,
            special_tokens_mask,
            attention_mask,
            overflowing,
            sequence_ranges,
        }
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            id: Vec::with_capacity(capacity),
            id_type: Vec::with_capacity(capacity),
            tokens: Vec::with_capacity(capacity),
            words: Vec::with_capacity(capacity),
            offsets: Vec::with_capacity(capacity),
            special_tokens_mask: Vec::with_capacity(capacity),
            attention_mask: Vec::with_capacity(capacity),
            overflowing: Vec::new(),
            sequence_ranges: HashMap::new(),
        }
    }
}
