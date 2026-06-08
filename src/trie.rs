//! This module provides a [trie (prefix tree)](https://en.wikipedia.org/wiki/Trie)
//! implementation for efficient string storage and lookup.
//!
//! A trie stores strings character by character. Each path from the root to a
//! node marked as a word-end represents a stored string, enabling O(m) insert
//! and search operations where m is the length of the string.

use std::collections::HashMap;

/// A single node inside the trie.
///
/// Each node maps characters to child nodes and records whether it marks the
/// end of an inserted word.
pub struct Node {
    /// The children of this node, keyed by character.
    pub children: HashMap<char, Node>,
    /// Whether the path from the root to this node forms a complete word.
    pub is_end: bool,
}

impl Node {
    /// Creates a new, empty node.
    pub fn new() -> Self {
        Node {
            children: HashMap::new(),
            is_end: false,
        }
    }
}

/// A trie (prefix tree) that stores and looks up strings efficiently.
pub struct Trie {
    root: Node,
}

impl Trie {
    /// Creates a new, empty trie.
    pub fn new() -> Self {
        Trie { root: Node::new() }
    }

    /// Inserts a word into the trie.
    pub fn insert(&mut self, word: &str) {
        let mut current = &mut self.root;
        for c in word.chars() {
            current = current.children.entry(c).or_insert_with(Node::new);
        }
        current.is_end = true;
    }

    /// Returns `true` if `word` was previously inserted into the trie.
    pub fn search(&self, word: &str) -> bool {
        let mut current = &self.root;
        for c in word.chars() {
            if let Some(n) = current.children.get(&c) {
                current = n;
            } else {
                return false;
            }
        }
        return current.is_end;
    }

    /// Returns `true` if any inserted word starts with `prefix`.
    pub fn starts_with(&self, prefix: &str) -> bool {
        let mut current = &self.root;
        for c in prefix.chars() {
            if let Some(n) = current.children.get(&c) {
                current = n;
            } else {
                return false;
            }
        }
        return true;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Searching for a word that was never inserted must return false. A naive
    /// implementation that always returns true would fail this.
    #[test]
    fn test_search_missing_word_returns_false() {
        let trie = Trie::new();

        assert!(!trie.search("hello"));
    }

    /// A word that has been inserted must be found by search. The most basic
    /// correctness check for the insert + search pair.
    #[test]
    fn test_insert_then_search_returns_true() {
        let mut trie = Trie::new();
        trie.insert("hello");

        assert!(trie.search("hello"));
    }

    /// A prefix of an inserted word is not itself a word unless it was also
    /// inserted. Without the `is_end` flag, "hell" would wrongly match "hello".
    #[test]
    fn test_prefix_of_word_is_not_a_word() {
        let mut trie = Trie::new();
        trie.insert("hello");

        assert!(!trie.search("hell"));
    }

    /// Inserting a short word must not prevent finding a longer word that
    /// extends it, and vice versa. Both must coexist independently.
    #[test]
    fn test_prefix_and_full_word_coexist() {
        let mut trie = Trie::new();
        trie.insert("he");
        trie.insert("hello");

        assert!(trie.search("he"));
        assert!(trie.search("hello"));
    }

    /// `starts_with` must return true when a matching prefix exists, even if
    /// the prefix itself was never inserted as a complete word.
    #[test]
    fn test_starts_with_existing_prefix_returns_true() {
        let mut trie = Trie::new();
        trie.insert("hello");

        assert!(trie.starts_with("hel"));
    }

    /// `starts_with` must return false when no inserted word shares the given
    /// prefix. A faulty impl might conflate prefix-search with word-search.
    #[test]
    fn test_starts_with_absent_prefix_returns_false() {
        let mut trie = Trie::new();
        trie.insert("hello");

        assert!(!trie.starts_with("world"));
    }

    /// Multiple distinct words must all be retrievable after being inserted.
    /// Ensures that inserting one word does not overwrite or corrupt another.
    #[test]
    fn test_multiple_words_all_found() {
        let mut trie = Trie::new();
        let words = ["apple", "app", "application", "banana"];
        for w in &words {
            trie.insert(w);
        }

        for w in &words {
            assert!(trie.search(w), "expected to find '{w}'");
        }
        assert!(!trie.search("ap"), "bare prefix 'ap' should not be a word");
    }
}
