use std::collections::{HashMap, HashSet};

use aho_corasick::{AhoCorasick, AhoCorasickBuilder, MatchKind};
use lazy_static::lazy_static;
use regex::Regex;
use serde::{ser::SerializeSeq, Serialize};

use super::token::{TokenExtension, TokenExtensionID};

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

type MatchingSet = (AhoCorasick, Vec<u32>);

#[derive(Debug, Clone)]
pub struct VocabExtension {
    added_tokens_map: HashMap<String, u32>,
    added_tokens_map_r: HashMap<u32, TokenExtension>,
    added_tokens: Vec<TokenExtension>,
    special_tokens_set: HashSet<String>,
    special_tokens: Vec<TokenExtension>,
    split_trie: MatchingSet,
    split_normalized_trie: MatchingSet,
    encode_special_tokens: bool,
}

impl VocabExtension {
    pub fn new() -> Self {
        let trie = AhoCorasickBuilder::new()
            .match_kind(MatchKind::LeftmostLongest)
            .build::<_, &&[u8]>([])
            .unwrap();

        let normalized_trie = AhoCorasickBuilder::new()
            .match_kind(MatchKind::LeftmostLongest)
            .build::<_, &&[u8]>([])
            .unwrap();

        Self {
            added_tokens_map: HashMap::new(),
            added_tokens_map_r: HashMap::new(),
            added_tokens: vec![],
            special_tokens_set: HashSet::new(),
            special_tokens: vec![],
            split_trie: (trie, vec![]),
            split_normalized_trie: (normalized_trie, vec![]),
            encode_special_tokens: false,
        }
    }

    pub fn len(&self) -> usize {
        self.added_tokens_map.len()
    }
}

impl Serialize for VocabExtension {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut token_extension_id =
            self.added_tokens_map_r
                .iter()
                .map(|(id, token)| TokenExtensionID {
                    id: *id,
                    token: token.clone(),
                })
                .collect::<Vec<_>>();

            token_extension_id.sort_by_key(|token| token.id);

            let mut vocab = serializer.serialize_seq(Some(token_extension_id.len())).unwrap();
            for token in token_extension_id {
                vocab.serialize_element(&token).unwrap();
            }

            vocab.end()
    }
}
